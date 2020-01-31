use std::collections::HashMap;
use reqwest::Response;

pub struct QSharpHttpClient<'a> {
    uri: &'a str
}

impl QSharpHttpClient<'_> {

    pub async fn compile_snippet(&self, snippet: &str) -> Result<Response, Box<dyn std::error::Error>> {

        let mut map = HashMap::new();
        map.insert("code", snippet);

        let post_url = format!("{}/Snippets/compile", self.uri);

        let client = reqwest::Client::new();
        let resp = client.post(&post_url)
            .json(&map)
            .send()
            .await?;

        Ok(resp)
    }

}

#[tokio::test]
async fn can_compile_code(){

    // This relies on the HTTP IQ# server from https://github.com/microsoft/iqsharp to be running.

    const CODE: &str = "operation HelloQ() : Unit
    {
        Message($\"Hello from quantum world!\"); 
    }";

    const URI: &str = "http://localhost:8888";

    let client = QSharpHttpClient{uri: URI};
    let result = client.compile_snippet(CODE).await;
    assert!(result.is_ok());
}
