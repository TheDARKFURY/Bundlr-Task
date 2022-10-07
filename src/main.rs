// use reqwest;
// use std::error::Error;
// use std::time::Duration;
// use std::fs::File;
// use std::io::prelude::*;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let client = reqwest::Client::new();
//     let test = client
//         .get("https://arweave.net/tx/J1OYZmmwHjexnvwsjVjmgxpAp7PU8OAYG-N2IECjTJ0/offset")
//         // .get("https://arweave.net/chunk/103503565637739")
//         .header("Accept", "text/plain")
//         .timeout(Duration::from_secs(3))
//         .send()
//         .await?
//         .text()
//         .await?;
//         let mut file = File::create("test.json")?;
//         file.write_all(test.as_bytes()).expect("Something got wrong");
//     // println!("{:}", test);
//     Ok(())
// }

fn main(){
    println!("Bundlr Network");
}
