use futures::future::FutureResult;
use futures::future::{self, Future};
use std::thread;
use tokio_core::reactor::Core;
use tokio_modbus::*;
use tokio_service::Service;

struct MbServer;

impl Service for MbServer {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            Request::ReadInputRegisters(_addr, cnt) => {
                let mut registers = vec![0; cnt as usize];
                registers[2] = 0x77;
                let res = Response::ReadInputRegisters(registers);
                future::ok(res)
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(feature = "tcp")]
fn main() {
    let _server = thread::spawn(|| {
        let socket_addr = "127.0.0.1:5502".parse().unwrap();
        let server = Server::new_tcp(socket_addr);
        server.serve(|| Ok(MbServer));
    });

    let client = thread::spawn(|| {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let socket_addr = "127.0.0.1:5502".parse().unwrap();

        let task = Client::connect_tcp(&socket_addr, &handle).and_then(|client| {
            client.read_input_registers(0x0, 7).and_then(move |res| {
                println!("The result is '{:?}'", res);
                Ok(())
            })
        });

        core.run(task).unwrap();
    });

    client.join().unwrap();
}

#[cfg(not(feature = "tcp"))]
pub fn main() {
    println!("feature `tcp` is required to run this example");
    ::std::process::exit(1);
}
