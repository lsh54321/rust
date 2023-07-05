pub fn print_chars() {
    for c in b'A'..=b'z' {
        println!("{}", c as char);
    }
}