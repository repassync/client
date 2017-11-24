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
use std::cell::RefCell;

use std::thread;
use std::sync::mpsc;

use gtk::prelude::*;
use gtk::*;
use glib;

use util::PasswordGenerator;

pub fn create_entry_ui() -> Popover {
    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/CreateEntry.glade");

    let ui: Popover = builder.get_object("add-popover").unwrap();
    let content: Grid = builder.get_object("add-grid").unwrap();
    let new_name: Entry = builder.get_object("new-name").unwrap();
    let add_button: Button = builder.get_object("add-button").unwrap();
    let password_field: Entry = builder.get_object("new-password").unwrap();
    let show_password: CheckButton = builder.get_object("show-password").unwrap();

    let length: SpinButton = builder.get_object("password-generator-length").unwrap();
    let use_lower: CheckButton = builder.get_object("password-generator-use-lower").unwrap();
    let use_upper: CheckButton = builder.get_object("password-generator-use-upper").unwrap();
    let use_numbers: CheckButton = builder.get_object("password-generator-use-numbers").unwrap();
    let use_special: CheckButton = builder.get_object("password-generator-use-special").unwrap();

    let generate_enabled: CheckButton = builder.get_object("password-generator-enable").unwrap();
    let generate_button: Button = builder.get_object("password-generator-generate-button").unwrap();
    let spinner: Spinner = builder.get_object("password-generator-working").unwrap();

    {
        let length_bis = length.clone();
        let use_lower_bis = use_lower.clone();
        let use_upper_bis = use_upper.clone();
        let use_numbers_bis = use_numbers.clone();
        let use_special_bis = use_special.clone();
        let generate_button_bis = generate_button.clone();
        generate_enabled.connect_toggled(move |toggle| {
            let gen = toggle.get_active();
            length_bis.set_sensitive(gen);
            use_lower_bis.set_sensitive(gen);
            use_upper_bis.set_sensitive(gen);
            use_numbers_bis.set_sensitive(gen);
            use_special_bis.set_sensitive(gen);
            generate_button_bis.set_sensitive(gen);
        });
    }

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

thread_local!(
    static GEN: RefCell<Option<(GeneratorUI, GenThread)>> = RefCell::new(None)
);

#[derive(Clone)]
struct GeneratorUI {
    ui: Grid,
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
