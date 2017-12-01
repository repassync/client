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

pub struct MainWindow {
    pub ui: Box,
    pub search_bar: SearchBar,
    pub search_entry: SearchEntry,
    pub stack: Stack
}

pub fn create_main_window_ui() -> MainWindow {

    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/MainWindow.glade");

    let ui: Box = builder.get_object("box").unwrap();
    let search_bar: SearchBar = builder.get_object("search_bar").unwrap();
    let search_entry: SearchEntry = builder.get_object("search_entry").unwrap();
    let stack: Stack = builder.get_object("stack").unwrap();

    MainWindow {
        ui,
        search_bar,
        search_entry,
        stack
    }

}
