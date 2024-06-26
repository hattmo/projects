use crate::wasm::{self, Store};

#[test]
fn simple_test() {
    println!("Running the Test!!!!");
    let store = Store::default();
    wasm::WasmFunc::new(4)
        .i64const(20)
        .local_get(0)
        .i64add()
        .exec(&[wasm::DataTypes::I64(30)], &store)
        .expect("should not fail");
    assert_eq!(1, 1);
    assert_eq!(1, 2);
}
