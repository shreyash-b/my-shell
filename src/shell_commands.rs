#[allow(dead_code)]
pub mod commands {
    use std::fs;
    pub fn echo_callback(arg: String) {
        let cmd_name = "ECHO";

        println!("{}: {}", cmd_name, arg);
    }

    pub fn cat_callback(arg: String) {
        let cmd_name = "CAT";
        let file_contents = match fs::read_to_string(&arg) {
            Ok(v) => v,
            Err(_) => {
                println!("{}: [ERROR] Unable to read {}", cmd_name, arg);
                return;
            }
        };
        println!("{}: {}", cmd_name, file_contents);
    }
}
