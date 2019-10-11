use cabide::Cabide;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{Ordering};
use cabide::READ_BLOCKS_COUNT;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Data {
    uhe: u64,
    cenario: u64,
    estagio: String,
    geracao: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cbd: Cabide<Data> = Cabide::new("gh_head.file", None)?;

    // Edit this array with all desired ids
    let ids: [u64; 2] = [0, 3];

    for id in ids.iter() {
        let result = &cbd.read(*id);
        match result {
            Ok(_v) => println!("Found {} from {}", _v.uhe, _v.estagio),
            Err(_e) => println!("Found nothing"),
        }
    }

    println!();
    println!("Used blocks: {}", cbd.blocks()?);
    println!("Read blocks: {}", READ_BLOCKS_COUNT.load(Ordering::Relaxed));

    Ok(())
}
