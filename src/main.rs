mod submodule1;
mod submodule2;

fn main() {
    submodule1::print_chars();
    println!("------------------");
    submodule2::print_chars();
}
