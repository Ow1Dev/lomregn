mod lexer;

fn main() {
    for v in lexer::new("42 + 3 * 3 / 2") {
        print!("{}", v);
    }

    print!("\n");
}
