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

use secstr::SecStr;

#[derive(Debug, Clone)]
pub struct Vault {
    owner: String,
    entries: HashMap<String, Entry>
}

impl Vault {

    pub fn new(owner: String) -> Vault {
        Vault {
            owner: owner,
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, name: String, password: SecStr) -> Entry {
        let entry = Entry::new(name.clone(), password);
        self.entries.insert(name, entry.clone());
        entry
    }

}

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    password: SecStr,
    pub comment: Option<String>,
    pub user: Option<String>,
    pub uri: Option<String>,
    pub tags: HashSet<String>,
    created: DateTime<Utc>,
    last_modified: DateTime<Utc>
}

impl Entry {

    fn new(name: String, password: SecStr) -> Entry {
        let created = Utc::now();
        return Entry {
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

}
