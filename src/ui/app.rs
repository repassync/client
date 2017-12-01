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

use std::process;

use std::rc::Rc;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::*;
use gio::{Resource, resources_register};

use model::Vault;

use xdg;

use ui::vault::{create_vault_ui, create_unlock_vault_ui};
use ui::entry::create_entry_ui;
use ui::main_window::create_main_window_ui;
use ui::views::create_views;

use io::file::EncryptedVaultFile;

pub struct App {
    window: ApplicationWindow,

    file: Option<EncryptedVaultFile>,
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
                error!("Unable to create application: {}", e);
                process::exit(1);
            }
        }

        let window = ApplicationWindow::new(&app);

        let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/HeaderBar.glade");
        let header: HeaderBar = builder.get_object("header").unwrap();

        let new_entry_button: MenuButton = builder.get_object("add-toggle-button").unwrap();

        window.set_titlebar(&header);

        window.set_default_size(800, 600);
        window.set_position(WindowPosition::Center);

        let create_entry = create_entry_ui();
        new_entry_button.set_popover(&create_entry);
        new_entry_button.set_sensitive(false);

        let main_window = create_main_window_ui();
        window.add(&main_window.ui);

        create_views(&main_window.stack);

        let create_vault = create_vault_ui();
        main_window.stack.add_named(&create_vault, "create-vault");

        let unlock_vault = create_unlock_vault_ui();
        main_window.stack.add_named(&unlock_vault, "unlock-vault");

        let xdg_dirs = xdg::BaseDirectories::with_prefix("repassync").unwrap();
        let file =
            match xdg_dirs.find_data_file("repassync.vault") {
                Some(f) => {
                    match EncryptedVaultFile::from_file(f) {
                        Ok(f) => {
                            main_window.stack.set_visible_child_name("unlock-vault");
                            Some(f)
                        },
                        Err(e) => {
                            error!("Unable to open vault file: {}", e);
                            main_window.stack.set_visible_child_name("error-vault");
                            None
                        }
                    }
                },
                None => {
                    main_window.stack.set_visible_child_name("create-vault");
                    None
                }

            };

        let app = App {
            window,

            file,
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
