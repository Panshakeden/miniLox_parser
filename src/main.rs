fn main() {
    println!("Hello, world!");

// program    → statement* EOF ;
// statement  → exprStmt | printStmt ;
// exprStmt   → expression ";" ;
// printStmt  → "print" expression ";" ;

    #[derive(Debug)]
    enum Token{
        Print,
        Number(f64),
        Semicolon,
        EOF,
    }

    #[derive(Debug)]
    enum Expr{
        Literals(f64),
    }

    #[derive(Debug)]
    enum stmt{
        Print(Expr),
    }

    struct Parser{
        tokens: Vec<Token>,
        current:usize,
    }
}
