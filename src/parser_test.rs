#[cfg(test)]
mod tests {
    use crate::annotate_src::CodeFrame;
    use ruff_python_parser::{Mode, ParseOptions, parse_unchecked};
    use ruff_source_file::{LineIndex, SourceCode};
    use ruff_text_size::{Ranged, TextLen};
    use std::fmt::Write;

    fn test_valid_source(source: &str) -> String {
        let parsed = parse_unchecked(source, ParseOptions::from(Mode::Module));

        println!("Tokens: ");
        for token in parsed.tokens() {
            println!("{token:?} {}", &source[token.range()]);
        }
        println!("length: {:?}", source.text_len());

        if !parsed.has_no_syntax_errors() {
            let line_index = LineIndex::from_source_text(source);
            let source_code = SourceCode::new(source, &line_index);

            let mut message = "Expected no syntax errors for a valid \
            program but the parser generated the following errors:\n"
                .to_string();

            for error in parsed.errors() {
                let frame = CodeFrame::new(&source_code, error);
                writeln!(&mut message, "{frame}\n").unwrap();
                writeln!(&mut message,).unwrap();
            }

            panic!("{source:?}: {message}");
        }

        let mut output = String::new();
        writeln!(&mut output, "## AST").unwrap();
        writeln!(&mut output, "\n```\n{:#?}\n```", parsed.syntax()).unwrap();

        assert!(parsed.has_no_syntax_errors());
        output
    }

    #[test]
    fn test_tmp() {
        // let source = r#"![a@$(echo 1 2)b a]"#;
        let source = r#"!(ls !micro)"#;
        // let source = r#"![a b c2]"#;
        // let source = r#"print('hello')"#;
        let _output = test_valid_source(source);
        // insta::assert_snapshot!(output);
    }
}

#[cfg(test)]
mod py_tests {
    use crate::parser::PyParser;
    use crate::test_utils::get_big_py_file;
    use pyo3::prelude::*;

    #[test]
    fn test_py_parser() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let file_name = get_big_py_file();
            PyParser::parse_file(py, file_name.as_str()).unwrap();
        })
    }
}
