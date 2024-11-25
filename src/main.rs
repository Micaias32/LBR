use lbr_compiler::lexer::Tokenizer;

fn main() {
    let tokenizer = Tokenizer::new("".to_string());
    let a = tokenizer.tokens();
    println!("{:?}", a);
}
