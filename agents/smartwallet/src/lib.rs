use amplify::hex::FromHex;
use bp::Outpoint;
use rgbstd::Txid;

#[ic_cdk::query]
fn greet(name: String) -> String {
    let beneficiary_txid =
        Txid::from_hex("14295d5bb1a191cdb6286dc0944df938421e3dfcbf0811353ccac4100c2068c5").unwrap();
    let beneficiary = Outpoint::new(beneficiary_txid, 1);

    // let contract = NonInflatableAsset::testnet("TEST", "Test asset", None, Precision::CentiMicro)
    //     .expect("invalid contract data")
    //     .allocate(Method::TapretFirst, beneficiary, 1_000_000_000_000u64.into())
    //     .expect("invalid allocations")
    //     .issue_contract()
    //     .expect("invalid contract data");

    // let contract_id = contract.contract_id();

    ic_cdk::api::print(format!("Hello from IC debugger, {}, {}", name, beneficiary));
    println!("Hello from WASI: {}, {}", name, beneficiary);

    format!("Hello, {}!, {}", name, beneficiary)
}

#[ic_cdk::init]
fn init() {
    unsafe {
        ic_wasi_polyfill::init(&[0u8; 32], &[]);
    };
}
