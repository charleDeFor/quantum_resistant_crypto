pub fn sign_transaction(private_key: &[u8], data: &[u8]) -> Vec<u8> {
    // Example signing logic (using XOR as a simple operation)
    data.iter().zip(private_key.iter().cycle())
        .map(|(&byte, &key)| byte ^ key)
        .collect()
}

pub fn verify_signature(public_key: &[u8], data: &[u8], signature: &[u8]) -> bool {
    // Verification should mirror the signing logic
    let expected_signature: Vec<u8> = data.iter().zip(public_key.iter().cycle())
        .map(|(&byte, &key)| byte ^ key)
        .collect();

    expected_signature == signature
}
