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

use std::thread;
use std::sync::mpsc;

use gtk::prelude::*;
use gtk::*;
use gio::{Resource, resources_register};
use glib;

use model::Vault;
use util::PasswordGenerator;

use xdg;

pub struct App {
    window: ApplicationWindow,
    new_entry_button: MenuButton,
    new_name: Entry,
    new_password: Entry,
    show_password: CheckButton,
    add_button: Button,

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

        let new_name: Entry = builder.get_object("new-name").unwrap();

        let add_button: Button = builder.get_object("add-button").unwrap();

        let new_entry_button: MenuButton = builder.get_object("add-toggle-button").unwrap();
        new_entry_button.set_sensitive(false);

        let new_password: Entry = builder.get_object("new-password").unwrap();
        let show_password: CheckButton = builder.get_object("show-password").unwrap();

        window.set_titlebar(&header);

        window.set_default_size(800, 600);
        window.set_position(WindowPosition::Center);

        let create_vault = CreateVault::new();
        window.add(&create_vault.ui);

        let password_generator = GeneratorUI::new(new_password.clone());
        let expander: Expander = builder.get_object("add-password-options").unwrap();
        expander.add(&password_generator.generator_ui);

        let xdg_dirs = xdg::BaseDirectories::with_prefix("repassync").unwrap();
        match xdg_dirs.find_data_file("repassync.vault") {
            Some(f) => {},
            None => {}
        }

        let app = App {
            window,
            new_entry_button,
            new_name,
            new_password,
            show_password,
            add_button,

            vault: None
        };

        let me = Rc::new(RefCell::new(app));
        {
            let me_too = me.clone();
            me.borrow().new_name.connect_changed(move |entry| {
                let txt = entry.get_text();
                if txt.is_some() && !txt.unwrap().is_empty() {
                    me_too.borrow().add_button.set_sensitive(true);
                } else {
                    me_too.borrow().add_button.set_sensitive(false);
                }
            });
        }

        {
            let me_too = me.clone();
            me.borrow().show_password.connect_toggled(move |check| {
                let show = check.get_active();
                me_too.borrow().new_password.set_visibility(show);
            });
        }

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

thread_local!(
    static GEN: RefCell<Option<(GeneratorUI, GenThread)>> = RefCell::new(None)
);

#[derive(Clone)]
struct GeneratorUI {
    generator_ui: Box,
    length: SpinButton,
    use_lower: CheckButton,
    use_upper: CheckButton,
    use_numbers: CheckButton,
    use_special: CheckButton,
    spinner: Spinner,
    password_field: Entry
}

impl GeneratorUI {

    fn new(password_field: Entry) -> GeneratorUI {
        let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/PasswordGenerator.glade");

        let generator_ui: Box = builder.get_object("password-generator-options-box").unwrap();
        let ui_bis = generator_ui.clone();

        let length: SpinButton = builder.get_object("password-generator-length").unwrap();
        let use_lower: CheckButton = builder.get_object("password-generator-use-lower").unwrap();
        let use_upper: CheckButton = builder.get_object("password-generator-use-upper").unwrap();
        let use_numbers: CheckButton = builder.get_object("password-generator-use-numbers").unwrap();
        let use_special: CheckButton = builder.get_object("password-generator-use-special").unwrap();

        let generate_button: Button = builder.get_object("password-generator-generate-button").unwrap();
        let spinner: Spinner = builder.get_object("password-generator-working").unwrap();

        let length_too = length.clone();
        let use_lower_too = use_lower.clone();
        let use_upper_too = use_upper.clone();
        let use_numbers_too = use_numbers.clone();
        let use_special_too = use_special.clone();
        let spinner_bis = spinner.clone();

        let me = GeneratorUI {
            generator_ui,
            length,
            use_lower,
            use_upper,
            use_numbers,
            use_special,
            spinner,
            password_field
        };

        {
            let me_bis = me.clone();
            generate_button.connect_clicked(move |_| {
                ui_bis.set_sensitive(false);
                spinner_bis.set_visible(true);
                spinner_bis.start();

                let length = length_too.get_value_as_int() as usize;
                let use_lower = use_lower_too.get_active();
                let use_upper = use_upper_too.get_active();
                let use_numbers = use_numbers_too.get_active();
                let use_special = use_special_too.get_active();

                let me_ter = me_bis.clone();
                GEN.with(move |gen| {
                    *gen.borrow_mut() =
                        Some((me_ter, GenThread::new(length, use_lower, use_upper, use_numbers, use_special, || { glib::idle_add(generated); })));
                });
            });
        }

        me

    }

}

fn generated() -> Continue {
    GEN.with(move |gen| {
        if let Some((ref ui, ref gen_thread)) = *gen.borrow() {
            let pass = gen_thread.password_channel.recv().unwrap();
            ui.password_field.set_text(pass.as_str());
            ui.spinner.stop();
            ui.spinner.set_visible(false);
            ui.generator_ui.set_sensitive(true);
        }
        *gen.borrow_mut() = None;
    });
    Continue(false)
}

struct GenThread {
    password_channel: mpsc::Receiver<String>
}

impl GenThread {

