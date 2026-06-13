use std::env;
use std::io::Write;

pub fn get_big_py_file(lines: Option<usize>) -> String {
    let lines = lines.unwrap_or(10000);
    let dir = env::temp_dir();
    let path = dir.join(format!("xonsh-rd-parser-test-file-{}.py", lines));

    if !path.exists() {
        let mut file = std::fs::File::create(&path).unwrap();
        for idx in 0..lines {
            writeln!(file, r#"x_{idx} = {idx} + 1"#).unwrap();
            writeln!(file, r#"print(x_{idx})"#).unwrap();
            writeln!(file, r#"assert x_{idx} == {idx} + 1"#).unwrap();
        }
    }
    path.to_str().unwrap().to_string()
}
