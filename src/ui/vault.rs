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
use std::rc::Rc;
use std::cell::RefCell;

use std::thread;
use std::sync::mpsc;

use gtk::prelude::*;
use gtk::*;
use glib;

use secstr::SecStr;

use util::check_password_quality;

use ui::App;

use model::Vault;

pub fn create_vault_ui(app: Rc<RefCell<App>>) -> Box {
    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/CreateVault.glade");

    let ui: Box = builder.get_object("create-vault-box").unwrap();
    let passphrase: Entry = builder.get_object("create-vault-password-1").unwrap();
    let show_passphrase: CheckButton = builder.get_object("create-vault-show-password").unwrap();
    let passphrase_confirm: Entry = builder.get_object("create-vault-password-2").unwrap();
    let level: LevelBar = builder.get_object("create-vault-password-strength").unwrap();
    let confirm_hint: Label = builder.get_object("create-vault-confirm-hint").unwrap();
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

    {
        let app_bis = app.clone();
        create.connect_clicked(move |_| {
            app_bis.borrow().set_busy();
            let pass = SecStr::new(passphrase.get_text().unwrap().into_bytes());
            let app_ter = app_bis.clone();
            CREATE.with(move |create| {
                *create.borrow_mut() =
                    Some((app_ter, CreateThread::new(pass, || { glib::idle_add(created); })));
            });
        });
    }

    ui

}

fn created() -> Continue {
    CREATE.with(move |create| {
        if let Some((ref app, ref create_thread)) = *create.borrow() {
            let (vault, pass) = create_thread.vault_channel.recv().unwrap();
            app.borrow_mut().set_vault(vault, pass);
        }
        *create.borrow_mut() = None;
    });
    Continue(false)
}

thread_local!(
    static CREATE: RefCell<Option<(Rc<RefCell<App>>, CreateThread)>> = RefCell::new(None)
);

struct CreateThread {
    vault_channel: mpsc::Receiver<(Vault, SecStr)>
}

impl CreateThread {
    fn new<F: Fn() + Send + 'static>(passphrase: SecStr, callback: F) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vault = Vault::new("".to_owned());
            tx.send((vault, passphrase)).unwrap();
            callback();
        });

        CreateThread { vault_channel: rx }
    }
}

pub fn create_unlock_vault_ui() -> Box {

    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/UnlockVault.glade");

    let ui: Box = builder.get_object("unlock-vault-box").unwrap();
    let password: Entry = builder.get_object("unlock-vault-password").unwrap();
    let error: Label = builder.get_object("unlock-vault-error").unwrap();
    let unlock: Button = builder.get_object("unlock-vault-unlock").unwrap();

    {
        let password_bis = password.clone();
        let unlock_bis = unlock.clone();
        password_bis.connect_changed(move |pwd| {
            let value = pwd.get_text();
            unlock_bis.set_sensitive(value.map(|t| t.len() > 0).unwrap_or_else(|| false));
        });
    }

    {
        let password_bis = password.clone();
        let unlock_bis = unlock.clone();
        unlock_bis.connect_clicked(move |b| {
        });
    }

    ui
}
