#[ic_cdk::query]
fn greet(name: String) -> String {
    // let beneficiary_txid = 
    //     Txid::from_hex("14295d5bb1a191cdb6286dc0944df938421e3dfcbf0811353ccac4100c2068c5").unwrap();
    // let beneficiary = Outpoint::new(beneficiary_txid, 1);

    // let contract = NonInflatableAsset::testnet("TEST", "Test asset", None, Precision::CentiMicro)
    //     .expect("invalid contract data")
    //     .allocate(Method::TapretFirst, beneficiary, 1_000_000_000_00u64.into())
    //     .expect("invalid allocations")
    //     .issue_contract()
    //     .expect("invalid contract data");

    // let contract_id = contract.contract_id();

    format!("Hello, {}!", name)
}
