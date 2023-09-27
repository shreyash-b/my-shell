
#[allow(dead_code, unused_variables)]
pub mod commands {
    use std::fmt::Write;
    use std::fs;


    pub fn echo_callback(out: &mut String, err: &mut String, arg: &str) -> i32 {
        writeln!(out, "{}", arg);
        return 0;
    }

    pub fn cat_callback(out: &mut String, err: &mut String, arg: &str) -> i32 {
        let file_contents = match fs::read_to_string(&arg) {
            Ok(v) => v,
            Err(_) => {
                writeln!(err, "[ERROR] Unable to read {}", arg).unwrap();
                return -1;
            }
        };
        writeln!(out, "{}", file_contents).unwrap();
        return 0;
    }

    pub fn ls_callback(out: &mut String, err: &mut String, arg: &str) -> i32 {
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            writeln!(out, "{}", path.unwrap().path().display()).unwrap();
        }

        return 0;
    }
}
