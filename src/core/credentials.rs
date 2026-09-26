// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::Error;
use aws_lc_rs::{hmac, signature};
use base64::Engine as _;
use std::{fmt, fmt::Write as _, sync::Arc};
use zeroize::Zeroizing;

/// A signing boundary for externally held or hardware-backed key material.
pub trait Signer: Send + Sync {
    /// Sign the exact Binance payload and return its wire signature.
    ///
    /// # Errors
    /// Return `Error::Signing` without exposing key material on signing failure.
    fn sign(&self, payload: &[u8]) -> Result<String, Error>;
}

enum Key {
    Hmac(Zeroizing<Vec<u8>>),
    Rsa(signature::RsaKeyPair),
    Ed25519(signature::Ed25519KeyPair),
}
impl Signer for Key {
    fn sign(&self, payload: &[u8]) -> Result<String, Error> {
        match self {
            Self::Hmac(secret) => {
                let key = hmac::Key::new(hmac::HMAC_SHA256, secret);
                let tag = hmac::sign(&key, payload);
                let mut output = String::with_capacity(64);
                for byte in tag.as_ref() {
                    write!(&mut output, "{byte:02x}").map_err(|_| Error::Signing)?;
                }
                Ok(output)
            }
            Self::Ed25519(key) => {
                Ok(base64::engine::general_purpose::STANDARD.encode(key.sign(payload).as_ref()))
            }
            Self::Rsa(key) => {
                let mut output = vec![0; key.public_modulus_len()];
                key.sign(
                    &signature::RSA_PKCS1_SHA256,
                    &aws_lc_rs::rand::SystemRandom::new(),
                    payload,
                    &mut output,
                )
                .map_err(|_| Error::Signing)?;
                Ok(base64::engine::general_purpose::STANDARD.encode(output))
            }
        }
    }
}

/// Redacted credentials. Key bytes are zeroized on drop; clones share ownership.
#[derive(Clone)]
pub struct Credentials {
    api_key: Arc<Zeroizing<String>>,
    signer: Arc<dyn Signer>,
    ed25519: bool,
}
impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Credentials([REDACTED])")
    }
}
impl Credentials {
    /// HMAC-SHA256 authentication.
    ///
    /// # Errors
    /// Refuses empty credentials or an invalid API key header.
    pub fn hmac(api_key: impl Into<String>, secret: impl Into<String>) -> Result<Self, Error> {
        let secret = Zeroizing::new(secret.into());
        if secret.is_empty() {
            return Err(Error::Validation("empty signing key"));
        }
        Self::external(
            api_key,
            Arc::new(Key::Hmac(Zeroizing::new(secret.as_bytes().to_vec()))),
        )
    }
    /// RSA PKCS#8 DER authentication, using PKCS#1 v1.5 with SHA-256.
    ///
    /// # Errors
    /// Refuses invalid keys or API key headers.
    pub fn rsa_pkcs8(
        api_key: impl Into<String>,
        private_key: Zeroizing<Vec<u8>>,
    ) -> Result<Self, Error> {
        let key = signature::RsaKeyPair::from_pkcs8(&private_key).map_err(|_| Error::Signing)?;
        drop(private_key);
        Self::external(api_key, Arc::new(Key::Rsa(key)))
    }
    /// Ed25519 PKCS#8 DER authentication.
    ///
    /// # Errors
    /// Refuses invalid keys or API key headers.
    pub fn ed25519_pkcs8(
        api_key: impl Into<String>,
        private_key: Zeroizing<Vec<u8>>,
    ) -> Result<Self, Error> {
        let key =
            signature::Ed25519KeyPair::from_pkcs8(&private_key).map_err(|_| Error::Signing)?;
        drop(private_key);
        Self::external_ed25519(api_key, Arc::new(Key::Ed25519(key)))
    }
    /// Attach an externally owned signer, such as an HSM.
    ///
    /// # Errors
    /// Refuses an empty or invalid API key header.
    pub fn external(api_key: impl Into<String>, signer: Arc<dyn Signer>) -> Result<Self, Error> {
        let api_key = Zeroizing::new(api_key.into());
        if api_key.is_empty() || reqwest::header::HeaderValue::from_str(&api_key).is_err() {
            return Err(Error::Validation("API key header"));
        }
        Ok(Self {
            api_key: Arc::new(api_key),
            signer,
            ed25519: false,
        })
    }
    /// Attach an external Ed25519 signer, permitting WebSocket session authentication.
    /// The caller guarantees the signer's algorithm; payloads are still signed explicitly.
    ///
    /// # Errors
    /// Refuses an empty or invalid API key header.
    pub fn external_ed25519(
        api_key: impl Into<String>,
        signer: Arc<dyn Signer>,
    ) -> Result<Self, Error> {
        let mut credentials = Self::external(api_key, signer)?;
        credentials.ed25519 = true;
        Ok(credentials)
    }
    pub(crate) fn is_ed25519(&self) -> bool {
        self.ed25519
    }
    pub(crate) fn header(&self) -> Result<reqwest::header::HeaderValue, Error> {
        let mut header = reqwest::header::HeaderValue::from_str(&self.api_key)
            .map_err(|_| Error::Validation("API key header"))?;
        header.set_sensitive(true);
        Ok(header)
    }
    pub(crate) fn api_key(&self) -> &str {
        &self.api_key
    }
    pub(crate) fn sign(&self, payload: &str) -> Result<String, Error> {
        self.signer
            .sign(payload.as_bytes())
            .map_err(|_| Error::Signing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_lc_rs::{
        encoding::{AsDer, Pkcs8V1Der},
        signature::KeyPair as _,
    };

    #[test]
    fn hmac_matches_rfc_4231_vector() {
        let signer = Key::Hmac(Zeroizing::new(vec![0x0b; 20]));
        assert_eq!(
            signer.sign(b"Hi There").unwrap(),
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
    }

    #[test]
    fn ed25519_pkcs8_signatures_verify_and_keys_remain_redacted() {
        // Public synthetic seed; no exchange account has this key.
        let pair = signature::Ed25519KeyPair::from_seed_unchecked(&[7; 32]).unwrap();
        let der: Pkcs8V1Der<'_> = pair.as_der().unwrap();
        let credentials =
            Credentials::ed25519_pkcs8("synthetic-api-key", Zeroizing::new(der.as_ref().to_vec()))
                .unwrap();
        let signature = base64::engine::general_purpose::STANDARD
            .decode(credentials.sign("exact=payload").unwrap())
            .unwrap();
        signature::UnparsedPublicKey::new(&signature::ED25519, pair.public_key())
            .verify(b"exact=payload", &signature)
            .unwrap();
        assert!(credentials.is_ed25519());
        assert!(!format!("{credentials:?}").contains("synthetic-api-key"));
        assert!(Credentials::rsa_pkcs8("synthetic", Zeroizing::new(vec![1, 2, 3])).is_err());
        assert!(Credentials::ed25519_pkcs8("synthetic", Zeroizing::new(vec![1, 2, 3])).is_err());
    }
}
