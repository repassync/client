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
extern crate gtk;
extern crate gio;
extern crate glib;

extern crate chrono;
extern crate rand;
extern crate char_iter;
extern crate xdg;
extern crate secstr;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate serde_cbor;
extern crate openssl;
extern crate pwquality;
extern crate env_logger;
#[macro_use]
extern crate log;

mod ui;
mod model;
mod util;
mod io;

use gtk::Application;
use gio::APPLICATION_FLAGS_NONE;
use gio::ApplicationExt;

use ui::App;

pub static DATADIR: &'static str = include!(concat!(env!("OUT_DIR"), "/datadir.in"));

fn main() {

    env_logger::init().expect("Failed to initialize logging");

    match Application::new("org.gnieh.Repassync", APPLICATION_FLAGS_NONE) {
        Ok(app) => {
            // register this application as the default one for the process
            app.set_default();
            // build the application gui
            app.connect_activate(|app| {
                let rep_app = App::new(&app);
                rep_app.borrow().show();
            });

            // Run GTK application
            app.run(&[]);

        },
        Err(e) => {
            error!("Failed to initialize GTK: {}", e);
        }
    }


}
