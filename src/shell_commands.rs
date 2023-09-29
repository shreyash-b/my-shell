
#[allow(dead_code, unused_variables)]
pub mod commands {
    use std::fmt::Write;
    use std::fs;
    // use std::io::Write;


    pub fn echo_callback(in_stream: &mut String, out_stream: &mut String, err_stream: &mut String, arg: &String) -> i32 {
        // let mut write_str = arg.clone();
        write!(out_stream, "{}", in_stream).expect("Unable to write");
        if arg.len() != 0 {
            writeln!(out_stream, "{}", arg).expect("Unable to write");
            // write_str = in_stream.clone();
        } 
        return 0;
    }

    pub fn cat_callback(in_stream: &mut String, out_stream: &mut String, err_stream: &mut String, arg: &String) -> i32 {
        let file_contents = match fs::read_to_string(&arg) {
            Ok(v) => v,
            Err(_) => {
                writeln!(err_stream, "[ERROR] Unable to read {}", arg).unwrap();
                return -1;
            }
        };
        writeln!(out_stream, "{}", file_contents).unwrap();
        return 0;
    }

    pub fn ls_callback(in_stream: &mut String, out_stream: &mut String, err_stream: &mut String, arg: &String) -> i32 {
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            writeln!(out_stream, "{}", path.unwrap().path().display()).unwrap();
        }

        return 0;
    }
}
