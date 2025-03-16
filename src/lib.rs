use pyo3::prelude::*;

#[pyclass(module = "submodule_mkdocs_test._submodule_mkdocs_test")]
struct It();

#[pymethods]
impl It {
    #[new]
    fn new() -> Self {
        It()
    }

    fn it_method(&self) -> PyResult<()> {
        println!("It method called");
        Ok(())
    }
}

#[pyclass(module = "submodule_mkdocs_test._submodule_mkdocs_test._my_submodule")]
struct Thing();

#[pymethods]
impl Thing {
    #[new]
    fn new() -> Self {
        Thing()
    }

    fn thing_method(&self) -> PyResult<()> {
        println!("Thing method called");
        Ok(())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _submodule_mkdocs_test(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<It>()?;
    let my_submodule = PyModule::new(py, "_my_submodule")?;
    m.add_submodule(&my_submodule)?;
    my_submodule.add_class::<Thing>()?;

    // workaround for pyo3 bug #759
    let sys_modules = py.import("sys")?.getattr("modules")?;
    sys_modules.set_item("submodule_mkdocs_test._submodule_mkdocs_test._my_submodule", &my_submodule)?;

    // Workaround proposed by amorenoz in Pyo3 issue #4870
    my_submodule.setattr(
        "__module__",
        "submodule_mkdocs_test._submodule_mkdocs_test._my_submodule",
    )?;

    Ok(())
}
