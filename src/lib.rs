mod qsharp_http_client;
use qsharp_http_client::QSharpHttpClient;

pub struct QSharpClient<'a> {
    http_client: &'a QSharpHttpClient<'a>
}

impl QSharpClient<'_> {
 
    pub fn new<'a> () -> QSharpClient<'a> {
        QSharpClient {
            http_client: &QSharpHttpClient { uri: "http://localhost:8888" },
        }
    }

    pub async fn compile_file(&self){
        // TODO: Get text from a file, send to API.

        // Return a struct that represents the whole response.
        unimplemented!()
    }
}