#[allow(dead_code, unused_variables)]
pub mod commands {
    use std::fs;
    use std::io::{self, stdout, stderr, Write};
    use std::env;
    use std::process;
    use std::path::Path;
    // use std::io::Write;

    pub fn echo_callback(arg: &String) -> i32 {
        writeln!(stdout(), "{}", arg).expect("Unable to write");
        return 0;
    }

    pub fn cd_callback(arg: &String) -> i32 {
        let root = Path::new(arg);
        if env::set_current_dir(root).is_ok() {
            writeln!(io::stdout(), "Successfully CDed into {arg} dir").unwrap();
            process::Command::new("pwd")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        }
        if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{}", e);
        }
        return 0;
    }

    pub fn cat_callback(arg: &String) -> i32 {
        let file_contents = match fs::read_to_string(&arg) {
            Ok(v) => v,
            Err(_) => {
                writeln!(stderr(), "[ERROR] Unable to read {}", arg).unwrap();
                return -1;
            }
        };
        writeln!(stdout(), "{}", file_contents).unwrap();
        return 0;
    }

    pub fn ls_callback(arg: &String) -> i32 {
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            writeln!(stdout(), "{}", path.unwrap().path().display()).unwrap();
        }

        return 0;
    }
}
