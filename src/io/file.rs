// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use std::collections::{HashMap, HashSet};

use std::path::Path;

use std::fs::File;

use chrono::prelude::*;
use chrono::serde::ts_seconds;

use secstr::{SecStr, SecVec};

use serde_bytes;
use serde_cbor::de::{from_reader, from_slice};
use serde_cbor::ser::{to_writer, to_vec};
use serde_cbor::error::Result;

use openssl::pkcs5::pbkdf2_hmac;
use openssl::hash::MessageDigest;
use openssl::symm::{encrypt, decrypt, Cipher};
use openssl::rand::rand_bytes;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct EncryptedVaultFile {
    version: u8,
    #[serde(with="serde_bytes")]
    salt: Vec<u8>,
    iter: u32,
    #[serde(with="serde_bytes")]
    iv: Vec<u8>,
    #[serde(with="serde_bytes")]
    encrypted: Vec<u8>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DecryptedVaultFile {
    owner: String,
    entries: HashMap<String, EncryptedEntry>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct EncryptedEntry {
    tags: HashSet<String>,
    #[serde(with="ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with="ts_seconds")]
    last_created: DateTime<Utc>,
    #[serde(with="serde_bytes")]
    salt: Vec<u8>,
    iter: u32,
    #[serde(with="serde_bytes")]
    iv: Vec<u8>,
    #[serde(with="serde_bytes")]
    encrypted: Vec<u8>
}

impl EncryptedVaultFile {

    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<EncryptedVaultFile> {
        use serde::de::Error;
        match File::open(file_path) {
            Ok(file) => {
                from_reader(file)
            },
            Err(e) => {
                Err(Error::custom(e))
            }
        }
    }

    pub fn to_file<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        use serde::ser::Error;
        match File::open(file_path) {
            Ok(ref mut file) => {
                to_writer(file, self)
            },
            Err(e) => {
                Err(Error::custom(e))
            }
        }
    }

    pub fn decrypt(&self, password: SecStr) -> Result<DecryptedVaultFile> {
        use serde::de::Error;
        // derive the 256 bits key
        let mut derived_key = SecVec::new(vec![0_u8; 32]);
        match pbkdf2_hmac(password.unsecure(), &self.salt, self.iter as usize, MessageDigest::sha256(), derived_key.unsecure_mut()) {
            Ok(()) => {
                // decipher with the 256 bits derived key
                let cipher = Cipher::aes_256_cbc();
                match decrypt(cipher, derived_key.unsecure(), Some(&self.iv), &self.encrypted) {
                    Ok(raw_decrypted) => {
                        from_slice(&raw_decrypted)
                    },
                    Err(e) => {
                        Err(Error::custom(e))
                    }
                }
            },
            Err(e) => {
                Err(Error::custom(e))
            }
        }
    }

}

impl DecryptedVaultFile {

    pub fn new(owner: String) -> Self {
        DecryptedVaultFile {
            owner: owner,
            entries: HashMap::new()
        }
    }

    pub fn encrypt(&self, password: SecStr, iter: u32) -> Result<EncryptedVaultFile> {
        use serde::ser::Error;
        // serializes data
        let raw_decrypted = to_vec(self)?;
        // generate 128 bits salt and iv
        let mut salt = [0; 16];
        let mut iv = [0; 16];
        match (rand_bytes(&mut salt), rand_bytes(&mut iv)) {
            (Ok(_), Ok(_)) => {
                // derive the 256 bits key
                let mut derived_key = SecVec::new(vec![0_u8; 32]);
                match pbkdf2_hmac(password.unsecure(), &salt, iter as usize, MessageDigest::sha256(), derived_key.unsecure_mut()) {
                    Ok(()) => {
                        // encrypt with the derived key and iv
                        let cipher = Cipher::aes_256_cbc();
                        match encrypt(cipher, derived_key.unsecure(), Some(&iv), &raw_decrypted) {
                            Ok(encrypted) => {
                                Ok(EncryptedVaultFile {
                                    version: 1,
                                    salt: salt.to_vec(),
                                    iter: iter,
                                    iv: iv.to_vec(),
                                    encrypted: encrypted
                                })
                            },
                            Err(e) => {
                                Err(Error::custom(e))
                            }
                        }
                    },
                    Err(e) => {
                        Err(Error::custom(e))
                    }
                }
            },
            (Err(e), _) => {
                Err(Error::custom(e))
            },
            (_, Err(e)) => {
                Err(Error::custom(e))
            }
        }
    }

}
