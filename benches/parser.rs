// extern crate xonsh_rd_parser;
use divan::Bencher;
use pyo3::prelude::*;
use xonsh_rd_parser::parser::PyParser;
use xonsh_rd_parser::test_utils::get_big_py_file;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench()]
fn python_parser(bencher: Bencher) {
    let file_name = get_big_py_file();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        bencher.bench_local(move || {
            PyParser::parse_file(py, file_name.as_str()).unwrap();
        });
    })
}
