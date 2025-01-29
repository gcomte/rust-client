use serde::{Deserialize, Serialize};
use tokio;
use wundergraph_rust_client::{Client, ClientOptions};

#[derive(Serialize)]
struct DragonsQueryInput;

#[derive(Deserialize, Debug)]
struct Dragon {
    name: String,
    active: bool,
}

#[derive(Deserialize, Debug)]
struct DragonsResponse {
    spacex_dragons: Vec<Dragon>,
}

#[tokio::main]
async fn main() {
    let options = ClientOptions::default();
    let client = Client::new(options);

    match client
        .query::<_, DragonsQueryInput, DragonsResponse>("Dragons", DragonsQueryInput {})
        .await
    {
        Ok(response) => {
            println!("Received response: {:?}", response);
            for dragon in response.spacex_dragons {
                println!("Dragon Name: {}, Active: {}", dragon.name, dragon.active);
            }
        }
        Err(err) => {
            eprintln!("Failed to query dragons: {:?}", err);
        }
    }
}
