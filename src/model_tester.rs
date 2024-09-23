use std::fs::File;
use std::io::{BufRead, BufReader};
use ndarray::{Array2, Array3};
use serde_json::{self, Value};
use anyhow::{Result, Context, anyhow};
use crate::maze_logic::Maze;

pub fn main() -> Result<()> {
    // Load test data and model outputs
    let (mazes, model_outputs) = load_test_data("model_outputs.json", 1000)?;

    let mut correct_predictions = 0;
    let mut total_predictions = 0;

    for (mut maze, output) in mazes.into_iter().zip(model_outputs.iter()) {
        let output_2d = Array2::from_shape_vec((maze.height, maze.width), output.to_vec())
            .context("Failed to reshape output to 2D array")?;

        // Debug: Print maze dimensions and output shape
        println!("Maze dimensions: {}x{}", maze.width, maze.height);
        println!("Output shape: {:?}", output_2d.shape());

        // Check if the predicted path is valid and optimal
        let interpreted = maze.interpret_model_output(output_2d);
        let can_follow = maze.can_follow_path();

        // Debug: Print intermediate results
        println!("interpret_model_output result: {}", interpreted);
        println!("can_follow_path result: {}", can_follow);

        if interpreted && can_follow {
            correct_predictions += 1;
        }
        
        total_predictions += 1;

        // Debug: Print current accuracy
        println!("Current accuracy: {:.2}%", (correct_predictions as f32 / total_predictions as f32) * 100.0);
        println!("--------------------");
    }

    let accuracy = correct_predictions as f32 / total_predictions as f32;
    println!("Final Model Accuracy: {:.2}%", accuracy * 100.0);
    Ok(())
}

fn load_test_data(file_path: &str, num_samples: usize) -> Result<(Vec<Maze>, Vec<Vec<f32>>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut mazes = Vec::new();
    let mut outputs = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.context(format!("Failed to read line {}", line_number + 1))?;
        let data: Value = serde_json::from_str(&line)
            .context(format!("Failed to parse JSON on line {}", line_number + 1))?;
        
        let input = data["input"].as_array()
            .ok_or_else(|| anyhow!("Invalid 'input' structure on line {}", line_number + 1))?;
        let output = data["output"].as_array()
            .ok_or_else(|| anyhow!("Invalid 'output' structure on line {}", line_number + 1))?;

        let height = input.len();
        let width = input[0].as_array()
            .ok_or_else(|| anyhow!("Invalid input row on line {}", line_number + 1))?
            .len();

        let mut maze = Maze::new(width, height);

        for (y, row) in input.iter().enumerate() {
            let row = row.as_array()
                .ok_or_else(|| anyhow!("Invalid input row on line {}", line_number + 1))?;
            for (x, cell) in row.iter().enumerate() {
                let cell = cell.as_array()
                    .ok_or_else(|| anyhow!("Invalid cell data on line {}", line_number + 1))?;
                
                if x < width - 1 && cell[0].as_f64() == Some(1.0) {
                    maze.r_walls[y * width + x] = true;
                }
                if y < height - 1 && cell[1].as_f64() == Some(1.0) {
                    maze.b_walls[y * width + x] = true;
                }
                if cell[2].as_f64() == Some(1.0) {
                    maze.start_pos = (x as u8, y as u8);
                } else if cell[2].as_f64() == Some(-1.0) {
                    maze.end_pos = (x as u8, y as u8);
                }
            }
        }

    let flattened_output: Vec<f32> = output.iter()
        .flat_map(|row| {
            row.as_array()
                .map(|arr| arr.iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect::<Vec<f32>>())
                .unwrap_or_else(Vec::new)
        })
        .collect();

    // Debug: Print a sample of the output values
    println!("Sample output values: {:?}", &flattened_output.iter().take(10).collect::<Vec<_>>());

    mazes.push(maze);
    outputs.push(flattened_output);

        if mazes.len() >= num_samples {
            break;
        }
    }

    Ok((mazes, outputs))
}
