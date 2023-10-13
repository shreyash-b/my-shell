pub mod commands {
    use std::fs;
    use std::io::{stdout, stderr, Write};

    pub fn echo_callback(arg: &String) -> i32 {
        writeln!(stdout(), "{}", arg).expect("Unable to write");
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

    pub fn ls_callback(_arg: &String) -> i32 {
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            writeln!(stdout(), "{}", path.unwrap().path().display()).unwrap();
        }

        return 0;
    }
}
