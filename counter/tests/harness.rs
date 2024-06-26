use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "Counter",
    abi = "out/debug/counter-abi.json"
));

async fn get_contract_instance() -> (Counter<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from("./out/debug/counter.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxPolicies::default())
        .await
        .unwrap();

    let instance = Counter::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn can_get_contract_id() {
    let (_instance, _id) = get_contract_instance().await;
}

#[tokio::test]
async fn test_inc_and_dec() {
    let (instance, _id) = get_contract_instance().await;

    let mut result = instance.methods().inc().call().await.unwrap();
    println!("{:#?}", result);
    result = instance.methods().get().call().await.unwrap();
    assert_eq!(result.value, 1);

    instance.methods().dec().call().await.unwrap();
    result = instance.methods().get().call().await.unwrap();
    assert_eq!(result.value, 0);
}

#[tokio::test]
#[should_panic]
async fn test_dec_underflow() {
    let (instance, _id) = get_contract_instance().await;
    instance.methods().dec().call().await.unwrap();
}
