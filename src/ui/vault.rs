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
use gtk::prelude::*;
use gtk::*;

use util::check_password_quality;

pub fn create_vault_ui() -> Box {
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

    ui

}
