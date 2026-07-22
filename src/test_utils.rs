use std::env;
use std::io::Write;
use std::path::PathBuf;

pub fn get_big_py_file(lines: Option<usize>, file_name: Option<&str>) -> String {
    let lines = lines.unwrap_or(10000);
    let path = file_name.map_or_else(
        || env::temp_dir().join(format!("xonsh-rd-parser-test-file-{}.py", lines)),
        |file_name| {
            let path = PathBuf::from(file_name);
            if path.is_absolute() {
                path
            } else {
                env::temp_dir().join(path)
            }
        },
    );

    if !path.exists() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        let mut file = std::fs::File::create(&path).unwrap();
        for idx in 0..lines {
            writeln!(file, r#"x_{idx} = {idx} + 1"#).unwrap();
            writeln!(file, r#"print(x_{idx})"#).unwrap();
            writeln!(file, r#"assert x_{idx} == {idx} + 1"#).unwrap();
        }
    }
    path.to_str().unwrap().to_string()
}
