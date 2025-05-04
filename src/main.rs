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

    impl Parser{
        fn new (tokens:Vec<Token>) -> Self{
            Self{tokens,current:0}
        }

        fn parse(&mut self) -> Vec<Stmt>{
            let mut statements = Vec::new();

            while !self.at_end(){
                statements.push(self.statement())
            }

            statements
        }

        fn statement(&mut self)-> Stmt{
            if self.match_token(&[Token::Print]){
                let expr= self.expression();
                self.consume(Token::Semicolon, "; expected");

                Stmt::Print(expr)
            }else{
                panic!("statement unknown");
            }

        }

        fn expression(&mut self)->Expr{
            if let Token::Number(value) = self.move_advance_token(){
                Expr::Literals(value)
            }else{
                panic!("A number expected");
            }
        }
             
        fn token_check(&self, kind:&Token) -> bool{
            if self.at_end(){
                return false;
            }
            &self.tokens[self.current] ==kind
        }
        
        fn move_advance_token(&mut self) -> Token{
            let token =self.tokens[self.current].clone();
            self.current += 1;
            token
        }

        fn consume(&mut self, kind:Token, msg:&str){
             if self.token_check(&kind){
                self.move_advance_token();
             }else{
                panic!("{}",msg);
             }
        }

        fn at_end(&self)-> bool{
            self.tokens[self.current] == Token::EOF
    
        }

        fn match_token(&mut self, kinds: &[Token])->bool{
               for kind in kinds{
                if self.token_check(kind){
                    self.move_advance_token();
                    return true;
                }
               }
               false
        }
    }

  
}


 






