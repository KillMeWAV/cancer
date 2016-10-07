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

#![allow(non_snake_case)]

use std::str;
use nom::{self, IResult, Needed, rest};

#[macro_use]
mod util;

mod c0;
pub use self::c0::shim as C0;

mod c1;
pub use self::c1::shim as C1;

mod csi;
pub use self::csi::shim as CSI;

mod sgr;
pub use self::sgr::shim as SGR;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Item<'a> {
	String(&'a str),

	C0(C0::T),
	C1(C1::T),
}

named!(pub parse<Item>,
	alt!(control | string));


fn string(i: &[u8]) -> IResult<&[u8], Item> {
	const WIDTH: [u8; 256] = [
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x1F
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x3F
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x5F
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x7F
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x9F
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xBF
		0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
		2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 0xDF
		3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
		4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
	];

	if i.is_empty() {
		return IResult::Incomplete(Needed::Size(1));
	}

	let mut length = 0;
	let mut input  = i;

	while control(input).is_err() {
		let w = WIDTH[input[0] as usize] as usize;

		if input.len() < w {
			return IResult::Incomplete(Needed::Size(w - input.len()));
		}

		length += w;
		input   = &input[w..];

		if input.is_empty() {
			break;
		}
	}

	if let Ok(string) = str::from_utf8(&i[..length]) {
		IResult::Done(&i[length..], Item::String(string))
	}
	else {
		IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(9001)))
	}
}

named!(control<Item>,
	alt!(
		map!(C1::parse, |c| Item::C1(c)) |
		map!(C0::parse, |c| Item::C0(c))));


