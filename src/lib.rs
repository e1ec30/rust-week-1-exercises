// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let as_bytes = match hex::decode(raw_tx_hex) {
        Ok(v) => v,
        Err(_) => return Err("Hex decode error".into()),
    };

    if as_bytes.len() < 8 {
        return Err("Transaction data too short".into());
    }

    // for (i, num) in as_bytes[..4].iter().enumerate() {
    //     let x = *num as u32 * ((10u32).pow(i as u32));
    //     ver += x;
    // }

    //Now that I have working code, I want to get more familiar with rust iterators
    let ver = as_bytes[..4]
        .iter()
        .enumerate()
        .fold(0, |x, (i, num)| x + (*num as u32 * ((10u32).pow(i as u32))));

    Ok(ver)
}
