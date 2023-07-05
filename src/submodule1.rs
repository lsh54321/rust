pub fn print_chars() {
    for c in b'a'..=b'Z' {
        println!("{}", c as char);
    }
}