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
use rand::{Rng, OsRng};

use char_iter;

pub struct PasswordGenerator {
    size: usize,
    use_lower: bool,
    use_upper: bool,
    use_numbers: bool,
    use_special: bool,
    rng: OsRng
}

impl PasswordGenerator {

    pub fn new(size: usize, use_lower: bool, use_upper: bool, use_numbers: bool, use_special: bool) -> PasswordGenerator {
        let rng = OsRng::new().expect("Error while getting RNG");
        PasswordGenerator {
            size,
            use_lower,
            use_upper,
            use_numbers,
            use_special,
            rng
        }
    }

    pub fn generate(&mut self) -> String {
        // the possible characters
        let mut chars = vec![];
        if self.use_lower {
            for c in char_iter::new('a', 'z') {
                chars.push(c);
            }
        }
        if self.use_upper {
            for c in char_iter::new('A', 'Z') {
                chars.push(c);
            }
        }
        if self.use_numbers {
            for c in char_iter::new('0', '9') {
                chars.push(c);
            }
        }
        if self.use_special {
            for c in char_iter::new('!', '/') {
                chars.push(c);
            }
            for c in char_iter::new(':', '@') {
                chars.push(c);
            }
            for c in char_iter::new('[', '`') {
                chars.push(c);
            }
            for c in char_iter::new('{', '~') {
                chars.push(c);
            }
        }

        let mut password = String::new();

        for _ in 0..self.size {
            let i = self.rng.gen_range::<usize>(0, chars.len());
            password.push(chars[i]);
        }

        password
    }

}
