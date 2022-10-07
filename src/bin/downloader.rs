
use reqwest;

use std::collections::HashMap;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    let mut args_map: HashMap<&str, &str> = HashMap::new();
    for i in (1..args.len()).step_by(2usize) {
        let key = &args[i][2..];
        args_map.insert(key, &args[i + 1]);
    }

    // println!("Args:{:?}", args_map);

    // println!("transaction:{}", args_map["transaction"]);
    // println!("outfile:{}", args_map["output"]);
    let client = reqwest::Client::new();
    let mut url: String = "https://arweave.net/tx/".to_string();
    url.push_str(args_map["transaction"]);
    url.push_str("/offset");
    let response = client
        .get(url)
        //.get("https://arweave.net/chunk/103503565637739")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

    let s1 = response.strip_prefix("{").unwrap();
    let s2 = response.strip_suffix("}").unwrap();
    let v: Vec<&str> = s2.split(",").collect();
    let chunck: Vec<&str> = v[1].split(":").collect();
    let chunck_data = &chunck[1][1..chunck[1].len() - 1];

    println!("chunk_data:{}", chunck_data);
    let mut url2: String = "https://arweave.net/chunk/".to_string();
    url2.push_str(chunck_data);

    let response2 = client
        .get(url2)
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    let mut file = File::create(args_map["output"])?;
    file.write_all(response2.as_bytes())
        .expect("Something got wrong");
    // file.write_all(response2.as_bytes())
    //     .expect("Something got wrong");
    // println!("{:}", test);
    Ok(())
}