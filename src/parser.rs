use py_ast::ast_module::AstModule;
use py_ast::to_ast::ToAst;
use pyo3::exceptions::PySyntaxError;
use pyo3::{PyObject, PyResult, Python};
use ruff_source_file::{LineIndex, SourceCode};

pub fn parse_str<'py>(
    py: Python<'py>,
    src: &'py str,
    filename: Option<&'py str>,
) -> PyResult<PyObject> {
    let parsed = ruff_python_parser::parse_module(src);
    match parsed {
        Ok(parsed) => {
            let line_index = LineIndex::from_source_text(src);
            let source_code = SourceCode::new(src, &line_index);
            let tree = parsed.into_syntax();
            let module = AstModule::new(py, &source_code)?;
            tree.to_ast(&module)
        }
        Err(err) => {
            let filename = filename.unwrap_or("<string>");
            let msg = crate::annotate_src::to_exc_msg(src, filename, &err);
            let err = PySyntaxError::new_err(msg);
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::annotate_src::CodeFrame;
    use ruff_python_parser::{parse_unchecked, Mode};
    use ruff_source_file::{LineIndex, SourceCode};
    use ruff_text_size::{Ranged, TextLen};
    use std::fmt::Write;

    fn test_valid_source<'a>(source: &'a str) -> String {
        let parsed = parse_unchecked(&source, Mode::Module);

        println!("Tokens: ");
        for token in parsed.tokens() {
            println!("{token:?} {}", source[token.range()].to_string());
        }
        println!("length: {:?}", source.text_len());

        if !parsed.is_valid() {
            let line_index = LineIndex::from_source_text(&source);
            let source_code = SourceCode::new(&source, &line_index);

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
        let source = r#"!(ls @(None or "."))"#;
        // let source = r#"![a b c2]"#;
        // let source = r#"print('hello')"#;
        let _output = test_valid_source(source);
        // insta::assert_snapshot!(output);
    }
}
