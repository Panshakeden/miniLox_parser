fn main() {
    println!("Hello, world!");

// program    → statement* EOF ;
// statement  → exprStmt | printStmt ;
// exprStmt   → expression ";" ;
// printStmt  → "print" expression ";" ;

    #[derive(Debug,Clone,PartialEq)]
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
    enum Stmt{
        Print(Expr),
    }

    struct Parser{
        tokens: Vec<Token>,
        current:usize,
    }


    fn tokenizer(input: &str) -> Vec<Token>{
       let mut tokens = Vec::new();
       let mut chars= input.chars().peekable();

       while let Some(&c) = chars.peek(){

        match c {
            ' ' => {
                chars.next();
            }

            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            
            '0'..='9' => {
                let num:String= chars.by_ref().take_while(|c| c.is_ascii_digit()).collect();
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            'p' =>{
                let word:String= chars.by_ref().take_while(|c| c.is_alphabetic()).collect();
                if word == "print"{
                    tokens.push(Token::Print);
                }
            }

            _ => {
                chars.next();
            }
           
        }
       }

       tokens.push(Token::EOF);
       tokens
    }
}


 






