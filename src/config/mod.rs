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

use std::fs::File;
use std::io::Read;
use std::path::Path;

use toml;
use app_dirs::{AppInfo, AppDataType, app_root};

use error;

pub mod util;

pub mod environment;
pub use self::environment::Environment;

pub mod style;
pub use self::style::Style;

pub mod color;
pub use self::color::Color;

pub mod input;
pub use self::input::Input;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Config {
	environment: Environment,
	style:       Style,
	color:       Color,
	input:       Input,
}

impl Config {
	pub fn load<P: AsRef<Path>>(path: Option<P>) -> error::Result<Self> {
		let path = if let Some(path) = path {
			path.as_ref().into()
		}
		else {
			app_root(AppDataType::UserConfig, &AppInfo { name: "cancer", author: "meh." })
				?.join("config.toml")
		};

		let table = if let Ok(mut file) = File::open(path) {
			let mut content = String::new();
			let _ = file.read_to_string(&mut content);
			let mut parser = toml::Parser::new(&content);

			if let Some(table) = parser.parse() {
				table
			}
			else {
				error!("could not load configuration file");

				for error in &parser.errors {
					error!("syntax error: {}", error);
				}

				toml::Table::new()
			}
		}
		else {
			error!("could not load configuration file");
			toml::Table::new()
		};

		Ok(Config::from(table))
	}

	pub fn style(&self) -> &Style {
		&self.style
	}

	pub fn environment(&self) -> &Environment {
		&self.environment
	}

	pub fn color(&self) -> &Color {
		&self.color
	}

	pub fn input(&self) -> &Input {
		&self.input
	}
}

impl From<toml::Table> for Config {
	fn from(table: toml::Table) -> Config {
		let mut config = Config::default();

		if let Some(table) = table.get("environment").and_then(|v| v.as_table()) {
			config.environment.load(table);
		}

		if let Some(table) = table.get("style").and_then(|v| v.as_table()) {
			config.style.load(table);
		}

		if let Some(table) = table.get("color").and_then(|v| v.as_table()) {
			config.color.load(table);
		}

		if let Some(table) = table.get("input").and_then(|v| v.as_table()) {
			config.input.load(table);
		}

		config
	}
}
