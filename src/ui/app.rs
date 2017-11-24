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
use util::{PasswordGenerator, check_password_quality};

use xdg;

use pwquality::PWQuality;

pub struct App {
    window: ApplicationWindow,
    new_entry_button: MenuButton,

    pwq: PWQuality,

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

        let pwq = PWQuality::new();

        let window = ApplicationWindow::new(&app);

        let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/HeaderBar.glade");
        let header: HeaderBar = builder.get_object("header").unwrap();

        let new_entry_button: MenuButton = builder.get_object("add-toggle-button").unwrap();
        new_entry_button.set_sensitive(false);

        window.set_titlebar(&header);

        window.set_default_size(800, 600);
        window.set_position(WindowPosition::Center);

        let create_vault = create_vault_ui();
        window.add(&create_vault);

        let create_entry = create_entry_ui();
        new_entry_button.set_popover(&create_entry);

        let xdg_dirs = xdg::BaseDirectories::with_prefix("repassync").unwrap();
        match xdg_dirs.find_data_file("repassync.vault") {
            Some(f) => {},
            None => {}
        }

        let app = App {
            window,
            new_entry_button,

            pwq,

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

thread_local!(
    static GEN: RefCell<Option<(GeneratorUI, GenThread)>> = RefCell::new(None)
);

#[derive(Clone)]
struct GeneratorUI {
    ui: Box,
    length: SpinButton,
    use_lower: CheckButton,
    use_upper: CheckButton,
    use_numbers: CheckButton,
    use_special: CheckButton,
    spinner: Spinner,
    password_field: Entry
}

fn generated() -> Continue {
    GEN.with(move |gen| {
        if let Some((ref ui, ref gen_thread)) = *gen.borrow() {
            let pass = gen_thread.password_channel.recv().unwrap();
            ui.password_field.set_text(pass.as_str());
            ui.spinner.stop();
            ui.ui.set_sensitive(true);
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
    create: Button
}

fn create_entry_ui() -> Popover {
    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/CreateEntry.glade");

    let ui: Popover = builder.get_object("add-popover").unwrap();
    let content: Box = builder.get_object("add-box").unwrap();
    let new_name: Entry = builder.get_object("new-name").unwrap();
    let add_button: Button = builder.get_object("add-button").unwrap();
    let password_field: Entry = builder.get_object("new-password").unwrap();
    let show_password: CheckButton = builder.get_object("show-password").unwrap();

    let length: SpinButton = builder.get_object("password-generator-length").unwrap();
    let use_lower: CheckButton = builder.get_object("password-generator-use-lower").unwrap();
    let use_upper: CheckButton = builder.get_object("password-generator-use-upper").unwrap();
    let use_numbers: CheckButton = builder.get_object("password-generator-use-numbers").unwrap();
    let use_special: CheckButton = builder.get_object("password-generator-use-special").unwrap();

    let generate_button: Button = builder.get_object("password-generator-generate-button").unwrap();
    let spinner: Spinner = builder.get_object("password-generator-working").unwrap();

    {
        let ui_bis = ui.clone();
        let length_bis = length.clone();
        let use_lower_bis = use_lower.clone();
        let use_upper_bis = use_upper.clone();
        let use_numbers_bis = use_numbers.clone();
        let use_special_bis = use_special.clone();
        let password_field_bis = password_field.clone();

        generate_button.connect_clicked(move |_| {
            content.set_sensitive(false);
            spinner.start();

            let length = length_bis.get_value_as_int() as usize;
            let use_lower = use_lower_bis.get_active();
            let use_upper = use_upper_bis.get_active();
            let use_numbers = use_numbers_bis.get_active();
            let use_special = use_special_bis.get_active();

            let me = GeneratorUI {
                ui: content.clone(),
                length: length_bis.clone(),
                use_lower: use_lower_bis.clone(),
                use_upper: use_upper_bis.clone(),
                use_numbers: use_numbers_bis.clone(),
                use_special: use_special_bis.clone(),
                spinner: spinner.clone(),
                password_field: password_field_bis.clone()
            };

            GEN.with(move |gen| {
                *gen.borrow_mut() =
                    Some((me, GenThread::new(length, use_lower, use_upper, use_numbers, use_special, || { glib::idle_add(generated); })));
            });
        });
    }

    {
        new_name.connect_changed(move |entry| {
            let txt = entry.get_text();
            if txt.is_some() && !txt.unwrap().is_empty() {
                add_button.set_sensitive(true);
            } else {
                add_button.set_sensitive(false);
            }
        });
    }

    {
        let password_field_bis = password_field.clone();
        show_password.connect_toggled(move |check| {
            let show = check.get_active();
            password_field_bis.set_visibility(show);
        });
    }

    ui
}

fn create_vault_ui() -> Box {
    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/CreateVault.glade");

    let ui: Box = builder.get_object("create-vault-box").unwrap();
    let passphrase: Entry = builder.get_object("create-vault-password-1").unwrap();
    let show_passphrase: CheckButton = builder.get_object("create-vault-show-password").unwrap();
    let passphrase_confirm: Entry = builder.get_object("create-vault-password-2").unwrap();
    let level: LevelBar = builder.get_object("create-vault-password-strength").unwrap();
    let confirm_hint: Label = builder.get_object("create-vault-confirm-hint").unwrap();
    let create: Button = builder.get_object("create-vault-create").unwrap();

    let pwq = PWQuality::new();
    pwq.set_min_length(10);
    pwq.set_digit_credit(-1);
    pwq.set_lowercase_credit(-1);
    pwq.set_uppercase_credit(-1);
    pwq.set_other_credit(-1);

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
        let level_bis = level.clone();
        passphrase_bis.connect_changed(move |entry| {
            let value = entry.get_text();

            let confirm_value = passphrase_confirm_bis.get_text();
            match (value, confirm_value) {
                (Some(value), Some(confirm_value)) => {
                    let quality = check_password_quality(value.as_str());
                    level_bis.set_value(f64::from(quality as i32));

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

    ui

}
