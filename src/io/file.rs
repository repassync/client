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
use std::path::Path;

use std::fs::File;

use secstr::SecStr;

use serde::{Deserialize, Serialize};

use msgpack::{decode, encode};
use rmp;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct VaultFile {
    salt: Vec<u8>,
    iter: u32,
    encrypted: Vec<u8>,
    #[serde(skip_serializing)]
    clear: Vec<u8>
}

impl VaultFile {

    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<VaultFile, decode::Error> {
        match File::open(file_path) {
            Ok(file) =>
                decode::from_read(file),
            Err(e) =>
                Err(decode::Error::InvalidDataRead(e))
        }
    }

    pub fn to_file<P: AsRef<Path>>(&self, file_path: P) -> Result<(), encode::Error> {
        match File::open(file_path) {
            Ok(ref mut file) =>
                encode::write(file, self),
            Err(e) =>
                Err(encode::Error::InvalidValueWrite(rmp::encode::ValueWriteError::InvalidDataWrite(e)))
        }
    }

    pub fn decrypt(&mut self, password: SecStr) {
    }

}
