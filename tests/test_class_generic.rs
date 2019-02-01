#![feature(specialization)]

extern crate pyo3;
use pyo3::prelude::*;

#[macro_use]
mod common;

#[pyclass(variants("SimpleGenericU32<u32>", "SimpleGenericF32<f32>"))]
struct SimpleGeneric<T: 'static> {
    _foo: T,
}

#[test]
fn generic_names() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let ty_u32 = py.get_type::<SimpleGeneric<u32>>();
    py_assert!(py, ty_u32, "ty_u32.__name__ == 'SimpleGenericU32'");

    let ty_f32 = py.get_type::<SimpleGeneric<f32>>();
    py_assert!(py, ty_f32, "ty_f32.__name__ == 'SimpleGenericF32'");
}

#[test]
fn generic_type_eq() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let tup = (
        (SimpleGeneric { _foo: 1u32 }).into_object(py),
        (SimpleGeneric { _foo: 1u32 }).into_object(py),
        (SimpleGeneric { _foo: 1f32 }).into_object(py),
        (SimpleGeneric { _foo: 1f32 }).into_object(py),
    );

    py_assert!(py, tup, "type(tup[0]) == type(tup[1])");
    py_assert!(py, tup, "type(tup[2]) == type(tup[3])");
    py_assert!(py, tup, "type(tup[0]) != type(tup[2])");
}

#[pyclass(variants("GenericSquarerU64<u64>", "GenericSquarerF64<f64>"))]
struct GenericSquarer<T>
where
    T: std::ops::Mul<Output = T> + Copy + 'static,
{
    val: T,
}

#[pymethods(variants("GenericSquarerU64<u64>", "GenericSquarerF64<f64>"))]
impl<T> GenericSquarer<T>
where
    T: std::ops::Mul<Output = T> + Copy + 'static,
    // #[pyclass] only implements `PyTypeInfo` for the given variants,
    // so we need this constraint below.
    GenericSquarer<T>: pyo3::typeob::PyTypeInfo,
{
    #[new]
    fn __new__(obj: &PyRawObject, val: T) -> PyResult<()> {
        obj.init(|| Self { val })
    }

    #[getter]
    fn get_val(&self) -> PyResult<T> {
        Ok(self.val)
    }

    #[setter]
    fn set_val(&mut self, value: T) -> PyResult<()> {
        self.val = value;
        Ok(())
    }

    fn square(&self) -> PyResult<T> {
        Ok(self.val * self.val)
    }
}

#[test]
fn generic_squarer() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let u64_squarer = py.init(|| GenericSquarer { val: 42u64 }).unwrap();
    py_assert!(
        py,
        u64_squarer,
        "type(u64_squarer).__name__ == 'GenericSquarerU64'"
    );
    py_assert!(py, u64_squarer, "u64_squarer.square() == 42 ** 2");

    let f64_squarer = py.init(|| GenericSquarer { val: 42f64 }).unwrap();
    py_assert!(
        py,
        f64_squarer,
        "type(f64_squarer).__name__ == 'GenericSquarerF64'"
    );
    py_assert!(py, f64_squarer, "f64_squarer.square() == 42. ** 2.");
}

#[test]
fn generic_squarer_getter_setter() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let u64_squarer = py.init(|| GenericSquarer { val: 1337u64 }).unwrap();
    py_assert!(py, u64_squarer, "u64_squarer.val == 1337");
    py_run!(py, u64_squarer, "u64_squarer.val = 42");
    py_assert!(py, u64_squarer, "u64_squarer.val == 42");
    py_assert!(py, u64_squarer, "u64_squarer.square() == 42 ** 2");
}

#[test]
fn generic_squarer_pynew() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let u64_squarer_ty = py.get_type::<GenericSquarer<u64>>();
    py_assert!(
        py,
        u64_squarer_ty,
        "u64_squarer_ty(111).square() == 111 ** 2"
    );
}