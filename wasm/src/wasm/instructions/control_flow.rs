use crate::wasm::DataTypes;
use crate::wasm::Executable;
use crate::wasm::State;

pub struct Jump {
    dst: usize,
}

impl Executable for Jump {
    fn exec(&self, state: &mut State) -> Result<(), &'static str> {
        state.ip = self.dst;
        Ok(())
    }
}

pub struct CondJump {
    pub dst: usize,
}

impl Executable for CondJump {
    fn exec(&self, state: &mut State) -> Result<(), &'static str> {
        let item = state.stack.pop().ok_or("No items on stack")?;
        if match item {
            DataTypes::I32(i) => i == 0,
            DataTypes::I64(i) => i == 0,
            _ => return Err("Wrong type on stack"),
        } {
            state.ip = state.ip + 1;
        } else {
            state.ip = self.dst;
        }
        Ok(())
    }
}
