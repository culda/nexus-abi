use ethers::prelude::Abigen;

fn main() {
    let abi_file =
        std::fs::read_to_string("./abi/SyncSwapRouter.json").expect("Failed to read ABI file");
    let abigen = Abigen::new("SyncSwapRouter", abi_file).expect("Failed to parse ABI");

    match abigen.generate() {
        Ok(b) => {
            let _ = b.write_to_file("./bindings/sync_swap_router.rs");
        }
        Err(e) => {
            println!("Failed to generate bindings: {}", e)
        }
    }
}
