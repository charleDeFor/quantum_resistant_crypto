pub fn sign_transaction(private_key: &[u8], data: &[u8]) -> Vec<u8> {
    // Example signing logic (dummy example)
    data.iter().map(|&byte| byte ^ private_key[0]).collect()
}

pub fn verify_signature(public_key: &[u8], data: &[u8], signature: &[u8]) -> bool {
    // Example signature verification logic (dummy example)
    data.iter().map(|&byte| byte ^ public_key[0]).collect::<Vec<u8>>() == signature
}
