use nix::sys::wait::waitpid;
use nix::unistd::{dup2, fork, pipe, ForkResult};
use shell_commands::commands;
use std::io::{self, stderr, stdout, Write};
use std::path::Path;
use std::process::exit;
use std::{env, process};

mod shell_commands;

struct Shell {
    shell_prefix: String,
}

#[allow(unused_assignments)]
impl Shell {
    fn new(prefix: String) -> Self {
        return Shell {
            shell_prefix: prefix,
        };
    }

    fn command_executor(&self, cmd: String) {
        let input_cmd = cmd.split_ascii_whitespace().collect::<Vec<_>>();
        let cmd = input_cmd[0];
        let arg = &input_cmd[1..].join(" ");

        type Func = fn(&String) -> i32;

        let mut exec_func: Func = commands::echo_callback; // dummy value

        match cmd {
            "echo" => exec_func = commands::echo_callback,
            "cat" => exec_func = commands::cat_callback,
            "ls" => exec_func = commands::ls_callback,
            "grep" => {
                println!(
                    "{:#?}",
                    process::Command::new("grep").args(&input_cmd[1..]).output().unwrap()
                );
                return;
            }
            &_ => return,
        }

        exec_func(&arg);
    }

    fn parse(&self, user_cmd: String) {
        let mut user_cmd = user_cmd;

        if user_cmd.contains(">&") {
            let redir_index = user_cmd.find(">&").unwrap(); //10
                                                            // toFix: indexing here
            let from_fd = &user_cmd[redir_index - 1..redir_index]; //1
            match from_fd {
                "1" => {
                    // will execute if 1>&2
                    dup2(2, 1).unwrap();
                }
                "2" => {
                    // will execute if 2>&1
                    dup2(1, 2).unwrap();
                }
                &_ => {
                    panic!("invalid fd for redirect");
                }
            }
            // execute_cmd.remove(execute_cmd[redir_index-1..redir_index+3]);
            user_cmd.replace_range(redir_index - 1..redir_index + 3, "");

            if user_cmd.contains(">&") {
                writeln!(stderr(), "[ERROR] Invalid use of redirection!!").unwrap();
                exit(-1);
            }

            // cat 1234  >file
            // execute_cmd = command.clone();
        }
        // user_cmd = "cat 1234  >file"

        if user_cmd.contains(">") {
            if user_cmd.contains(">>") {
                // open file in append mode
            } else {
                // open file in truncate mode
            }
            // open file
            // call dup2 on file descriptors
            // replace file redirection with ""
            // user_cmd = "cat 1234  "
        }
        //pipes
        let cmd = user_cmd.split(" | ");
        // let cmd = user_cmd.split(" ");

        let redir_pipe = pipe().unwrap();

        for c in cmd {
            let execute_cmd = c.to_string();
            self.command_executor(execute_cmd);
        }

        // exit(0);
    }

    fn run(&self) {
        loop {
            let mut cmd = String::new();
            write!(stdout(), "{}> ", self.shell_prefix).unwrap();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cmd).unwrap();

            if cmd.len() < 2 {
                //newline
                continue;
            }

            // let ret_value = 0;
            if cmd.trim() == "exit" {
                break;
            }
            self.parse(cmd);

            // match unsafe { fork() } {
            //     Ok(ForkResult::Child) => {
            //     }
            //     Ok(ForkResult::Parent { child }) => {
            //         // waitpid(child, None).unwrap();
            //     }
            //     Err(_) => {}
            // }
        }
    }
}

fn main() {
    let playground_path = Path::new("playground/");
    if env::set_current_dir(playground_path).is_ok() {
        writeln!(io::stdout(), "Successfully CDed into playground dir").unwrap();
    }

    let my_shell = Shell::new(String::from("my-shell"));
    my_shell.run();
}
