#[allow(dead_code, unused_variables)]
pub mod commands {
    use std::fs;
    pub fn echo_callback(arg: &str) -> i32{

        println!("{}", arg);
        return 0;
    }

    pub fn cat_callback(arg: &str) -> i32 {
        let file_contents = match fs::read_to_string(&arg) {
            Ok(v) => v,
            Err(_) => {
                println!("[ERROR] Unable to read {}", arg);
                return -1;
            }
        };
        eprintln!("{}", file_contents);
        return 0;
    }
}
