use std::{fs, io};

struct CmdDets<'a>{
    cmd_name: &'a str,
    callback_fn: Box<dyn Fn(String)>
}

fn main(){
    let echo_cmd = CmdDets{
        cmd_name: "echo",
        callback_fn: Box::new(|arg| {
            println!("ECHO: {}", arg);
        })
    };

    (echo_cmd.callback_fn)(String::from("value"));


    let cat_cmd = CmdDets{
        cmd_name: "cat",
        callback_fn: Box::new(|arg| {
            let file_content = fs::read_to_string(&arg)
                .expect("unable to read file");
            
            println!("CAT: {}", arg);
            println!("{}", file_content);
        })
    };

    (cat_cmd.callback_fn)(String::from("testfile.txt"));

}