pub fn parse_chain_arg() -> u64 {
    let args: Vec<String> = std::env::args().collect();

    args.get(2)
        .unwrap_or_else(|| panic!("Missing chain_id arg, received: {args:?}"))
        .parse()
        .unwrap_or_else(|_| panic!("Expected u64 chain_id"))
}
