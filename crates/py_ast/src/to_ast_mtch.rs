use std::vec;

use crate::ast_module::{AstModule, Callable};
use crate::to_ast::ToAst;
use pyo3::{IntoPy, PyObject};
use ruff_python_ast::*;

type PyResult = pyo3::PyResult<PyObject>;
impl ToAst for MatchCase {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("match_case")?.callk(
            [
                ("pattern", self.pattern.to_ast(module)?),
                ("guard", self.guard.to_ast(module)?),
                ("body", self.body.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for Pattern {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        match self {
            Pattern::MatchValue(node) => node.to_ast(module),
            Pattern::MatchSingleton(node) => node.to_ast(module),
            Pattern::MatchSequence(node) => node.to_ast(module),
            Pattern::MatchMapping(node) => node.to_ast(module),
            Pattern::MatchClass(node) => node.to_ast(module),
            Pattern::MatchStar(node) => node.to_ast(module),
            Pattern::MatchAs(node) => node.to_ast(module),
            Pattern::MatchOr(node) => node.to_ast(module),
        }
    }
}
impl ToAst for PatternMatchValue {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("MatchValue")?
            .call_with_loc(self.range, [("value", self.value.to_ast(module)?)])
    }
}
impl ToAst for PatternMatchSingleton {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let value = match self.value {
            Singleton::None => module.py.None(),
            Singleton::True => true.into_py(module.py),
            Singleton::False => false.into_py(module.py),
        };
        module
            .attr("MatchSingleton")?
            .call_with_loc(self.range, [("value", value)])
    }
}
impl ToAst for PatternMatchSequence {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("MatchSequence")?
            .call_with_loc(self.range, [("patterns", self.patterns.to_ast(module)?)])
    }
}
impl ToAst for PatternMatchMapping {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("MatchMapping")?.call_with_loc(
            self.range,
            [
                ("rest", self.rest.to_ast(module)?),
                ("keys", self.keys.to_ast(module)?),
                ("patterns", self.patterns.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for PatternMatchStar {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("MatchStar")?
            .call_with_loc(self.range, [("name", self.name.to_ast(module)?)])
    }
}
impl ToAst for PatternMatchAs {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module.attr("MatchAs")?.call_with_loc(
            self.range,
            [
                ("pattern", self.pattern.to_ast(module)?),
                ("name", self.name.to_ast(module)?),
            ],
        )
    }
}
impl ToAst for PatternMatchOr {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        module
            .attr("MatchOr")?
            .call_with_loc(self.range, [("patterns", self.patterns.to_ast(module)?)])
    }
}
impl ToAst for PatternMatchClass {
    fn to_ast(&self, module: &AstModule) -> PyResult {
        let mut kwd_attrs = vec![];
        let mut kwd_patterns = vec![];
        for kwd in self.arguments.keywords.iter() {
            kwd_attrs.push(kwd.attr.to_ast(module)?);
            kwd_patterns.push(kwd.pattern.to_ast(module)?);
        }
        module.attr("MatchClass")?.call_with_loc(
            self.range,
            [
                ("cls", self.cls.to_ast(module)?),
                ("patterns", self.arguments.patterns.to_ast(module)?),
                ("kwd_attrs", kwd_attrs.into_py(module.py)),
                ("kwd_patterns", kwd_patterns.into_py(module.py)),
            ],
        )
    }
}
