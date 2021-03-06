use crate::frame::{tcp::*, *};
use crate::proto;

use futures::prelude::*;
use std::io::Error;
use std::net::SocketAddr;
use tokio_proto::TcpServer;
use tokio_service::{NewService, Service};

/// A multithreaded Modbus server.
pub struct Server {
    server_type: ServerType,
}

enum ServerType {
    Tcp(SocketAddr),
}

struct ServiceWrapper<S> {
    service: S,
}

impl<S> ServiceWrapper<S> {
    fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S> Service for ServiceWrapper<S>
where
    S: Service + Send + Sync + 'static,
    S::Request: From<Request>,
    S::Response: Into<Response>,
    S::Error: Into<Error>,
{
    type Request = RequestAdu;
    type Response = ResponseAdu;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, adu: Self::Request) -> Self::Future {
        let Self::Request { hdr, pdu } = adu;
        let req: Request = pdu.into();
        Box::new(self.service.call(req.into()).then(move |res| match res {
            Ok(res) => {
                let res: Response = res.into();
                let pdu = res.into();
                Ok(Self::Response { hdr, pdu })
            }
            Err(e) => Err(e.into()),
        }))
    }
}

impl Server {
    /// Create a new Modbus TCP server instance.
    #[cfg(feature = "tcp")]
    pub fn new_tcp(addr: SocketAddr) -> Server {
        Server {
            server_type: ServerType::Tcp(addr),
        }
    }

    #[cfg(feature = "tcp")]
    pub fn serve<S>(&self, service: S)
    where
        S: NewService + Send + Sync + 'static,
        S::Request: From<Request>,
        S::Response: Into<Response>,
        S::Error: Into<Error>,
        S::Instance: Send + Sync + 'static,
    {
        match self.server_type {
            ServerType::Tcp(addr) => {
                TcpServer::new(proto::tcp::Proto, addr)
                    .serve(move || Ok(ServiceWrapper::new(service.new_service()?)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future;

    #[test]
    fn service_wrapper() {
        #[derive(Clone)]
        struct DummyService {
            response: Response,
        };

        impl Service for DummyService {
            type Request = Request;
            type Response = Response;
            type Error = Error;
            type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

            fn call(&self, _: Self::Request) -> Self::Future {
                Box::new(future::ok(self.response.clone()))
            }
        }

        let s = DummyService {
            response: Response::ReadInputRegisters(vec![0x33]),
        };
        let service = ServiceWrapper::new(s.clone());

        let hdr = Header {
            transaction_id: 9,
            unit_id: 7,
        };
        let pdu = Request::ReadInputRegisters(0, 1).into();
        let req_adu = RequestAdu { hdr, pdu };
        let res_adu = service.call(req_adu).wait().unwrap();

        assert_eq!(
            res_adu.hdr,
            Header {
                transaction_id: 9,
                unit_id: 7,
            }
        );
        assert_eq!(res_adu.pdu, s.response.into());
    }
}
