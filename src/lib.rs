#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python, PyObject};
use cpython::buffer::PyBuffer;

// Our module is named 'hello', and can be imported using `import hello`.
// This requires that the output binary file is named `hello.so` (or Windows: `hello.pyd`).
// As the output name cannot be configured in
// cargo (https://github.com/rust-lang/cargo/issues/1970),
// you'll have to rename the output file.
py_module_initializer!(hello, inithello, PyInit_hello, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "func", py_fn!(py, func(x: Vec<f64>)))?;
    m.add(py, "funcbuff", py_fn!(py, funcbuff(x: &PyObject)))?;
    Ok(())
});

// The py_fn!()-macro can translate between Python and Rust values,
// so you can use `&str`, `i32` or `String` in the signature of a function
// callable from Python.
// The first argument of type `Python<'p>` is used to indicate that your
// function may assume that the current thread holds the global interpreter lock.
// Most functions in the `cpython` crate require that you pass this argument.
fn func(_: Python,  x: Vec<f64>) -> PyResult<f64> {
    Ok(x.into_iter().inspect(|i| println!("{:}", i)).sum())
}

fn funcbuff(py: Python,  x: &PyObject) -> PyResult<f64> {
    let x = PyBuffer::get(py, x)?;
    let x = x.as_slice::<f64>(py).expect("not a buffer of f64"); // fix to return err
    Ok(x.into_iter().map(|i| i.get()).inspect(|i| println!("{:}", i)).sum())
}