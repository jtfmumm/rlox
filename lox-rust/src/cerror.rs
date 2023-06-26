pub fn cerror(line_n: u32, msg: &str) -> String {
	report(line_n, "", msg);
	msg.to_string()
}

pub fn report(line_n: u32, location: &str, msg: &str) {
	eprintln!("[line: {:?}] Error {:?}: {:?}", line_n, location, msg);
}
