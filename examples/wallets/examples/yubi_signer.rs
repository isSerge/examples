//! Example of signing and sending a transaction using a Yubi device.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::request::TransactionRequest,
    signers::wallet::{
        yubihsm::{Connector, Credentials, UsbConfig},
        YubiWallet,
    },
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // We use USB for the example, but you can connect over HTTP as well. Refer
    // to the [YubiHSM](https://docs.rs/yubihsm/0.34.0/yubihsm/) docs for more information.
    let connector = Connector::usb(&UsbConfig::default());

    // Instantiate the connection to the YubiKey. Alternatively, use the
    // `from_key` method to upload a key you already have, or the `new` method
    // to generate a new keypair.
    let signer = YubiWallet::connect(connector, Credentials::default(), 0);
    let from = signer.address();

    // Create a provider with the signer.
    let rpc_url = "http://localhost:8545".parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(signer))
        .on_http(rpc_url)?;

    // Build a transaction to send 100 wei to Vitalik.
    let tx = TransactionRequest::default()
        .with_from(from)
        .with_to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into())
        .with_value(U256::from(100));

    // Send the transaction and wait for the receipt.
    let receipt =
        provider.send_transaction(tx).await?.with_required_confirmations(3).get_receipt().await?;

    println!("Send transaction: {:?}", receipt.transaction_hash);

    Ok(())
}
