use nix::sys::wait::waitpid;
use nix::unistd::{dup2, fork, ForkResult};
use shell_commands::commands;
use std::cell::RefCell;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

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

    fn command_executor(
        &self,
        in_stream: &String,
        out_stream: &mut String,
        err_string: &mut String,
        cmd: String,
    ) -> i32 {
        let input_cmd = cmd.split_ascii_whitespace().collect::<Vec<_>>();
        let cmd = input_cmd[0];
        let arg = input_cmd[1..].join(" ");

        let mut ret_val: i32 = 0;

        type Func = fn(&String, &mut String, &mut String, &String) -> i32;

        let mut exec_func: Func = commands::echo_callback;
        // exec_func = commands::echo_callback;

        match cmd {
            "echo" => exec_func = commands::echo_callback,
            "cat" => exec_func = commands::cat_callback,
            "ls" => exec_func = commands::ls_callback,
            "exit" => ret_val = 11,
            &_ => return 12,
        }

        if ret_val != 11 {
            exec_func(in_stream, out_stream, err_string, &arg);
        }

        return ret_val;
    }

    fn parse(&self, user_cmd: String) -> i32 {
        //        let in_stream = RefCell::new(String::new());
        let out_stream = RefCell::new(String::new()); // stdout stream
        let err_stream = RefCell::new(String::new()); // stderr stream

        let mut in_param: String;
        let mut out_param = out_stream.borrow_mut();
        let mut err_param = err_stream.borrow_mut();

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
                    dup2(1,2).unwrap();
                }
                &_ => {
                    panic!("invalid fd for redirect");
                }
            }
            // execute_cmd.remove(execute_cmd[redir_index-1..redir_index+3]);
            user_cmd.replace_range(redir_index - 1..redir_index + 3, ""); 
            // cat 1234  >file
            // execute_cmd = command.clone();
            
        }
        // user_cmd = "cat 1234  >file"
        
        if user_cmd.contains(">"){
            if user_cmd.contains(">>"){
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

        for c in cmd {

            let execute_cmd = c.to_string();

            in_param = out_param.clone();
            out_param.clear();
            err_param.clear();

            self.command_executor(&in_param, &mut out_param, &mut err_param, execute_cmd);
        }

        exit(0);
    }


    fn run(&self) {
        loop {
            let mut cmd = String::new();
            print!("{}> ", self.shell_prefix);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cmd).unwrap();
            // let ret_value = 0;
            if cmd.trim() == "exit"{
                break;
            }
            match unsafe{fork()}{
                Ok(ForkResult::Child)=>{
                    self.parse(cmd);
                }
                Ok(ForkResult::Parent { child })=>{
                    waitpid(child, None).unwrap();
                },
                Err(_)=>{}
            }
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
