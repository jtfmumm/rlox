pub fn error(line_n: u32, msg: &str) {
	report(line_n, "", msg);
}

pub fn report(line_n: u32, location: &str, msg: &str) {
	eprintln!("[line: {:?}] Error {:?}: {:?}", line_n, location, msg);
}
