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

pub fn create_views(stack: &Stack) {
    stack.add_named(&create_empty_view(), "empty-vault");
    stack.add_named(&create_error_view(), "error-vault");
    stack.add_named(&create_busy_view(), "busy-vault");
    stack.add_named(&create_empty_search_view(), "empty-search");
}

fn create_empty_view() -> Box {

    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/EmptyVault.glade");

    let view: Box = builder.get_object("empty-vault").unwrap();

    view

}

pub fn create_list_view() -> FlowBox {

    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/ListVault.glade");

    let flow: FlowBox = builder.get_object("list-box").unwrap();

    flow


}

fn create_empty_search_view() -> Box {

    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/EmptySearch.glade");

    let view: Box = builder.get_object("empty-search").unwrap();

    view

}

fn create_busy_view() -> Box {

    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/BusyVault.glade");

    let view: Box = builder.get_object("busy-box").unwrap();

    view

}

fn create_error_view() -> Box {

    let builder = Builder::new_from_resource("/org/gnieh/Repassync/ui/ErrorVault.glade");

    let view: Box = builder.get_object("error-box").unwrap();

    view

}
