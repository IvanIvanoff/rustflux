extern crate rustyline;

use self::rustyline::Editor;
use self::rustyline::error::ReadlineError;
use interpreter;
use context::Context;

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

pub fn start() {
    let mut editor = Editor::<()>::new();

    // TODO: Fix
    let mut context = Context::new().unwrap();

    loop {
        match input("> ", &mut editor) {
            Ok(input) => {
                interpreter::execute(&mut context, input);
            }
            Err(_err) => {
                break;
            }
        }
    }
}
