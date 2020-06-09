extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use std::io::{stdin, stdout, Write};
use std::process::exit;

mod percolation_grid;
use percolation_grid::*;
mod constants;
use constants::BLOCK_SIZE;
mod percolation;
mod weighted_union_find;

fn read(input: &mut String) {
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read");
}

fn create_grid(rows: u32, cols: u32, automatic: bool) {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Percolation Visualizer",
        [(BLOCK_SIZE * cols) as f64, (BLOCK_SIZE * rows) as f64],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
    // Create a new game and run it.
    let mut grid = PercolationGrid::new(opengl, rows, cols, automatic);

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            grid.render(&args);
        }
        if let Some(args) = e.update_args() {
            if !automatic && !grid.percolates() {
                let (row, col) = get_rows_cols_from_input();
                if row > rows {
                    println!("Too high row number, max is {}", rows);
                } else if col > cols {
                    println!("Too high column number, max is {}", cols);
                } else {
                    grid.open_site(row, col);
                    if grid.percolates() {
                        println!("Percolation achieved!");
                    }
                }
            }
            grid.update(&args);
        }
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                grid.pressed(&args.button);
            }
        }
    }
}

fn auto_grid(rows: u32, cols: u32) {
    create_grid(rows, cols, true);
}

fn manual_grid(rows: u32, cols: u32) {
    create_grid(rows, cols, false);
}

fn get_rows_cols_from_input() -> (u32, u32) {
    let mut rows = String::new();
    let mut cols = String::new();
    let rows_parsed: i32;
    let cols_parsed: i32;

    print!("What is the row number?: ");
    loop {
        rows = String::new();
        read(&mut rows);
        match rows.trim().parse() {
            Err(_err) => {
                println!("Invalid number");
                continue;
            }
            Ok(res) => {
                if res < 0 {
                    println!("Too small number");
                    continue;
                }
                rows_parsed = res;
                break;
            }
        }
    }

    print!("What is the column number?: ");
    loop {
        cols = String::new();
        read(&mut cols);
        match cols.trim().parse() {
            Err(_err) => {
                println!("Invalid number");
                continue;
            }
            Ok(res) => {
                if res < 0 {
                    println!("Too small number");
                    continue;
                }
                cols_parsed = res;
                break;
            }
        }
    }

    return (rows_parsed as u32, cols_parsed as u32);
}

fn main() {
    let rows: u32 = 40;
    let cols: u32 = 60;

    loop {
        println!(
            "Choose a mode of operation by entering a number(0-3):
    0) Default {} x {} grid, then automatic site opening
    1) Enter rows and columns number for the grid, then automatic site opening
    2) Enter rows and columns number for the grid, then manually open sites
    3) Exit",
            rows, cols
        );
        loop {
            let choice_parsed: i32;
            let mut choice = String::new();
            read(&mut choice);
            match choice.trim().parse() {
                Err(_err) => {
                    println!("Invalid choice");
                    continue;
                }
                Ok(res) => {
                    if res > 3 || res < 0 {
                        println!("Invalid choice");
                        continue;
                    }
                    choice_parsed = res;
                }
            }

            match choice_parsed {
                0 => {
                    auto_grid(rows, cols);
                    break;
                }
                1 => {
                    let (rows, cols) = get_rows_cols_from_input();
                    auto_grid(rows, cols);
                    break;
                }
                2 => {
                    let (rows, cols) = get_rows_cols_from_input();
                    manual_grid(rows, cols);
                    break;
                }
                3 => {
                    exit(0);
                }
                _ => println!("This should never happen."),
            }
        }
    }
}
