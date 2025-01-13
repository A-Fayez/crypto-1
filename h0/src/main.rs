use itertools::Itertools;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let data: Vec<u8> = fs::read("/Users/fayez/personal/crypto/h0/test.mp4_download")?;
    let number_of_blocks = (data.len() / 1024) + 1;
    let blocks = data.iter().rev().copied().chunks(1024);
    let mut hashed_blocks: Vec<Vec<u8>> = Vec::new();
    let _bytes_blocks: Vec<Vec<u8>> = Vec::new();

    for (i, block) in blocks.into_iter().enumerate() {
        let mut b = block.collect::<Vec<u8>>();
        if i == 0 {
            hashed_blocks.push(b);
            continue;
        }
        let h0 = sha256::digest(hashed_blocks[i - 1].clone());
        let h = sha256::digest(hashed_blocks[i - 1].clone()).into_bytes();
        b.extend(h);
        hashed_blocks.push(b);
        if i == number_of_blocks - 1 {
            println!("h0 is: {}", h0);
        }
    }

    let h0 = hashed_blocks.last().unwrap().to_owned();
    let h0 = sha256::digest(h0);
    println!("h0 -> {}", h0);

    Ok(())
}
