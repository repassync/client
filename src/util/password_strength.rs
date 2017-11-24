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
use pwquality::PWQuality;

#[derive(Clone, Debug)]
pub enum Quality {
    Weak,
    Low,
    Medium,
    Good,
    High
}

pub fn check_password_quality(password: &str) -> Quality {

    let pwq = PWQuality::new();
    pwq.set_min_length(10);

    let score = pwq.check(password.to_owned(), None, None).unwrap_or_else(|_| -1);

    let length = password.len();

    let strength = score.max(0).min(100);

    if score < 0 {
        if length > 0 {
            Quality::Low
        } else {
            Quality::Weak
        }
    } else if strength < 50 {
        Quality::Medium
    } else if strength < 75 {
        Quality::Good
    } else {
        Quality::High
    }

}
