#[cfg(feature = "rtu")]
pub mod rtu;

#[cfg(feature = "tcp")]
pub mod tcp;

use super::{
    Client as AsyncClient, Connection as AsyncConnection, Reader as AsyncReader,
    Writer as AsyncWriter,
};

use crate::frame::*;

use std::io::Result;

use tokio_core::reactor::Core;

/// A transport independent synchronous client trait.
pub trait Client {
    fn call(&mut self, req: Request) -> Result<Response>;
}

/// A transport independent synchronous reader trait.
pub trait Reader {
    fn read_coils(&mut self, _: Address, _: Quantity) -> Result<Vec<Coil>>;
    fn read_discrete_inputs(&mut self, _: Address, _: Quantity) -> Result<Vec<Coil>>;
    fn read_input_registers(&mut self, _: Address, _: Quantity) -> Result<Vec<Word>>;
    fn read_holding_registers(&mut self, _: Address, _: Quantity) -> Result<Vec<Word>>;
    fn read_write_multiple_registers(
        &mut self,
        _: Address,
        _: Quantity,
        _: Address,
        _: &[Word],
    ) -> Result<Vec<Word>>;
}

/// A transport independent synchronous writer trait.
pub trait Writer {
    fn write_single_coil(&mut self, _: Address, _: Coil) -> Result<()>;
    fn write_multiple_coils(&mut self, _: Address, _: &[Coil]) -> Result<()>;
    fn write_single_register(&mut self, _: Address, _: Word) -> Result<()>;
    fn write_multiple_registers(&mut self, _: Address, _: &[Word]) -> Result<()>;
}

/// A synchronous Modbus client connection.
pub struct Connection {
    async_connection: AsyncConnection,
    core: Core,
}

impl Client for Connection {
    fn call(&mut self, req: Request) -> Result<Response> {
        self.core.run(self.async_connection.call(req))
    }
}

impl Reader for Connection {
    fn read_coils(&mut self, addr: Address, cnt: Quantity) -> Result<Vec<Coil>> {
        self.core.run(self.async_connection.read_coils(addr, cnt))
    }

    fn read_discrete_inputs(&mut self, addr: Address, cnt: Quantity) -> Result<Vec<Coil>> {
        self.core
            .run(self.async_connection.read_discrete_inputs(addr, cnt))
    }

    fn read_input_registers(&mut self, addr: Address, cnt: Quantity) -> Result<Vec<Word>> {
        self.core
            .run(self.async_connection.read_input_registers(addr, cnt))
    }

    fn read_holding_registers(&mut self, addr: Address, cnt: Quantity) -> Result<Vec<Word>> {
        self.core
            .run(self.async_connection.read_holding_registers(addr, cnt))
    }

    fn read_write_multiple_registers(
        &mut self,
        read_addr: Address,
        read_cnt: Quantity,
        write_addr: Address,
        write_data: &[Word],
    ) -> Result<Vec<Word>> {
        self.core.run(
            self.async_connection
                .read_write_multiple_registers(read_addr, read_cnt, write_addr, write_data),
        )
    }
}

impl Writer for Connection {
    fn write_single_register(&mut self, addr: Address, data: Word) -> Result<()> {
        self.core
            .run(self.async_connection.write_single_register(addr, data))
    }

    fn write_multiple_registers(&mut self, addr: Address, data: &[Word]) -> Result<()> {
        self.core
            .run(self.async_connection.write_multiple_registers(addr, data))
    }

    fn write_single_coil(&mut self, addr: Address, coil: Coil) -> Result<()> {
        self.core
            .run(self.async_connection.write_single_coil(addr, coil))
    }

    fn write_multiple_coils(&mut self, addr: Address, coils: &[Coil]) -> Result<()> {
        self.core
            .run(self.async_connection.write_multiple_coils(addr, coils))
    }
}