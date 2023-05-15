use ethers::prelude::Abigen;

fn main() {
    // SyncSwap Router
    let abi_file =
        std::fs::read_to_string("./abi/SyncSwapRouter.json").expect("Failed to read ABI file");
    let abigen = Abigen::new("SyncSwapRouter", abi_file).expect("Failed to parse ABI");

    match abigen.generate() {
        Ok(b) => {
            let _ = b.write_to_file("./src/bindings/sync_swap_router.rs");
        }
        Err(e) => {
            println!("Failed to generate bindings: {}", e)
        }
    }

    // 1inch Aggregation Router V5
    // let abi_file =
    //     std::fs::read_to_string("./abi/AggregationRouterV5.json").expect("Failed to read ABI file");
    // let abigen = Abigen::new("AggregationRouterV5", abi_file).expect("Failed to parse ABI");

    // match abigen.generate() {
    //     Ok(b) => {
    //         let _ = b.write_to_file("./src/bindings/aggregation_router_v5.rs");
    //     }
    //     Err(e) => {
    //         println!("Failed to generate bindings: {}", e)
    //     }
    // }

    // ERC20
    let abi_file = std::fs::read_to_string("./abi/ERC20.json").expect("Failed to read ABI file");
    let abigen = Abigen::new("ERC20", abi_file).expect("Failed to parse ABI");

    match abigen.generate() {
        Ok(b) => {
            let _ = b.write_to_file("./src/bindings/erc20.rs");
        }
        Err(e) => {
            println!("Failed to generate bindings: {}", e)
        }
    }

    // Permit
    let abi_file = std::fs::read_to_string("./abi/Permit.json").expect("Failed to read ABI file");
    let abigen = Abigen::new("Permit", abi_file).expect("Failed to parse ABI");

    match abigen.generate() {
        Ok(b) => {
            let _ = b.write_to_file("./src/bindings/permit.rs");
        }
        Err(e) => {
            println!("Failed to generate bindings: {}", e)
        }
    }
}
