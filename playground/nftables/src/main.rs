pub mod bindings;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::ffi::CString;

    use super::bindings::*;
    #[test]
    fn test() {
        unsafe {
            let ctx = nft_ctx_new(0);
            let command = CString::new("list tables").unwrap();
            nft_run_cmd_from_buffer(ctx, command.as_ptr());
            nft_ctx_free(ctx)
        }
    }
}
