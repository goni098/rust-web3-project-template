/// Parses the chain ID from command line arguments (expects 3rd argument)
///
/// # Panics
/// Panics if the chain_id argument is missing or not a valid u64
pub fn parse_chain_arg() -> u64 {
    let args: Vec<String> = std::env::args().collect();

    args.get(2)
        .unwrap_or_else(|| {
            panic!(
                "❌ Missing chain_id argument. Usage: {} <command> <chain_id>",
                args.first().map(|s| s.as_str()).unwrap_or("program")
            )
        })
        .parse()
        .unwrap_or_else(|e| {
            panic!(
                "❌ Invalid chain_id: expected u64, got '{}'. Error: {}",
                args[2], e
            )
        })
}
