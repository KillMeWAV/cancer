// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// This file is part of cancer.
//
// cancer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cancer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cancer.  If not, see <http://www.gnu.org/licenses/>.

use std::ops::Deref;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Hints {
	inner:   HashMap<String, Hint, BuildHasherDefault<FnvHasher>>,
	current: usize,
	table:   Vec<char>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Hint {
	pub name:     String,
	pub position: ((u32, u32), (u32, u32)),
	pub content:  String,
}

impl Hints {
	pub fn new(table: Vec<char>, length: usize) -> Self {
		Hints {
			inner:   Default::default(),
			current: (length / table.len()) * table.len(),
			table:   table,
		}
	}

	pub fn put<T: Into<String>>(&mut self, position: ((u32, u32), (u32, u32)), content: T) -> &Hint {
		let name      = self.name_for(self.current);
		self.current += 1;

		self.inner.entry(name.clone()).or_insert(Hint {
			name:     name.clone(),
			position: position,
			content:  content.into(),
		})
	}

	pub fn into_inner(self) -> HashMap<String, Hint, BuildHasherDefault<FnvHasher>> {
		self.inner
	}

	fn name_for(&self, index: usize) -> String {
		let mut result = String::new();
		let mut index  = index;

		while index >= self.table.len() {
			result.push(self.table[index % self.table.len()]);
			index /= self.table.len();
		}

		result.push(self.table[index]);
		result
	}
}

impl Deref for Hints {
	type Target = HashMap<String, Hint, BuildHasherDefault<FnvHasher>>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}