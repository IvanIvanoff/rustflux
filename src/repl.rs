extern crate rustyline;

use self::rustyline::Editor;
use self::rustyline::error::ReadlineError;
use interpreter;
use context::Context;
use errors::RustfluxError;

fn input(prompt: &str, editor: &mut Editor<()>) -> Result<String, ReadlineError> {
    let line = editor.readline(prompt);

    match line {
        Ok(text) => {
            editor.add_history_entry(&text);
            Ok(text)
        }
        Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
            Err(ReadlineError::Interrupted)
        }
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
            Err(ReadlineError::Eof)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            Err(err)
        }
    }
}

pub fn start() -> Result<(), RustfluxError> {
    let mut editor = Editor::<()>::new();

    let mut context = Context::new()?;

    loop {
        match input("> ", &mut editor) {
            Ok(input) => {
                interpreter::execute(&mut context, &input)?;
            }
            Err(_) => {
                return Err(RustfluxError::InputParse(String::from(
                    "Cannot parse the input",
                )))
            }
        }
    }
}
