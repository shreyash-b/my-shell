mod shell_commands;
use shell_commands::commands;

fn main(){
    let filename = "/home/shreyash/testfile";
    commands::cat_callback(String::from(filename));
}