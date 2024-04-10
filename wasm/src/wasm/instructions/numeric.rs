use crate::wasm::{DataTypes, Executable, State, Store};

struct Const {
    val: DataTypes,
}

impl Executable for Const {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &str> {
        state.stack.push(self.val);
        Ok(())
    }
}
