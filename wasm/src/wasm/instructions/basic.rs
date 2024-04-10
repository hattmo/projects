use crate::wasm::DataTypes;
use crate::wasm::Executable;
use crate::wasm::State;
use crate::wasm::Store;

pub struct NoOp {}

impl Executable for NoOp {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        state.ip += 1;
        Ok(())
    }
}

pub struct Drop {}

impl Executable for Drop {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        state.stack.pop();
        state.ip += 1;
        Ok(())
    }
}

pub struct I32Const {
    pub val: i32,
}

impl Executable for I32Const {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        state.stack.push(DataTypes::I32(self.val));
        state.ip += 1;
        Ok(())
    }
}
pub struct I64Const {
    pub val: i64,
}

impl Executable for I64Const {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        state.stack.push(DataTypes::I64(self.val));
        state.ip += 1;
        Ok(())
    }
}

pub struct F32Const {
    pub val: f32,
}

impl Executable for F32Const {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        state.stack.push(DataTypes::F32(self.val));
        state.ip += 1;
        Ok(())
    }
}
pub struct F64Const {
    pub val: f64,
}

impl Executable for F64Const {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        state.stack.push(DataTypes::F64(self.val));
        state.ip += 1;
        Ok(())
    }
}

pub struct LocalGet {
    pub id: usize,
}

impl Executable for LocalGet {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        state.stack.push(state.local[self.id]);
        state.ip += 1;
        Ok(())
    }
}

pub struct LocalSet {
    pub id: usize,
}

impl Executable for LocalSet {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        let v = state.stack.pop().ok_or("No items on stack")?;
        state.local[self.id] = v;
        state.ip += 1;
        Ok(())
    }
}
