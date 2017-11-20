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

use xdg;

use gtk::prelude::*;
use glib;

use ui::App;
use model::Vault;

thread_local!(
    static LAUNCH: RefCell<Option<(Rc<RefCell<App>>, LaunchThread)>> = RefCell::new(None)
);

pub fn launch(app: Rc<RefCell<App>>) {

    LAUNCH.with(move |launch| {
        *launch.borrow_mut() =
            Some((app, LaunchThread::new(|| { glib::idle_add(launched); })));
    });

}

fn launched() -> Continue {
    LAUNCH.with(move |launch| {
        if let Some((ref app, ref launch_thread)) = *launch.borrow() {
            if let Some(vault) = launch_thread.vault_channel.recv().unwrap() {
                app.borrow_mut().set_vault(vault);
            }
        }
        *launch.borrow_mut() = None;
    });
    Continue(false)
}

struct LaunchThread {
    vault_channel: mpsc::Receiver<Option<Vault>>
}

impl LaunchThread {
    fn new<F: Fn() + Send + 'static>(callback: F) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let name = "".to_owned();

            let xdg_dirs = xdg::BaseDirectories::with_prefix("repassync").unwrap();

            let vault =
                xdg_dirs.find_data_file("repassync.vault").map(|f| {
                    println!("{:?}", f);
                    Vault::new(name)
                });
            tx.send(vault).unwrap();
            callback();
        });

        LaunchThread { vault_channel: rx }
    }
}
