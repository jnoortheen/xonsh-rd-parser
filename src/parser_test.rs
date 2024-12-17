#[cfg(test)]
mod tests {
    use crate::annotate_src::CodeFrame;
    use ruff_python_parser::{parse_unchecked, Mode};
    use ruff_source_file::{LineIndex, SourceCode};
    use ruff_text_size::{Ranged, TextLen};
    use std::fmt::Write;

    fn test_valid_source(source: &str) -> String {
        let parsed = parse_unchecked(source, Mode::Module);

        println!("Tokens: ");
        for token in parsed.tokens() {
            println!("{token:?} {}", &source[token.range()]);
        }
        println!("length: {:?}", source.text_len());

        if !parsed.is_valid() {
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

        assert!(parsed.is_valid());
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
