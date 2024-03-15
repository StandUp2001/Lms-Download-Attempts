use std::{process::Command, thread::sleep, time::Duration};

use ureq::{get, serde_json::Value, Error};

fn main() -> Result<(), Error> {
    println!("Getting the node-paths");
    let json: Value = get("https://sd42.nl/api/node-paths").call()?.into_json()?;
    for (key, _) in json.as_object().unwrap() {
        println!("Running: {}", key);
        Command::new("lms")
            .arg("download")
            .arg(key)
            .spawn()
            .expect("Not found attempt");

        sleep(Duration::from_millis(500))
    }
    println!("DONE");
    Ok(())
}
