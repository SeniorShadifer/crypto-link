pub trait AssymetricEncryptionAlgorythm {
    fn keypair(
        &self
    ) -> Result<
        (Box<dyn pqcrypto::traits::kem::PublicKey>, Box<dyn pqcrypto::traits::kem::SecretKey>),
        Box<dyn std::error::Error>
    >;

    fn encapsulate(
        &self,
        public_key: Box<dyn pqcrypto::traits::kem::PublicKey>
    ) -> Result<
        (Box<dyn pqcrypto::traits::kem::SharedSecret>, Box<dyn pqcrypto::traits::kem::Ciphertext>),
        Box<dyn std::error::Error>
    >;

    fn decapsulate(
        &self,
        secret_key: Box<dyn pqcrypto::traits::kem::SecretKey>,
        ciphertext: Box<dyn pqcrypto::traits::kem::Ciphertext>
    ) -> Result<Box<dyn pqcrypto::traits::kem::SharedSecret>, Box<dyn std::error::Error>>;
}

pub trait AssymetricVerificationAlgorythm {
    fn keypair(
        &self
    ) -> Result<
        (Box<dyn pqcrypto::traits::sign::PublicKey>, Box<dyn pqcrypto::traits::sign::SecretKey>),
        Box<dyn std::error::Error>
    >;

    fn sign(
        &self,
        secret_key: Box<dyn pqcrypto::traits::sign::SecretKey>,
        message: &[u8]
    ) -> Result<
        Box<dyn pqcrypto::traits::sign::DetachedSignature>,
        pqcrypto::traits::sign::VerificationError
    >;

    fn verify(
        &self,
        public_key: Box<dyn pqcrypto::traits::sign::PublicKey>,
        signature: Box<dyn pqcrypto::traits::sign::DetachedSignature>,
        message: &[u8]
    );
}

pub trait SymmetricEncryptionAlgorythm {
    fn encrypt(&self, key: &[u8], message: &[u8]) -> &[u8];
    fn decrypt(&self, key: &[u8], message: &[u8]) -> &[u8];
}
