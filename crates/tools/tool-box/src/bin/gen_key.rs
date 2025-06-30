use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use jsonwebtoken::jwk::JwkSet;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::rand_core::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    let mut rng = OsRng;
    let bits = 2048; // 密钥长度，通常是 2048 或 4096
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    // Private Key (PKCS#8 PEM)
    // The `EncodePrivateKey` trait (from `pkcs8` crate) provides `to_pkcs8_pem` for `RsaPrivateKey`
    let private_key_pem = private_key.to_pkcs8_pem(pkcs8::LineEnding::LF)?;
    println!("{}", *private_key_pem);

    // Public Key (SPKI PEM)
    // The `EncodePublicKey` trait (from `pkcs8` crate) provides `to_spki_pem` for `RsaPublicKey`
    let public_key_pem = public_key.to_public_key_pem(pkcs8::LineEnding::LF)?;
    println!("{}", public_key_pem);

    let key = URL_SAFE_NO_PAD.encode("abcdefghijklmnopqrstuvwxyz012345");
    let jwks_json = json!({
        "keys": [
            {
                "kty": "oct",
                "alg": "HS256",
                "kid": "abc123",
                "k": key
            }
        ]
    });

    let set: JwkSet = serde_json::from_value(jwks_json).expect("Failed HS256 check");

    Ok(())
}
