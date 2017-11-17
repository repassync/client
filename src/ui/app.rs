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
use gio::{Resource, Error, resources_register};

use model::Vault;
use util::PasswordGenerator;

pub struct App {
    builder: Builder,
    window: ApplicationWindow,
    new_name: Entry,
    new_password: Entry,
    show_password: CheckButton,
    add_button: Button,
    password_generator: GeneratorUI,

    vault: Option<Rc<Vault>>
}

impl App {

    pub fn new(app: &Application) -> Result<Rc<Self>, Error> {

        // load and register the resource
        match Resource::load(Path::new(format!("{}/repassync.gresource", DATADIR).as_str())) {
            Ok(res) => {
                resources_register(&res);
            },
            Err(e) => {
                return Err(e)
            }
        }

        let window = ApplicationWindow::new(&app);

        let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/HeaderBar.glade");
        let header: HeaderBar = builder.get_object("header").unwrap();

        let new_name: Entry = builder.get_object("new-name").unwrap();

        let add_button: Button = builder.get_object("add-button").unwrap();

        let new_password: Entry = builder.get_object("new-password").unwrap();
        let show_password: CheckButton = builder.get_object("show-password").unwrap();

        window.set_titlebar(&header);

        window.set_default_size(800, 600);
        window.set_position(WindowPosition::Center);

        let password_generator = GeneratorUI::new(&new_password);
        let expander: Expander = builder.get_object("add-password-options").unwrap();
        expander.add(&password_generator.generator_ui);

        let app = App {
            builder,
            window,
            new_name,
            new_password,
            show_password,
            add_button,
            password_generator,

            vault: None
        };

        let me = Rc::new(app);
        {
            let me_too = me.clone();
            me.new_name.connect_changed(move |entry| {
                let txt = entry.get_text();
                if txt.is_some() && !txt.unwrap().is_empty() {
                    me_too.add_button.set_sensitive(true);
                } else {
                    me_too.add_button.set_sensitive(false);
                }
            });
        }

        {
            let me_too = me.clone();
            me.show_password.connect_toggled(move |check| {
                let check_too = check.clone();
                let new_password = me_too.new_password.clone();
                idle_add(move || {
                    let show = check_too.get_active();
                    new_password.set_visibility(show);
                    Continue(false)
                });
            });
        }

        Ok(me)

    }

    pub fn show(&self) {
        self.window.show_all();
    }

}

struct GeneratorUI {
    generator_ui: Box,
    length: SpinButton,
    use_lower: CheckButton,
    use_upper: CheckButton,
    use_numbers: CheckButton,
    use_special: CheckButton,
}

impl GeneratorUI {

    fn new(new_password: &Entry) -> GeneratorUI {
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

        {
            let length_too = length.clone();
            let use_lower_too = use_lower.clone();
            let use_upper_too = use_upper.clone();
            let use_numbers_too = use_numbers.clone();
            let use_special_too = use_special.clone();
            let password_field_bis = new_password.clone();
            generate_button.connect_clicked(move |_| {
                ui_bis.set_sensitive(false);
                spinner.set_visible(true);
                spinner.start();
                let mut gen = PasswordGenerator::new(
                    length_too.get_value_as_int() as usize,
                    use_lower_too.get_active(),
                    use_upper_too.get_active(),
                    use_numbers_too.get_active(),
                    use_special_too.get_active());
                let pass = gen.generate();
                let spinner_bis = spinner.clone();
                let ui_ter = ui_bis.clone();
                let password_field_ter = password_field_bis.clone();
                idle_add(move || {
                    password_field_ter.set_text(pass.as_str());
                    spinner_bis.stop();
                    spinner_bis.set_visible(false);
                    ui_ter.set_sensitive(true);
                    Continue(false)
                });
            });
        }

        GeneratorUI {
            generator_ui,
            length,
            use_lower,
            use_upper,
            use_numbers,
            use_special,
        }

    }

}
