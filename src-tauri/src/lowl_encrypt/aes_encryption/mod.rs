use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use anyhow::Ok;

pub fn encrypt_with_password(password: &str, data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let salt: [u8; 32] = [0; 32];
    let cipher = create_cipher(password, salt)?;
    let nonce: &GenericArray<u8, _> = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let encrypted_data: Vec<u8> = cipher
        .encrypt(nonce, data)
        .map_err(|_err| anyhow::anyhow!("Unable to encrypt the data"))?;
    return Ok(encrypted_data);
}

pub fn decrypt_with_password(password: &str, data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let salt: [u8; 32] = [0; 32];
    let cipher = create_cipher(password, salt)?;
    let nonce = Nonce::from_slice(b"unique nonce");
    let decrypt_data = cipher
        .decrypt(nonce, data)
        .map_err(|_err| anyhow::anyhow!("Unable to decrypt the data"))?;
    return Ok(decrypt_data);
}

fn create_cipher(password: &str, salt: [u8; 32]) -> Result<Aes256GcmSiv, anyhow::Error> {
    let config: argon2::Config<'_> = argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    };
    let pass_key: Vec<u8> = argon2::hash_raw(password.as_bytes(), salt.as_ref(), &config)?;
    return Ok(Aes256GcmSiv::new(GenericArray::from_slice(&pass_key)));
}
