pub trait AssymetricEncryptionAlgorythm {
    fn keypair(&self) -> Result<(KEMPublicKey, KEMSecretKey), Box<dyn std::error::Error>>;

    fn encapsulate(
        &self,
        public_key: KEMPublicKey
    ) -> Result<(KEMSharedSecret, KEMCiphertext), Box<dyn std::error::Error>>;

    fn decapsulate(
        &self,
        secret_key: KEMSecretKey,
        ciphertext: KEMCiphertext
    ) -> Result<KEMSharedSecret, Box<dyn std::error::Error>>;
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

pub enum KEMPublicKey {
    Polymorphic(Box<dyn pqcrypto::traits::kem::PublicKey>),
    Kyber1024(pqcrypto_kyber::kyber1024::PublicKey),
}

pub enum KEMSecretKey {
    Polymorphic(Box<dyn pqcrypto::traits::kem::SecretKey>),
    Kyber1024(pqcrypto_kyber::kyber1024::SecretKey),
}

pub enum KEMCiphertext {
    Polymorphic(Box<dyn pqcrypto::traits::kem::Ciphertext>),
    Kyber1024(pqcrypto_kyber::kyber1024::Ciphertext),
}

pub enum KEMSharedSecret {
    Polymorphic(Box<dyn pqcrypto::traits::kem::SharedSecret>),
    Kyber1024(pqcrypto_kyber::kyber1024::SharedSecret),
}

custom_error::custom_error! { pub EncapsulationError
    PublicKeyIncorrectType = "Public key have incorrect type"
}

custom_error::custom_error! {pub DecapsulationError
    SecretKeyIncorrectType = "Secret key have incorrect type",
    CiphertextIncorrectType = "Cipher text have incorrect type"
}

mod kyber1024;

pub enum SupportedAssymetricEncryptionAlgorythms {
    Kyber1024(kyber1024::Kyber1024),
}

impl SupportedAssymetricEncryptionAlgorythms {
    pub fn select_algorythm_by_str(name: &str) -> Box<dyn AssymetricEncryptionAlgorythm> {
        match name {
            "Kyber1024" => Box::new(kyber1024::Kyber1024),
            _ => Box::new(kyber1024::Kyber1024),
        }
    }
}
