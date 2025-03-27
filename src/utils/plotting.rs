use pyo3::prelude::*;
use py03::types::PyList;

pub fn plot_injective_function<F: Fn(f64) -> f64>(
    f: F,
    x_range: (f64, f64),
) -> Result<(), Box<dyn std::error::Error>> {
    Python::with_gil(|py| -> PyResult<()> {
        let plt = py.import("matplotlib.pyplot")?;
        
        let num_points = 100;
        let (start, end) = x_range;
        let mut x_values = Vec::with_capacity(num_points);
        for i in 0..num_points {
            let x = start + (end - start) * (i as f64) / ((num_points - 1) as f64);
            x_values.push(x);
        }
        let y_values: Vec<f64> = x_values.iter().map(|&x| f(x)).collect();

        let py_x = PyList::new(py, &x_values);
        let py_y = PyList::new(py, &y_values);

        plt.call_method("plot", (py_x, py_y), None)?;
        plt.call_method("title", ("Injective Function Plot",), None)?;
        plt.call_method("xlabel", ("x",), None)?;
        plt.call_method("ylabel", ("f(x)",), None)?;
        plt.call_method("grid", (), None)?;
        plt.call_method("savefig", (file_name,), None)?;
        Ok(())
    })?;
    
    Ok(())
}