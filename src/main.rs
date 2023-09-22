use shell_commands::commands;
use std::io::{self, Write};

mod shell_commands;

struct Shell {
    shell_prefix: String,
}

impl Shell {
    fn new(prefix: String) -> Self {
        return Shell {
            shell_prefix: prefix,
        };
    }

    fn run(&self) {
        let mut cmd = String::new();

        print!("{}> ", self.shell_prefix);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut cmd).unwrap();


    }
}

fn main() {
    let my_shell = Shell::new(String::from("my-shell"));
    my_shell.run();
}
