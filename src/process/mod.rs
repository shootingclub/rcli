mod csv_convert;
mod gen_pass;

pub use self::csv_convert::process_csv;
pub use self::gen_pass::process_genpass;

pub const PASSWORD_UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const PASSWORD_LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const PASSWORD_NUMBER: &[u8] = b"123456789";
pub const PASSWORD_SYMBOL: &[u8] = b"!@#$_^&";
