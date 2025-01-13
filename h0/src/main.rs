use hex;
use itertools::Itertools;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let data: Vec<u8> = fs::read("/Users/fayez/personal/crypto/h0/p.mp4_download")?;
    let last_byte = data.clone().last().unwrap().to_owned();
    let blocks: Vec<Vec<u8>> = data
        .into_iter()
        .chunks(1024)
        .into_iter()
        .map(|b| b.collect::<Vec<u8>>())
        .collect();

    let mut hashed_blocks: Vec<Vec<u8>> = Vec::new();

    for (i, mut b) in blocks.into_iter().rev().enumerate() {
        if i == 0 {
            hashed_blocks.push(b);
            continue;
        }
        let h0 = sha256::digest(hashed_blocks[i - 1].clone());
        let h = sha256::digest(hashed_blocks[i - 1].clone()).into_bytes();
        b.extend(hex::decode(h).unwrap());
        hashed_blocks.push(b);
    }

    let h0 = hashed_blocks.last().unwrap().to_owned();
    let h0 = sha256::digest(h0);
    println!("h0 -> {:?}", h0);

    Ok(())
}
