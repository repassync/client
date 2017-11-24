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
use DATADIR;

use std::path::Path;

use std::rc::Rc;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::*;
use gio::{Resource, resources_register};

use model::Vault;

use xdg;

use ui::vault::create_vault_ui;
use ui::entry::create_entry_ui;

pub struct App {
    window: ApplicationWindow,

    vault: Option<Vault>
}

impl App {

    pub fn new(app: &Application) -> Rc<RefCell<Self>> {

        // load and register the resource
        match Resource::load(Path::new(format!("{}/repassync.gresource", DATADIR).as_str())) {
            Ok(res) => {
                resources_register(&res);
            },
            Err(e) => {
                panic!("Unable to create application: {}", e);
            }
        }

        let window = ApplicationWindow::new(&app);

        let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/HeaderBar.glade");
        let header: HeaderBar = builder.get_object("header").unwrap();

        let new_entry_button: MenuButton = builder.get_object("add-toggle-button").unwrap();

        window.set_titlebar(&header);

        window.set_default_size(800, 600);
        window.set_position(WindowPosition::Center);

        let create_vault = create_vault_ui();
        window.add(&create_vault);

        let create_entry = create_entry_ui();
        new_entry_button.set_popover(&create_entry);
        //new_entry_button.set_sensitive(false);

        let xdg_dirs = xdg::BaseDirectories::with_prefix("repassync").unwrap();
        match xdg_dirs.find_data_file("repassync.vault") {
            Some(f) => {},
            None => {}
        }

        let app = App {
            window,

            vault: None
        };

        let me = Rc::new(RefCell::new(app));

        me

    }

    pub fn show(&self) {
        self.window.show_all();
    }

    pub fn set_vault(&mut self, vault: Vault) {
        self.vault = Some(vault);
        // TODO refresh UI for entries
    }

}
