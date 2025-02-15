#![allow(dead_code)]

use plotters::prelude::*;
use plotters::coord::Shift;
use plotters::coord::types::RangedCoordf64;
use plotters::drawing::DrawingArea;
use plotters::backend::BitMapBackend;

struct PlotContext<'a> {
    filename: String,
    root: DrawingArea<BitMapBackend<'a>, Shift>,
    chart: ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
}

fn create_plot_context<'a>(
    title: &'a str, 
    x_min: f64, 
    x_max: f64, 
    y_min: f64, 
    y_max: f64,
) -> Result<PlotContext, Box<dyn std::error::Error>> {
    let filename: String = format!("{}.png", title.replace(" ", "_").to_lowercase());

    let root = BitMapBackend::new(&filename.clone(), (1600, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let chart = ChartBuilder::on(&root)
        .caption("Chart", ("Consolas", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    Ok(PlotContext { filename, root, chart })
}

/// calculate min and max for plotting
fn calculate_ranges<T: Into<f64> + Copy>(values: &[T]) -> (f64, f64) {
    let f_values: Vec<f64> = values.iter().map(|&x| x.into()).collect();
    let min = f_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = f_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (min, max)
}

/// creates a simple 2d scatter plot
pub fn simple_scatter_plot<T: Into<f64> + Copy>(
    x_data: &[T], 
    y_data: &[T], 
    filename: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // validate input
    if x_data.len() != y_data.len() {
        return Err("x and y must have the same length".into());
    }
    
    // convert and calculate ranges
    let x_values: Vec<f64> = x_data.iter().map(|&x| x.into()).collect();
    let y_values: Vec<f64> = y_data.iter().map(|&y| y.into()).collect();
    let (x_min, x_max) = calculate_ranges(x_data);
    let (y_min, y_max) = calculate_ranges(y_data);

    // create plot
    let (root, mut chart) = create_plot_context(filename, x_min, x_max, y_min, y_max)?;

    // configure and draw
    chart.configure_mesh().draw()?;
    chart.draw_series(
        x_values.iter().zip(y_values.iter()).map(|(&x, &y)|
            Cross::new((x, y), 5, Into::<ShapeStyle>::into(&BLACK))
        )
    )?;

    root.present()?;
    Ok(())
} 

/// creates a simple 2d line plot
pub fn simple_line_plot<T: Into<f64> + Copy, U: Into<f64> + Copy>(
    x_data: &[T], 
    y_data: &[U], 
    filename: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // validate input
    if x_data.len() != y_data.len() {
        return Err("x and y must have the same length".into());
    }
    
    // convert and calculate ranges
    let x_values: Vec<f64> = x_data.iter().map(|&x| x.into()).collect();
    let y_values: Vec<f64> = y_data.iter().map(|&y| y.into()).collect();
    let (x_min, x_max) = calculate_ranges(x_data);
    let (y_min, y_max) = calculate_ranges(y_data);

    // create plot
    let (root, mut chart) = create_plot_context(filename, x_min, x_max, y_min, y_max)?;

    // configure and draw
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        x_values.iter().zip(y_values.iter()).map(|(&x, &y)| (x, y)),
        &BLACK
    ))?;

    root.present()?;
    Ok(())
} 

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_scatter_plot() {
        let x = vec![1, 2, 3, 4, 5];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        
        let result = simple_scatter_plot(&x, &y, "scatter_test.png");
        assert!(result.is_ok());
        assert!(Path::new("scatter_test.png").exists());
    }

    #[test]
    fn test_line_plot() {
        let x: Vec<i32> = vec![1, 2, 3, 4, 5];
        let y: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        
        let result = simple_line_plot(&x, &y, "line_test.png");
        assert!(result.is_ok());
        assert!(Path::new("line_test.png").exists());
    }

    #[test]
    fn test_mismatched_lengths() {
        let x: Vec<i32> = vec![1, 2, 3];
        let y: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0];
        
        let scatter_result = simple_scatter_plot(&x, &y, "scatter_error.png");
        assert!(scatter_result.is_err());

        let line_result = simple_line_plot(&x, &y, "line_error.png");
        assert!(line_result.is_err());
    }
}