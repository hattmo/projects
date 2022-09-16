mod instructions;
use instructions::{basic, control_flow, integer_arithmetic};

#[derive(Debug, Copy, Clone)]
pub enum DataTypes {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
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
        let mut i = 0;
        for elem in params {
            local[i] = *elem;
            i = i + 1;
        }
        State {
            ip: 0,
            stack: Vec::new(),
            local: local,
        }
    }
}

trait Executable {
    fn exec(&self, state: &mut State) -> Result<(), &str>;
}

pub struct WasmFunc {
    instructions: Vec<Box<dyn Executable>>,
    local_size: usize,
}

impl WasmFunc {
    pub fn new(local_size: usize) -> WasmFunc {
        WasmFunc {
            instructions: Vec::new(),
            local_size: local_size,
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
        self.instructions
            .push(Box::new(basic::I32Const { val: val }));
        self
    }

    pub fn i64const(&mut self, val: i64) -> &mut WasmFunc {
        self.instructions
            .push(Box::new(basic::I64Const { val: val }));
        self
    }

    pub fn local_set(&mut self, id: usize) -> &mut WasmFunc {
        self.instructions.push(Box::new(basic::LocalSet { id: id }));
        self
    }

    pub fn local_get(&mut self, id: usize) -> &mut WasmFunc {
        self.instructions.push(Box::new(basic::LocalGet { id: id }));
        self
    }

    pub fn cond_jump(&mut self, dst: usize) {
        self.instructions
            .push(Box::new(control_flow::CondJump { dst: dst }))
    }

    pub fn exec(&mut self, params: &[DataTypes]) -> Result<(), &str> {
        let mut state = State::new(self.local_size, params);
        let len = self.instructions.len();
        while state.ip < len {
            self.instructions[state.ip].exec(&mut state)?;
        }
        print!("{:?}", state);
        Ok(())
    }
}
