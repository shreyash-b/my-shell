#[allow(dead_code, unused_variables)]
pub mod commands {
    use std::fs;
    use std::io::{stdout, stderr, Write};
    // use std::io::Write;

    pub fn echo_callback(
        in_stream: &String,
        out_stream: &mut String,
        err_stream: &mut String,
        arg: &String,
    ) -> i32 {
        // let mut write_str = arg.clone();
        write!(stdout(), "{}", in_stream).expect("Unable to write");
        if arg.len() != 0 {
            writeln!(stdout(), "{}", arg).expect("Unable to write");
            // write_str = in_stream.clone();
        }
        return 0;
    }

    pub fn cat_callback(
        in_stream: &String,
        out_stream: &mut String,
        err_stream: &mut String,
        arg: &String,
    ) -> i32 {
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

    pub fn ls_callback(
        in_stream: &String,
        out_stream: &mut String,
        err_stream: &mut String,
        arg: &String,
    ) -> i32 {
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            writeln!(stdout(), "{}", path.unwrap().path().display()).unwrap();
        }

        return 0;
    }
}
