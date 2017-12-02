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
use gtk::*;

use ui::entry::create_entry_ui;

pub struct Header {
    pub new_entry_button: MenuButton,
    pub search_button: ToggleButton,
    pub stack: Stack
}

pub fn create_header_bar_ui(search_bar: &SearchBar) -> Header {
    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/HeaderBar.glade");
    let stack: Stack = builder.get_object("header").unwrap();

    let new_entry_button: MenuButton = builder.get_object("add-toggle-button").unwrap();
    let search_button: ToggleButton = builder.get_object("search").unwrap();

    stack.set_visible_child_name("empty-bar");

    let create_entry = create_entry_ui();
    new_entry_button.set_popover(&create_entry);

    {
        let search_bar_bis = search_bar.clone();
        search_button.connect_property_active_notify(move |button| {
            search_bar_bis.set_search_mode(button.get_active());
        });
    }
    {
        let search_button_bis = search_button.clone();
        search_bar.connect_property_search_mode_enabled_notify(move |bar| {
            search_button_bis.set_active(bar.get_search_mode());
        });
    }

    Header {
        new_entry_button,
        search_button,
        stack
    }
}
