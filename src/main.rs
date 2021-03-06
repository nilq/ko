extern crate colored;

mod ko;
use ko::source::*;
use ko::lexer::*;
use ko::parser::*;

fn main() {
  let file    = "foo.ko";
  let content = r#"
window:
  width = 100
  height = 100

  title = "
    a window
  "
  "#;

  let source = Source::from(file, content.lines().map(|x| x.into()).collect::<Vec<String>>());
  let lexer  = Lexer::default(content.chars().collect(), &source);

  let mut tokens = Vec::new();

  for token_result in lexer {
    if let Ok(token) = token_result {
      tokens.push(token)
    } else {
      return
    }
  }

  let mut parser = Parser::new(tokens, &source);

  match parser.parse() {
    Ok(ref ast) => {
      println!("{:#?}", ast)
    },

    _ => return,
  }
}