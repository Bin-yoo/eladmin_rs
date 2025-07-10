use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use base64::prelude::*;
use anyhow::Result;

/// 生成一个新的 RSA 密钥对（1024位）
pub fn generate() -> Result<(RsaPublicKey, RsaPrivateKey)> {
    let mut rng = rand::thread_rng();
    let bits = 1024;

    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    Ok((public_key, private_key))
}

/// 使用公钥加密文本（支持长文本自动分块）
pub fn encrypt_by_public_key(public_key_text: &str, text: &str) -> Result<String> {
    let mut rng = rand::thread_rng();
    let public_key_bytes = BASE64_STANDARD.decode(public_key_text)?;
    let public_key = RsaPublicKey::from_public_key_der(&public_key_bytes)
        .map_err(|e| anyhow::anyhow!("Invalid public key encoding={}", e))?;

    let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, text.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed={}", e))?;

    Ok(BASE64_STANDARD.encode(enc_data))
}

/// 使用私钥解密文本（支持长文本自动分块）
pub fn decrypt_by_private_key(private_key_text: &str, cipher_text: &str) -> Result<String> {
    let cipher_bytes = BASE64_STANDARD.decode(cipher_text)?;
    let private_key_bytes = BASE64_STANDARD.decode(private_key_text)?;
    // 使用 PKCS#8 解析私钥
    let private_key = RsaPrivateKey::from_pkcs8_der(&private_key_bytes)
        .map_err(|e| anyhow::anyhow!("Invalid private key encoding={}", e))?;

    // 使用 PKCS1v1.5 进行解密
    let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &cipher_bytes)
        .map_err(|e| anyhow::anyhow!("Decryption failed={}", e))?;

    Ok(String::from_utf8(dec_data)?)
}


#[cfg(test)]
mod tests {
    use super::*;
    use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};

    const SRC: &str = "123456";

    #[test]
    fn test_rsa_key_pair_generation() {
        let (public_key, private_key) = generate().unwrap();
        // 将私钥和公钥转换为 PKCS#8 格式
        let pkcs8_public_key = public_key.to_public_key_der().unwrap();
        let pkcs8_private_key = private_key.to_pkcs8_der().unwrap();
        // 将 DER 字节转换为 Base64 编码的字符串
        let public_key_text = BASE64_STANDARD.encode(pkcs8_public_key.to_vec());
        let private_key_text = BASE64_STANDARD.encode(pkcs8_private_key.to_bytes());

        println!("Public Key: {}\n", public_key_text);
        println!("Private Key: {}", private_key_text);
    }

    #[test]
    fn test_private_encrypt_public_decrypt() {
        let (public_key, private_key) = generate().unwrap();
        // 将私钥和公钥转换为 PKCS#8 格式
        let pkcs8_public_key = public_key.to_public_key_der().unwrap();
        let pkcs8_private_key = private_key.to_pkcs8_der().unwrap();
        // 将 DER 字节转换为 Base64 编码的字符串
        let public_key_text = BASE64_STANDARD.encode(pkcs8_public_key.to_vec());
        let private_key_text = BASE64_STANDARD.encode(pkcs8_private_key.to_bytes());
        // 加密
        let cipher_text = encrypt_by_public_key(&public_key_text, SRC).unwrap();
        // 解密
        let plain_text = decrypt_by_private_key(&private_key_text, &cipher_text).unwrap();

        println!("Cipher Text: {}\n", cipher_text);
        println!("Plain Text: {}", plain_text);
        assert_eq!(SRC, plain_text);
    }
}