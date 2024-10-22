use crate::ast_module::{AST, Callable};
/// Convert Ruff's AST to Python AST.

use pyo3::{IntoPy, PyObject, PyResult, Python};
use ruff_python_ast::{ModModule, Stmt, Expr};

pub struct Converter<'py> {
    // Python ast module
    module: AST<'py>,
    // parsed Ruff's AST
    tree: ModModule,
}


impl<'py> Converter<'py> {
    pub fn new(py: Python<'py>, tree: ModModule) -> PyResult<Self> {
        let module = AST::new(py)?;
        Ok(Self { module, tree })
    }

    fn from_expr(&self, expr: &Expr) -> PyResult<PyObject> {
        todo!()
    }
    pub fn convert(&self) -> PyResult<PyObject> {
        let mut body: Vec<PyObject> = vec![];
        for stmt in &self.tree.body {
            let node: PyObject = match stmt {
                Stmt::Assert(stmt) => {
                    let mut args = vec![
                        ("test", self.from_expr(&stmt.test)?),
                    ];
                    if let Some(msg) = stmt.msg.as_ref() {
                        args.push(("msg", self.from_expr(msg)?));
                    }
                    self.module.attr("Assert")?.call_with_loc(stmt.range, args)?.into()
                }
                Stmt::Break(stmt) => {
                    self.module.attr("Break")?.call0_with_loc(stmt.range)?.into()
                }
                Stmt::Pass(stmt) => {
                    self.module.attr("Pass")?.call0_with_loc(stmt.range)?.into()
                }
                Stmt::Continue(stmt) => {
                    self.module.attr("Continue")?.call0_with_loc(stmt.range)?.into()
                }
                // Stmt::FunctionDef(stmt) => {
                //     // ast.FunctionDef(
                //     //                 name=n.string,
                //     //                 args=params or self.make_arguments(None, [], None, [], None),
                //     //                 returns=a,
                //     //                 body=b,
                //     //                 type_comment=tc,
                //     //                 **({'type_params': t or []} if sys.version_info >= (3, 12) else {}),
                //     //                 LOCATIONS,
                //     //             )
                //     // self.module.attr("FunctionDef")?.callk(
                //     //     []
                //     // )
                //     "fn".into_py(self.module.py)
                // }
                _ => todo!()
            };
            body.push(node);
        }
        self.module.to_module(body)
    }
}
