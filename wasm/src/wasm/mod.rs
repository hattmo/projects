mod instructions;
use instructions::{basic, control_flow, integer_arithmetic};

#[derive(Debug, Copy, Clone)]
pub enum DataTypes {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Label,
}

#[derive(Default)]
pub struct Store {
    funcs: Vec<WasmFunc>,
    tables: Vec<u8>,
    mems: Vec<u8>,
    globals: Vec<u8>,
    elems: Vec<u8>,
    datas: Vec<u8>,
}

#[derive(Debug)]
struct State {
    stack: Vec<DataTypes>,
    local: Vec<DataTypes>,
    ip: usize,
}

impl State {
    fn new(local_size: usize, params: &[DataTypes]) -> State {
        let mut local = vec![DataTypes::I32(0); local_size];
        for (i, elem) in params.iter().enumerate() {
            local[i] = *elem;
        }
        State {
            ip: 0,
            stack: Vec::new(),
            local,
        }
    }
}

trait Executable {
    fn exec(&self, state: &mut State, store: &Store) -> Result<(), &str>;
}

pub struct WasmFunc {
    instructions: Vec<Box<dyn Executable>>,
    local_size: usize,
}

impl WasmFunc {
    pub fn new(local_size: usize) -> WasmFunc {
        WasmFunc {
            instructions: Vec::new(),
            local_size,
        }
    }

    pub fn i32add(&mut self) -> &mut WasmFunc {
        self.instructions
            .push(Box::new(integer_arithmetic::I32Add {}));
        self
    }

    pub fn i64add(&mut self) -> &mut WasmFunc {
        self.instructions
            .push(Box::new(integer_arithmetic::I64Add {}));
        self
    }

    pub fn i32const(&mut self, val: i32) -> &mut WasmFunc {
        self.instructions.push(Box::new(basic::I32Const { val }));
        self
    }

    pub fn i64const(&mut self, val: i64) -> &mut WasmFunc {
        self.instructions.push(Box::new(basic::I64Const { val }));
        self
    }

    pub fn local_set(&mut self, id: usize) -> &mut WasmFunc {
        self.instructions.push(Box::new(basic::LocalSet { id }));
        self
    }

    pub fn local_get(&mut self, id: usize) -> &mut WasmFunc {
        self.instructions.push(Box::new(basic::LocalGet { id }));
        self
    }

    pub fn cond_jump(&mut self, dst: usize) {
        self.instructions
            .push(Box::new(control_flow::CondJump { dst }))
    }

    pub fn exec(&self, params: &[DataTypes], store: &Store) -> Result<(), &str> {
        let mut state = State::new(self.local_size, params);
        let len = self.instructions.len();
        while state.ip < len {
            self.instructions[state.ip].exec(&mut state, store)?;
        }
        print!("{:?}", state);
        Ok(())
    }
}
