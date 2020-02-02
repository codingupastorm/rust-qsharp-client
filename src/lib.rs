mod qsharp_http_client;
use qsharp_http_client::QSharpHttpClient;

pub struct QSharpClient<'a> {
    http_client: &'a QSharpHttpClient<'a>
}

impl QSharpClient<'_> {
 
    pub async fn compile_file(){
        // TODO: Get text from a file, send to API.

        // Return a struct that represents the whole response.
        unimplemented!()
    }
}