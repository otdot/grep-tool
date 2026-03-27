pub fn find_matches(pattern: &str, line: &String, mut writer: impl std::io::Write) {
	let line = line.trim();
	if line.contains(pattern) {
		let _ = writeln!(writer, "{}", line);
	}
}


#[test]
pub fn check_found_matches() {
	let line = String::from("tests are great\n");
	let mut result = Vec::new();
	crate::find_matches("test", &line, &mut result);
    assert_eq!(result, line.as_bytes())
}