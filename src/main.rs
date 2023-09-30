use shell_commands::commands;
use std::cell::RefCell;
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
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
            &_ => {return 12}
        }

        if ret_val != 11{
            exec_func(in_stream, out_stream, err_string, &arg);
        }

        return ret_val;
    }

    fn parse(&self, user_cmd: String) -> i32 {
//        let in_stream = RefCell::new(String::new());
        let out_stream = RefCell::new(String::new()); // stdout stream
        let err_stream = RefCell::new(String::new()); // stderr stream
        let mut ret_val = 0;


        let mut in_param: String;
        let mut out_param = out_stream.borrow_mut();
        let mut err_param = err_stream.borrow_mut();
        // out2.push(2);

        // add features for pipelining and output redirection
        // others: modify the parse function by utilizing splitted user_cmd
        // command_executor will execute the specified command and return the stdout and stderr in respective strings
        // for eg in case of "cat /some/file | echo > op.txt",
        // command_executor can only execute "cat /some/file" and "echo <args>" individually
        // pipelining and redirection is to be implemented in this function

        //pipelining
        let cmd = user_cmd.split(" | ");
        let mut to_append: bool = false;
        let mut std_out: bool = false; // Checks the standrad ouput
        let mut std_err: bool = false; // Checks the standrad error

        for c in cmd {
            // assuming that pipe redirects output as command line argument instead of stdin for simplicity

            let mut execute_cmd = c.to_string();
            let mut command = String::new();
            let mut file_path = String::from("");

            std_err = false;
            std_out = false;

            if execute_cmd.contains(">>") {
                let split: Vec<&str> = execute_cmd.split(">>").collect();
                (command, file_path) = (split[0].trim().to_string(), split[1].trim().to_string());
                std_out = true;
                to_append = true;
            } else if execute_cmd.contains(">") {
                if execute_cmd.contains("2>&1") {
                    execute_cmd = execute_cmd.replace("2>&1", "");
                    let split: Vec<&str> = execute_cmd.split(">").collect();
                    (command, file_path) =
                        (split[0].trim().to_string(), split[1].trim().to_string());
                    std_err = true;
                    std_out = true;
                } else if execute_cmd.contains("2>") {
                    let split: Vec<&str> = execute_cmd.split("2>").collect();
                    (command, file_path) =
                        (split[0].trim().to_string(), split[1].trim().to_string());
                    std_err = true;
                } else if execute_cmd.contains("1>&2") {
                    // out_param = err_stream.borrow_mut(); // not working
                } else {
                    let split: Vec<&str> = execute_cmd.split(">").collect();
                    (command, file_path) =
                        (split[0].trim().to_string(), split[1].trim().to_string());
                    std_out = true;
                }
            } else {
                command = execute_cmd;
            }

            // command.push_str(out_stream.as_str());
            // in_stream = out_param.clone();
            // in_param = out_param.clone();
            in_param = out_param.clone();
            out_param.clear();
            err_param.clear();
            ret_val = self.command_executor(&in_param, &mut out_param, &mut err_param, command);

            if !file_path.is_empty() {
                let path = Path::new(&file_path);

                let mut content = String::new();

                if std_out == true {
                    content += &out_param;
                }

                if std_err == true {
                    content += &err_param;
                }

                if to_append == false {
                    self.handle_redirect(&mut content, path)
                        .unwrap_or_else(|why| {
                            println!("! {:?}", why.kind());
                        });
                } else if to_append == true {
                    self.handle_redirect_append(&mut content, path)
                        .unwrap_or_else(|why| {
                            println!("! {:?}", why.kind());
                        });
                }
            }
        }

        if std_err == false {
            write!(io::stdout(), "{}", out_param).unwrap();
            write!(io::stderr(), "{}", err_param).unwrap();
        }

        return ret_val;
    }

    fn handle_redirect(&self, s: &str, path: &Path) -> io::Result<()> {
        // Function to handle redirects takes content and file path as arguments
        let mut f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        f.write_all(s.as_bytes())
    }

    fn handle_redirect_append(&self, s: &str, path: &Path) -> io::Result<()> {
        // Function to handle redirects takes content and file path as arguments
        let mut f = OpenOptions::new().create(true).append(true).open(path)?;

        f.write_all(s.as_bytes())
    }

    fn run(&self) {
        loop {
            let mut cmd = String::new();
            print!("{}> ", self.shell_prefix);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cmd).unwrap();

            let ret_val = self.parse(cmd);
            if ret_val == 11 {
                break;
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
