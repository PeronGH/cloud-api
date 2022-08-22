use std::{
    ffi::OsStr,
    io,
    process::{Command, Output},
};

pub fn exec_py(code: &str, args: Vec<&str>) -> io::Result<Output> {
    Command::new("python3")
        .arg("-c")
        .arg(code)
        .args(args)
        .output()
}

#[cfg(test)]
mod test {
    use super::exec_py;

    #[test]
    fn test_exec_py() {
        println!("{:?}", exec_py("print('hello')", vec![]).unwrap().stdout)
    }
}
