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

use secstr::SecStr;

use gtk::prelude::*;
use gtk::*;
use gio::{Resource, resources_register};

use model::Vault;

use xdg;

use ui::vault::{create_vault_ui, create_unlock_vault_ui};
use ui::header_bar::{Header, create_header_bar_ui};
use ui::main_window::{MainWindow, create_main_window_ui};
use ui::views::create_views;

use io::file::EncryptedVaultFile;

enum LoadedVault {
    LockedVault(EncryptedVaultFile),
    UnlockedVault(Vault, SecStr),
    NoVault
}

pub struct App {
    window: ApplicationWindow,
    header: Header,
    main_window: MainWindow,

    vault: LoadedVault
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

        window.set_default_size(800, 600);
        window.set_position(WindowPosition::Center);

        let main_window = create_main_window_ui();
        window.add(&main_window.ui);

        let header = create_header_bar_ui(&main_window.search_bar);
        window.set_titlebar(&header.stack);


        let main_window_bis = main_window.clone();

        create_views(&main_window.stack);

        let me = Rc::new(RefCell::new(App {
            window,
            header,
            main_window,

            vault: LoadedVault::NoVault
        }));

        let create_vault = create_vault_ui(me.clone());
        main_window_bis.stack.add_named(&create_vault, "create-vault");

        let unlock_vault = create_unlock_vault_ui();
        main_window_bis.stack.add_named(&unlock_vault, "unlock-vault");

        let xdg_dirs = xdg::BaseDirectories::with_prefix("repassync").unwrap();
        match xdg_dirs.find_data_file("repassync.vault") {
            Some(f) => {
                match EncryptedVaultFile::from_file(f) {
                    Ok(f) => {
                        main_window_bis.stack.set_visible_child_name("unlock-vault");
                        me.borrow_mut().vault = LoadedVault::LockedVault(f)
                    },
                    Err(e) => {
                        error!("Unable to open vault file: {}", e);
                        main_window_bis.stack.set_visible_child_name("error-vault");
                    }
                }
            },
            None => {
                main_window_bis.stack.set_visible_child_name("create-vault");
            }

        }

        me

    }

    pub fn show(&self) {
        self.window.show_all();
    }

    pub fn set_vault(&mut self, vault: Vault, pass: SecStr) {
        self.vault = LoadedVault::UnlockedVault(vault, pass);
        self.refresh();
    }

    pub fn refresh(&self) {
        use self::LoadedVault::*;
        match self.vault {
            UnlockedVault(ref vault, _) => {
                self.main_window.stack.set_visible_child_name("empty-vault");
                self.header.stack.set_visible_child_name("password-list");
            },
            LockedVault(_) => {
                self.main_window.stack.set_visible_child_name("unlock-vault");
                self.header.stack.set_visible_child_name("empty-bar");
            },
            NoVault => {
                self.main_window.stack.set_visible_child_name("create-vault");
                self.header.stack.set_visible_child_name("empty-bar");
            }
        }
        self.header.stack.set_sensitive(true);
    }

    pub fn set_busy(&self) {
        self.main_window.stack.set_visible_child_name("busy-vault");
        self.header.stack.set_sensitive(false);
    }

}
