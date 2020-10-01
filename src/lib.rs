use log::info;
use pyo3::prelude::*;
use std::{thread::JoinHandle, thread::sleep, time::Duration};

#[pymodule]
fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<WrappingPyStruct>()?;
    Ok(())
}



#[pyclass(module = "module")]
pub(crate) struct WrappingPyStruct {
    wrapped_struct: WrappedStruct,
}

#[pymethods]
impl WrappingPyStruct {
    #[new]
    fn new() -> Self {
        info!("Logging from pymethods (new)");
        WrappingPyStruct{
            wrapped_struct: WrappedStruct::new()
        }
    }
    pub fn run(&self){
        info!("Logging from pymethods (run)");
        self.wrapped_struct.run().join();
    }

}


// #[derive(Serialize, Deserialize)]
pub struct WrappedStruct {
}


impl WrappedStruct {
    pub fn new() -> Self {
        info!("Logging from pure Rust (new)");
        Self {}
    }

    pub fn run(&self) -> JoinHandle<()> {
        std::thread::spawn(move || loop {
            sleep(Duration::new(5,0));
            info!("Logging from pure Rust (run)");
        })
    }
}