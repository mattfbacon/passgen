#![deny(
	absolute_paths_not_starting_with_crate,
	future_incompatible,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	non_ascii_idents,
	nonstandard_style,
	noop_method_call,
	pointer_structural_match,
	private_in_public,
	rust_2018_idioms,
	unused_qualifications
)]
#![warn(clippy::pedantic)]
#![allow(clippy::let_underscore_drop)]
#![forbid(unsafe_code)]

const NUM_CHARS: usize = 103;
const CHARACTERS: &[u8; NUM_CHARS] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()`~!@#$%^&&*()[]{}/=?+\\|-_,<.>'\"";

const PASSWORD_LENGTH: usize = 40;

fn main() {
	let mut random_buf = [0u8; PASSWORD_LENGTH * std::mem::size_of::<u64>()];
	getrandom::getrandom(&mut random_buf).unwrap();

	let mut password = [0u8; PASSWORD_LENGTH];
	for (password_char, random_bytes) in password.iter_mut().zip(random_buf.chunks(8)) {
		let random = u64::from_le_bytes(random_bytes.try_into().unwrap());
		// this is very very very slightly biased toward lower values. not enough to actually be an issue.
		let index = usize::try_from(random % (NUM_CHARS as u64)).unwrap();
		*password_char = CHARACTERS[index];
	}
	let password = std::str::from_utf8(&password).unwrap();
	println!("{password}");
}
