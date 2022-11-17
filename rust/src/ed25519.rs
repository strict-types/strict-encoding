// LNP/BP client-side-validation foundation libraries implementing LNPBP
// specifications & standards (LNPBP-4, 7, 8, 9, 42, 81)
//
// Written in 2019-2022 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the Apache 2.0 License along with this
// software. If not, see <https://opensource.org/licenses/Apache-2.0>.

use std::io;

use ed25519_dalek::ed25519::signature::Signature;

use crate::{Error, StrictDecode, StrictEncode};

impl StrictEncode for ed25519_dalek::PublicKey {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(&self.as_bytes()[..])?)
    }
}

impl StrictDecode for ed25519_dalek::PublicKey {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; ed25519_dalek::PUBLIC_KEY_LENGTH];
        d.read_exact(&mut buf)?;
        Self::from_bytes(&buf).map_err(|_| {
            Error::DataIntegrityError(
                "invalid Curve25519 public key data".to_string(),
            )
        })
    }
}

impl StrictEncode for ed25519_dalek::Signature {
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        Ok(e.write(self.as_bytes())?)
    }
}

impl StrictDecode for ed25519_dalek::Signature {
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        let mut buf = [0u8; ed25519_dalek::SIGNATURE_LENGTH];
        d.read_exact(&mut buf)?;
        Self::from_bytes(&buf).map_err(|_| {
            Error::DataIntegrityError(
                "invalid Ed25519 signature data".to_string(),
            )
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ed25519() {
        let keypair = ed25519_dalek::Keypair::generate(&mut rand::thread_rng());

        let ser = keypair.public.strict_serialize().unwrap();
        assert_eq!(ser.len(), 32);
        assert_eq!(
            ed25519_dalek::PublicKey::strict_deserialize(ser).unwrap(),
            keypair.public
        );
    }

    #[test]
    fn x25519() {
        use ed25519_dalek::Signer;

        let keypair = ed25519_dalek::Keypair::generate(&mut rand::thread_rng());
        let message: &[u8] = b"This is a test of the tsunami alert system.";
        let signature = keypair.sign(message);

        let ser = signature.strict_serialize().unwrap();
        assert_eq!(ser.len(), 64);
        assert_eq!(
            ed25519_dalek::Signature::strict_deserialize(ser).unwrap(),
            signature
        );
    }
}
