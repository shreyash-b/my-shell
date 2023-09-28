use shell_commands::commands;
use std::io::{self, Write};
use std::fs::File;
use std::path::Path;

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

    fn command_executor(&self, out: &mut String, err: &mut String, cmd:String) -> i32{

        let input_cmd = cmd.split_ascii_whitespace().collect::<Vec<_>>();
        let cmd = input_cmd[0];
        let arg = input_cmd[1..].join(" ");

        let mut ret_val: i32 = 0;

        match cmd {
            "echo" => ret_val = commands::echo_callback(out, err, &arg),
            "cat" => ret_val = commands::cat_callback(out, err, &arg),
            "ls" => ret_val = commands::ls_callback(out, err, &arg),
            "exit" => ret_val = 11,
            &_ => {}
        }

        return ret_val;
    }

    fn parse(&self, user_cmd: String) -> i32 {

        let mut out = String::new(); // stdout stream
        let mut err = String::new(); // stderr stream
        let mut ret_val = 0;

        // add features for pipelining and output redirection
        // others: modify the parse function by utilizing splitted user_cmd
        // command_executor will execute the specified command and return the stdout and stderr in respective strings
        // for eg in case of "cat /some/file | echo > op.txt",
        // command_executor can only execute "cat /some/file" and "echo <args>" individually
        // pipelining and redirection is to be implemented in this function


        //pipelining
        let cmd = user_cmd.split(" | ");

        for c in cmd{
            // assuming that pipe redirects output as command line argument instead of stdin for simplicity

            let execute_cmd = c.to_string();
            let mut command = String::new();
            let mut file_path = String::from("");
            let mut err_out = false;

            
            if execute_cmd.contains("2>&1") {
                command = execute_cmd[..execute_cmd.find("2>&1").unwrap()-1].to_string();
                file_path = execute_cmd[execute_cmd.find("2>&1").unwrap()+5..execute_cmd.capacity()-2].to_string();
                err_out = true;
            }
            else if execute_cmd.contains("2>") {
                command = execute_cmd[..execute_cmd.find("2>").unwrap()-1].to_string();
                file_path = execute_cmd[execute_cmd.find("2>").unwrap()+3..execute_cmd.capacity()-2].to_string();
            }
            else {
                command = execute_cmd;
            }



            command.push_str(out.as_str());
            out.clear();
            err.clear();
            ret_val = self.command_executor(&mut out, &mut err, command);

            if !file_path.is_empty() {

                let path = Path::new(&file_path);
                let display = path.display();

                let mut file = match File::create(&path) {
                    Err(why) => panic!("couldn't create {}: {}", display, why),
                    Ok(file) => file,
                };

                match file.write_all(err.as_bytes()) {
                    Err(why) => panic!("couldn't write to {}: {}", display, why),
                    Ok(_) => println!("successfully wrote to {}", display),
                }

                if err_out == true {
                    match file.write_all(out.as_bytes()) {
                        Err(why) => panic!("couldn't write to {}: {}", display, why),
                        Ok(_) => println!("successfully wrote to {}", display),
                    }
                }
            }
        }

        write!(io::stdout(), "{}", out).unwrap();
        write!(io::stderr(), "{}", err).unwrap();

        return ret_val;
    }

    fn run(&self) {
        
        loop {
            let mut cmd = String::new();
            print!("{}> ", self.shell_prefix);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cmd).unwrap();

            let ret_val = self.parse(cmd);
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
