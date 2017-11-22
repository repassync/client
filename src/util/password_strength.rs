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
extern crate libc;

use self::libc::{c_char, c_void};

use std::ffi::CString;

use std::ptr::{null, null_mut};

static PWQ_SETTING_MIN_LENGTH: i32 = 3;

#[link(name = "pwquality")]
extern {
    fn pwquality_default_settings() -> *const u8;
    fn pwquality_check(pwq: *const u8, password: *const c_char, oldpassword: *const c_char, user: *const c_char, auxerror: *mut c_void) -> i32;
    fn pwquality_set_int_value(pwq: *const u8, setting: i32, value: i32) -> i32;
}

#[derive(Clone, Debug)]
pub enum Quality {
    Weak,
    Low,
    Medium,
    Good,
    High
}

pub fn check_password_quality(password: &str) -> Quality {
    let c_password = CString::new(password).unwrap();
    let score =
        unsafe {
            let settings = pwquality_default_settings();
            pwquality_set_int_value(settings, PWQ_SETTING_MIN_LENGTH, 10);
            pwquality_check(settings, c_password.as_ptr(), null(), null(), null_mut())
        };

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
