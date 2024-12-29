use lbr_compiler::ast::AST;

fn main() {
    let source = "\
val a = 1;
val b = 2;
escreval(a + b);
    ";

    let ast = AST::new(source);
    println!("AST: {ast:#?}");
}
