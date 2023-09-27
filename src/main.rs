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

    fn parse(&self, cmd: &String) -> i32 {
        let input_cmd = cmd.split_ascii_whitespace().collect::<Vec<_>>();
        let cmd = input_cmd[0];
        let arg = &input_cmd[1..].join(" ");
        let mut ret_val: i32 = 0;

        let mut op_out = String::new(); // stdout stream
        let mut op_err = String::new(); // stderr stream


        match cmd {
            "echo" => ret_val = commands::echo_callback(&mut op_out, &mut op_err, arg),
            "cat" => ret_val = commands::cat_callback(&mut op_out, &mut op_err, arg),
            "ls" => ret_val = commands::ls_callback(&mut op_out, &mut op_err, arg),
            "exit" => ret_val = 11,
            &_ => {}
        }

        write!(io::stdout(), "{}", op_out).unwrap();
        write!(io::stderr(), "{}", op_err).unwrap();

        return ret_val;
        
    }

    fn run(&self) {
        
        loop {
            let mut cmd = String::new();
            print!("{}> ", self.shell_prefix);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cmd).unwrap();

            let ret_val = self.parse(&cmd);
            if ret_val == 11{
                break;
            }
        }
    }
}

fn main() {
    let my_shell = Shell::new(String::from("my-shell"));
    my_shell.run();
}