    fn new<F: Fn() + Send + 'static>(length: usize, use_lower: bool, use_upper: bool, use_numbers: bool, use_special: bool, callback: F) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut gen = PasswordGenerator::new(
                length,
                use_lower,
                use_upper,
                use_numbers,
                use_special);
            let pass = gen.generate();
            tx.send(pass).unwrap();
            callback();
        });

        GenThread { password_channel: rx }
    }
}

struct CreateVault {
    ui: Box,
    passphrase: Entry,
    show_passphrase: CheckButton,
    passphrase_confirm: Entry,
    level: LevelBar,
    confirm_hint: Label,
    cancel: Button,
    create: Button
}

impl CreateVault {

    fn new() -> Self {
        let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/CreateVault.glade");

        let ui: Box = builder.get_object("create-vault-box").unwrap();
        let passphrase: Entry = builder.get_object("create-vault-password-1").unwrap();
        let show_passphrase: CheckButton = builder.get_object("create-vault-show-password").unwrap();
        let passphrase_confirm: Entry = builder.get_object("create-vault-password-2").unwrap();
        let level: LevelBar = builder.get_object("create-vault-password-strength").unwrap();
        let confirm_hint: Label = builder.get_object("create-vault-confirm-hint").unwrap();
        let cancel: Button = builder.get_object("create-vault-cancel").unwrap();
        let create: Button = builder.get_object("create-vault-create").unwrap();

        {
            let show_passphrase_bis = show_passphrase.clone();
            let passphrase_bis = passphrase.clone();
            show_passphrase_bis.connect_toggled(move |toggle| {
                let show = toggle.get_active();
                passphrase_bis.set_visibility(show);
            });
        }

        {
            let passphrase_bis = passphrase.clone();
            let passphrase_confirm_bis = passphrase_confirm.clone();
            let confirm_hint_bis = confirm_hint.clone();
            let create_bis = create.clone();
            passphrase_bis.connect_changed(move |entry| {
                let value = entry.get_text();
                let confirm_value = passphrase_confirm_bis.get_text();
                match (value, confirm_value) {
                    (Some(value), Some(confirm_value)) => {
                        if value.len() > 0 && value == confirm_value {
                            create_bis.set_sensitive(true);
                            confirm_hint_bis.set_opacity(0.0);
                        } else {
                            create_bis.set_sensitive(false);
                            if confirm_value.len() > 0 {
                                confirm_hint_bis.set_opacity(1.0);
                            } else {
                                confirm_hint_bis.set_opacity(0.0);
                            }
                        }
                    },
                    (_, confirm_value) => {
                        create_bis.set_sensitive(false);
                        if confirm_value.is_some() && confirm_value.unwrap().len() > 0 {
                            confirm_hint_bis.set_opacity(1.0);
                        } else {
                            confirm_hint_bis.set_opacity(0.0);
                        }
                    }
                }
            });
        }

        {
            let passphrase_bis = passphrase.clone();
            let passphrase_confirm_bis = passphrase_confirm.clone();
            let confirm_hint_bis = confirm_hint.clone();
            let create_bis = create.clone();
            passphrase_confirm_bis.connect_changed(move |entry| {
                let confirm_value = entry.get_text();
                let value = passphrase_bis.get_text();
                match (value, confirm_value) {
                    (Some(value), Some(confirm_value)) => {
                        if value.len() > 0 && value == confirm_value {
                            create_bis.set_sensitive(true);
                            confirm_hint_bis.set_opacity(0.0);
                        } else {
                            create_bis.set_sensitive(false);
                            if confirm_value.len() > 0 {
                                confirm_hint_bis.set_opacity(1.0);
                            } else {
                                confirm_hint_bis.set_opacity(0.0);
                            }
                        }
                    },
                    (_, confirm_value) => {
                        create_bis.set_sensitive(false);
                        if confirm_value.is_some() && confirm_value.unwrap().len() > 0 {
                            confirm_hint_bis.set_opacity(1.0);
                        } else {
                            confirm_hint_bis.set_opacity(0.0);
                        }
                    }
                }
            });
        }

        CreateVault {
            ui,
            passphrase,
            show_passphrase,
            passphrase_confirm,
            level,
            confirm_hint,
            cancel,
            create
        }
    }

}
