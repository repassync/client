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
use chrono::prelude::*;
use std::collections::{HashSet, HashMap};
use sha2::{Sha256, Digest};
use std::collections::LinkedList;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Vault {
    owner: String,
    entries: RefCell<HashMap<String, Rc<Entry>>>,
    history: RefCell<LinkedList<Change>>
}

impl Vault {

    pub fn new(owner: String) -> Vault {
        Vault {
            owner: owner,
            entries: RefCell::new(HashMap::new()),
            history: RefCell::new(LinkedList::new())
        }
    }

    // TODO when PR#45870 is released, transform `me` to `self`
    pub fn add_entry(me: Rc<Self>, name: String, password: String) -> Rc<Entry> {
        let entry = Rc::new(Entry::new(name.clone(), password));
        *entry.vault.borrow_mut() = Rc::downgrade(&me);
        me.entries.borrow_mut().insert(name, entry.clone());
        entry
    }

}

#[derive(Debug, Clone)]
pub struct Entry {
    vault: RefCell<Weak<Vault>>,
    pub name: String,
    password: String,
    pub comment: Option<String>,
    pub user: Option<String>,
    pub uri: Option<String>,
    pub tags: HashSet<String>,
    created: DateTime<Utc>,
    last_modified: DateTime<Utc>
}

impl Entry {

    fn new(name: String, password: String) -> Entry {
        let created = Utc::now();
        return Entry {
            vault: RefCell::new(Weak::new()),
            name: name,
            password: password,
            comment: None,
            user: None,
            uri: None,
            created: created,
            last_modified: created,
            tags: HashSet::new()
        }
    }

    // TODO when PR#45870 is released, transform `me` to `self`
    pub fn new_password(mut me: Rc<Self>, author: String, password: String) -> Rc<Entry> {

        let date = Utc::now();

        let mut new_entry = Rc::get_mut(&mut me).unwrap().clone();
        let old_pass = new_entry.password;
        new_entry.password = password;

        // compute the id of the change
        // it is the sha256 of the concatenation of:
        //  - the author
        //  - the old password
        //  - the new password
        //  - the modification date in RFC3339 format
        //  - the id of the parent change (if any)
        let mut hasher = Sha256::default();
        hasher.input(author.as_bytes());
        hasher.input(old_pass.as_bytes());
        hasher.input(&new_entry.password.as_bytes());
        hasher.input(date.to_rfc3339().as_bytes());
        // the parent change is the front one
        // in the history if any
        //if let Some(&Change { ref id, .. }) = new_entry.history.front() {
        //    hasher.input(id.as_bytes())
        //}

        let mut change_id = String::new();
        change_id.push_str(&format!("{:x}", hasher.result()));

        //new_entry.history.push_front(Change {
        //    author: author,
        //    date: date,
        //    id: change_id,
        //    old_password: old_pass
        //});
        new_entry.last_modified = date;
        return Rc::new(new_entry);
    }

}

#[derive(Debug, Clone)]
pub struct Change {
    pub id: String,
    pub date: DateTime<Utc>,
    old_password: String,
    pub author: String,
}
