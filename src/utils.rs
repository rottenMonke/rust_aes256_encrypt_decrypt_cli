use std::env;
use std::path::PathBuf;

pub struct Arguments {
    pub should_decrypt: bool,
    pub should_print: bool,
    pub path_to_file: String,
    pub path_to_file_to_write_to: Option<String>,
    pub secret: String,
}

pub fn get_arguments() -> Arguments {
    let args: Vec<String> = env::args().collect();

    let mut should_decrypt = false;
    let mut should_print = false;
    let mut non_flag_arguments: Vec<&String> = Vec::new();

    let path_to_file: String;
    let mut path_to_file_to_write_to: Option<String> = None;
    let secret: String;

    for i in 1..args.len() {
        if !should_decrypt && args[i].eq("--d") {
            should_decrypt = true;
        } else if !should_print && args[i].eq("--p") {
            should_print = true;
        } else {
            non_flag_arguments.push(&args[i]);
        }
    }

    if non_flag_arguments.len() < 2 {
        panic!("Invalid number of arguments");
    } else if non_flag_arguments.len() == 3 {
        path_to_file = non_flag_arguments[0].clone();
        path_to_file_to_write_to = Some(non_flag_arguments[1].clone());
        secret = non_flag_arguments[2].clone();
    } else {
        path_to_file = non_flag_arguments[0].clone();
        secret = non_flag_arguments[1].clone();
    }

    Arguments {
        should_decrypt,
        should_print,
        path_to_file,
        path_to_file_to_write_to,
        secret,
    }
}

pub fn make_path_to_file(user_path: String) -> PathBuf {
    let path = PathBuf::from(user_path);
    if path.is_absolute() {
        return path;
    }
    let current_dir = env::current_dir().unwrap();
    current_dir.as_path().join(path)
}
