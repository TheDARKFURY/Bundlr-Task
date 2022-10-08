
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
    let size:Vec<&str> = v[0].split(":").collect();
    let size2: u64 = (&size[1][1..size[1].len() - 1]).trim().parse().unwrap();
    // println!("{:?}", v);
    print!("{}", size2);
    let chunck_data = &chunck[1][1..chunck[1].len() - 1];
    let chunk_int:u64 = chunck_data.trim().parse().unwrap();
    // println!("chunk_data:{}", chunck_data);
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

    let mut chunk_data = String::new();
    let s1 = response2.strip_prefix("{").unwrap();
    let s2 = response2.strip_suffix("}").unwrap();
    let concatenate_chunk: Vec<&str> = response2.split(",").collect();
    let a = concatenate_chunk[3];
    let b: Vec<&str> = a.split(":").collect();
    let temp1= b[1].strip_prefix('"').unwrap();
    // println!("{}", temp1);
    let temp2 = temp1.strip_suffix("}").unwrap();
    // print!("{}", temp2);
    let temp2 = temp2.strip_suffix('"').unwrap();
    chunk_data.push_str(temp2);
    // println!("{:?}", chunk_data);
    // let mut file2 = File::create("./Result.txt")?;
    // file2.write_all(chunk_data.as_bytes())
    // .expect("Something got wrong");

    // println!("{}", chunk_data);
    // let chunk_int:u64 = chunk_data.trim().parse().unwrap();
    let chunk_data2 = chunk_int - size2 + 1;
    let chunk_data3 = chunk_data2.to_string();
    let mut url3: String = "https://arweave.net/chunk/".to_string();
    url3.push_str(chunk_data3.as_str());
    let response3 = client
        .get(url3)
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

        let mut chunk_data2 = String::new();
        let s1 = response3.strip_prefix("{").unwrap();
        let s2 = response3.strip_suffix("}").unwrap();
        let concatenate_chunk: Vec<&str> = response3.split(",").collect();
        let a = concatenate_chunk[3];
        let b: Vec<&str> = a.split(":").collect();
        let temp1= b[1].strip_prefix('"').unwrap();
        // println!("{}", temp1);
        let temp2 = temp1.strip_suffix("}").unwrap();
        // print!("{}", temp2);
        let temp2 = temp2.strip_suffix('"').unwrap();
        chunk_data2.push_str(temp2);
        // println!("{:?}", chunk_data);
        let mut file2 = File::create("./Result.txt")?;
        file2.write_all(chunk_data2.as_bytes())
        .expect("Something got wrong");
    
    Ok(())
}