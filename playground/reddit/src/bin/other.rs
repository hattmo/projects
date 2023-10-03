fn main() {
    let my_chars = vec![1u8,2,3,4,];
    String::from_utf8_lossy(&my_chars).to_string();
}