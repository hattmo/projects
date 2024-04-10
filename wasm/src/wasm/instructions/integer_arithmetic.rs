use crate::wasm::DataTypes;
use crate::wasm::Executable;
use crate::wasm::State;
use crate::wasm::Store;

pub struct I32Add {}

impl Executable for I32Add {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        let aval = state.stack.pop().ok_or("No items on stack")?;
        let bval = state.stack.pop().ok_or("No items on stack")?;
        if let (DataTypes::I32(a), DataTypes::I32(b)) = (aval, bval) {
            state.stack.push(DataTypes::I32(a + b));
            state.ip += 1;
            return Ok(());
        }
        Ok(())
    }
}

pub struct I64Add {}

impl Executable for I64Add {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        let aval = state.stack.pop().ok_or("No items on stack")?;
        let bval = state.stack.pop().ok_or("No items on stack")?;
        if let (DataTypes::I64(a), DataTypes::I64(b)) = (aval, bval) {
            state.stack.push(DataTypes::I64(a + b));
            state.ip += 1;
            return Ok(());
        }
        Err("Invalid data types on stack")
    }
}

pub struct I32Sub {}

impl Executable for I32Sub {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        let aval = state.stack.pop().ok_or("No items on stack")?;
        let bval = state.stack.pop().ok_or("No items on stack")?;
        if let (DataTypes::I32(a), DataTypes::I32(b)) = (aval, bval) {
            state.stack.push(DataTypes::I32(a - b));
            state.ip += 1;
            return Ok(());
        }
        Err("Invalid data types on stack")
    }
}

pub struct I64Sub {}

impl Executable for I64Sub {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        let aval = state.stack.pop().ok_or("No items on stack")?;
        let bval = state.stack.pop().ok_or("No items on stack")?;
        if let (DataTypes::I64(a), DataTypes::I64(b)) = (aval, bval) {
            state.stack.push(DataTypes::I64(a - b));
            state.ip += 1;
            return Ok(());
        }
        Err("Invalid data types on stack")
    }
}

pub struct I32Mul {}

impl Executable for I32Mul {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        let aval = state.stack.pop().ok_or("No items on stack")?;
        let bval = state.stack.pop().ok_or("No items on stack")?;
        if let (DataTypes::I32(a), DataTypes::I32(b)) = (aval, bval) {
            state.stack.push(DataTypes::I32(a * b));
            state.ip += 1;
            return Ok(());
        }
        Err("Invalid data types on stack")
    }
}

pub struct I64Mul {}

impl Executable for I64Mul {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        let aval = state.stack.pop().ok_or("No items on stack")?;
        let bval = state.stack.pop().ok_or("No items on stack")?;
        if let (DataTypes::I64(a), DataTypes::I64(b)) = (aval, bval) {
            state.stack.push(DataTypes::I64(a * b));
            state.ip += 1;
            return Ok(());
        }
        Err("Invalid data types on stack")
    }
}

pub struct I32DivS {}

impl Executable for I32DivS {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        let aval = state.stack.pop().ok_or("No items on stack")?;
        let bval = state.stack.pop().ok_or("No items on stack")?;
        if let (DataTypes::I32(a), DataTypes::I32(b)) = (aval, bval) {
            state.stack.push(DataTypes::I32(a / b));
            state.ip += 1;
            return Ok(());
        }
        Err("Invalid data types on stack")
    }
}

pub struct I64DivS {}

impl Executable for I64DivS {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        let aval = state.stack.pop().ok_or("No items on stack")?;
        let bval = state.stack.pop().ok_or("No items on stack")?;
        if let (DataTypes::I64(a), DataTypes::I64(b)) = (aval, bval) {
            state.stack.push(DataTypes::I64(a / b));
            state.ip += 1;
            return Ok(());
        }
        Err("Invalid data types on stack")
    }
}
