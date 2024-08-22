use quantum_resistant_crypto::key_management::QuantumKeyPair;
use quantum_resistant_crypto::encryption::{encrypt_data, decrypt_data};
use quantum_resistant_crypto::signature::{sign_transaction, verify_signature};

fn main() {
    // Step 1: Generate Quantum-Resistant Key Pair
    let keypair = QuantumKeyPair::generate();
    println!("Generated Quantum-Resistant Public Key: {:?}", keypair.public_key);
    println!("Generated Quantum-Resistant Private Key: {:?}", keypair.private_key);

    // Step 2: Encrypt Data
    let data = b"Sensitive transaction data";
    let encrypted_data = encrypt_data(&keypair.public_key, data);
    println!("Encrypted Data: {:?}", encrypted_data);

    // Step 3: Decrypt Data
    let decrypted_data = decrypt_data(&keypair.private_key, &encrypted_data);
    println!("Decrypted Data: {:?}", String::from_utf8(decrypted_data).unwrap());

    // Step 4: Sign Data
    let signature = sign_transaction(&keypair.private_key, data);
    println!("Signature: {:?}", signature);

    // Step 5: Verify Signature
    let is_valid = verify_signature(&keypair.public_key, data, &signature);
    println!("Is the signature valid? {}", is_valid);
}
