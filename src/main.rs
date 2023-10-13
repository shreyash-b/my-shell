use nix::sys::wait::waitpid;
use nix::unistd::{close, dup2, fork, pipe, ForkResult};
use shell_commands::commands;
use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::{self, stderr, stdout, Write};
use std::os::fd::AsRawFd;
use std::path::Path;
use std::process::exit;
use std::{env, process};

mod shell_commands;

struct CustomShell {
    shell_prefix: String,
}
impl CustomShell {
    fn new(prefix: String) -> Self {                      //Shell prompter
        return CustomShell {
            shell_prefix: prefix,
        };
    }

    fn command_executor(&self, cmd: String) {             //Command Matching and calling
        let input_cmd = cmd.split_ascii_whitespace().collect::<Vec<_>>();
        let cmd = input_cmd[0];
        let arg = &input_cmd[1..];
        type Func = fn(&String) -> i32;
        let mut _exec_func: Func;

        match cmd {
            "echo" => _exec_func = commands::echo_callback,
            "cat" => _exec_func = commands::cat_callback,
            "ls" => _exec_func = commands::ls_callback,
            &_ => {
                let child_command = process::Command::new(cmd)
                    .args(arg)
                    .spawn();

                match child_command {
                    Ok(mut child_command) => { let _ = child_command.wait(); },
                    Err(e) => eprintln!("{}", e),
                };
                return;
            }
        }

        _exec_func(&arg.join(" "));
    }


    fn error_logging(user_cmd: String) -> String {                        //Function for >&                  
        let mut user_cmd = user_cmd;
        let redir_index = user_cmd.find(">&").unwrap();
        let from_fd = &user_cmd[redir_index - 1..redir_index];

        match from_fd {
            "1" => {
                dup2(2, 1).unwrap();                    // will execute if 1>&2
            }
            "2" => {
                dup2(1, 2).unwrap();                    // will execute if 2>&1
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
        return user_cmd;
        // cat 1234  >file
        // execute_cmd = command.clone();
    }

    fn file_output(user_cmd: String) -> String {                   // Function for >> & >
        let mut user_cmd = user_cmd;
        let mut file_ops ;
        let file_path: String;
        file_ops = OpenOptions::new().create(true).write(true).to_owned();
        let mut _split: Vec<&str> = Vec::new();

        if user_cmd.contains(">>"){
            _split = user_cmd.split(">>").collect();
            file_ops.append(true);
        } else {
            _split = user_cmd.split(">").collect();
            file_ops.truncate(true);
            
        }

        (user_cmd, file_path) = (_split[0].trim().to_string(), _split[1].trim().to_string());
        let file = file_ops.open(file_path).unwrap();
        dup2(file.as_raw_fd(), 1).unwrap();
        return user_cmd;
    }

    fn parse(&self, user_cmd: String) {
        let mut user_cmd = user_cmd;
        //let file_path: String;
        //let mut file_ops ;

        if user_cmd.contains(">&") {
            user_cmd = Self::error_logging(user_cmd);
        }
        // user_cmd = "cat file1 > file2"

        if user_cmd.contains(">"){
            user_cmd = Self::file_output(user_cmd);
        }

        // open file
        // call dup2 on file descriptors
        // replace file redirection with ""
        // user_cmd = "cat 1234 | > file"
        //pipes
        let cmd = user_cmd.split(" | ").collect::<Vec<&str>>();
        let cmd_num = cmd.len();
        let mut redir_pipes = VecDeque::<(i32, i32)>::new();

        for i in 0..cmd_num {
            let stdout_redir = i > 0;                   // stdout from prev command false
            let stdin_redir = i < cmd_num - 1;          // stdin to next command true

            if stdin_redir {
                redir_pipes.push_back(pipe().unwrap().to_owned());  // pipe for passing stdout of current cmd to next cmd
            }

            match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    if stdout_redir {
                        dup2(redir_pipes.front().unwrap().0, 0).unwrap(); // getting stdin
                    }

                    if stdin_redir {
                        dup2(redir_pipes.back().unwrap().1, 1).unwrap();
                    }

                    self.command_executor(cmd[i].to_string());
                    exit(0);
                }

                Ok(ForkResult::Parent { child }) => {
                    if stdin_redir {
                        // true
                        close(redir_pipes.back().unwrap().1).unwrap();
                    }

                    if stdout_redir {
                        // false
                        close(redir_pipes.front().unwrap().0).unwrap();
                        redir_pipes.pop_front();
                    }

                    waitpid(child, None).unwrap();
                }

                Err(_) => {}
            }
        }
        exit(0);
    }

    fn run(&self) {
        loop {
            let mut user_cmd = String::new();
            write!(stdout(), "{}> ", self.shell_prefix).unwrap();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut user_cmd).unwrap();

            if user_cmd.len() == 0{
                break;                                      // EOF
            } else if user_cmd.len() < 2 {
                continue;                                   // newline
            } 

            let mut parts = user_cmd.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                },
                "exit" => break,
                &_ => {
                    self.parse(user_cmd);
                }
            }
        }
    }
}

fn main() {
    let program_dir = Path::new("playground/");             //This is the starting directing of shell
    if env::set_current_dir(program_dir).is_ok() {
        writeln!(io::stdout(), "Current directory set as Playground directory").unwrap();
    }

    let my_shell = CustomShell::new(String::from("my-shell"));
    my_shell.run();
}
