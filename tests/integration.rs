extern crate rust_qsharp_client;

#[tokio::test]
async fn compile_qsharp_file(){
    // TODO: Store a qs file and load and compile it here!
    let client = rust_qsharp_client::QSharpClient::new();
    client.compile_file().await;
}