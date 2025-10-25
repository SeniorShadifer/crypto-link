use crate::crypto::{
    AssymetricEncryptionAlgorythm,
    DecapsulationError,
    EncapsulationError,
    KEMCiphertext,
    KEMPublicKey,
    KEMSecretKey,
    KEMSharedSecret,
};

pub struct Kyber1024;

impl AssymetricEncryptionAlgorythm for Kyber1024 {
    fn keypair(
        &self
    ) -> Result<(super::KEMPublicKey, super::KEMSecretKey), Box<dyn std::error::Error>> {
        let (public_key, secret_key) = pqcrypto_kyber::kyber1024_keypair();
        return Ok((KEMPublicKey::Kyber1024(public_key), KEMSecretKey::Kyber1024(secret_key)));
    }

    fn encapsulate(
        &self,
        public_key: KEMPublicKey
    ) -> Result<(super::KEMSharedSecret, super::KEMCiphertext), Box<dyn std::error::Error>> {
        match public_key {
            KEMPublicKey::Kyber1024(kyber1024_public_key) => {
                let (shared_secret, ciphertext) = pqcrypto_kyber::kyber1024_encapsulate(
                    &kyber1024_public_key
                );
                return Ok((
                    KEMSharedSecret::Kyber1024(shared_secret),
                    KEMCiphertext::Kyber1024(ciphertext),
                ));
            }
            _ => {
                return Err(Box::new(EncapsulationError::PublicKeyIncorrectType));
            }
        }
    }

    fn decapsulate(
        &self,
        secret_key: KEMSecretKey,
        ciphertext: KEMCiphertext
    ) -> Result<KEMSharedSecret, Box<dyn std::error::Error>> {
        match secret_key {
            KEMSecretKey::Kyber1024(kyber1024_secret_key) => {
                match ciphertext {
                    KEMCiphertext::Kyber1024(kyber1024_ciphertext) =>
                        Ok(
                            KEMSharedSecret::Kyber1024(
                                pqcrypto_kyber::kyber1024_decapsulate(
                                    &kyber1024_ciphertext,
                                    &kyber1024_secret_key
                                )
                            )
                        ),
                    _ => Err(Box::new(DecapsulationError::CiphertextIncorrectType)),
                }
            }
            _ => Err(Box::new(DecapsulationError::SecretKeyIncorrectType)),
        }
    }
}
