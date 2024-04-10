use crate::wasm::{Executable, State, Store};

struct Nop;

impl Executable for Nop {
    fn exec(&self, state: &mut State, _store: &Store) -> Result<(), &'static str> {
        state.ip += 1;
        Ok(())
    }
}

struct Unreachable;

impl Executable for Unreachable {
    fn exec(&self, _state: &mut State, _store: &Store) -> Result<(), &'static str> {
        Err("Unreachable")
    }
}
