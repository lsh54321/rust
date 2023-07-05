pub fn print_chars() {
    for c in b'a'..=b'z' {
        println!("{}", c as char);
    }
    for c in b'A'..=b'Z' {
        println!("{}", c as char);
    }
}