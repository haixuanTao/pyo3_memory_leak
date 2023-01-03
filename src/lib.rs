use pyo3::{
    prelude::*,
    types::{PyBytes, PyDict},
};

use std::time::Duration;

#[pyfunction]
fn leak() {
    let object = Python::with_gil(|py| {
        let object: Py<PyAny> = Py::from(py.import("numpy").unwrap());
        object
    });
    for _ in 0..10 {
        let a: Vec<u8> = vec![0; 40_000_000];
        Python::with_gil(|py| -> PyResult<()> {
            let bytes = PyBytes::new(py, &a);
            let input_dict = PyDict::new(py);
            input_dict.set_item("data", bytes).unwrap();

            let _status_enum = object.call_method1(py, "array", (bytes,)).unwrap();

            std::thread::sleep(Duration::from_secs(1));
            Ok(())
        })
        .unwrap();
    }
}

#[pymodule]
fn pyo3_memory_leak(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(leak, m)?)?;
    Ok(())
}
