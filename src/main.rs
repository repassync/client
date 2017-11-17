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

extern crate gpgme;
extern crate chrono;
extern crate sha2;
extern crate rand;
extern crate char_iter;

mod ui;
mod model;
mod util;

use gtk::Application;
use gio::APPLICATION_FLAGS_NONE;
use gio::ApplicationExt;

use ui::App;

pub static DATADIR: &'static str = include!("datadir.in");

fn main() {
    match Application::new("org.gnieh.Repassync", APPLICATION_FLAGS_NONE) {
        Ok(app) => {
            // register this application as the default one for the process
            app.set_default();
            // build the application gui
            app.connect_activate(|app| {
                match App::new(&app) {
                    Ok(rep_app) => {
                        rep_app.show();
                    },
                    Err(e) => {
                        panic!("Unable to create the GUI: {:}", e)
                    }
                }
            });

            // Run GTK application
            app.run(&[]);

        },
        Err(e) => {
            panic!("Failed to initialize GTK: {}", e);
        }
    }


}
