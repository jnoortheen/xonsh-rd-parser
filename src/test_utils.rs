use std::env;
use std::io::Write;

pub fn get_big_py_file() -> String {
    let dir = env::temp_dir();

    let path = dir.join("xonsh-rd-parser-test-file.py");

    if !path.exists() {
        let mut file = std::fs::File::create(&path).unwrap();
        for idx in 0..10000 {
            writeln!(file, r#"x_{idx} = {idx} + 1"#).unwrap();
            writeln!(file, r#"print(x_{idx})"#).unwrap();
            writeln!(file, r#"assert x_{idx} == {idx} + 1"#).unwrap();
        }
    }
    path.to_str().unwrap().to_string()
}
