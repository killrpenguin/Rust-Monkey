#[allow(dead_code, unused)]
pub mod monkey_repl {
    use crate::lexer::monkey_lexer::*;
    use crate::token::tokens::*;
    use std::io;
    use std::io::Write;

    pub fn start_repl() -> std::io::Result<()> {
        'outer: loop {
            let mut input = String::new();
            print!("Monkey Do! >> ");
            let _ = io::stdout().flush();
            let stdin = io::stdin();
            stdin
                .read_line(&mut input)
                .expect("Error reading from stdin.");

            if input.contains("exit") {
                println!("Bye!");
                break 'outer;
            }
            let mut lexer = Lexer::new(&input);
            let _: Vec::<_> = lexer.input.chars().map(|letter| {
                if !letter.is_ascii_whitespace() {
                    println!("\t Char: {} Token: {}", letter, lexer.next_token());
                }
            }).collect();
        }
        Ok(())
    }
}
