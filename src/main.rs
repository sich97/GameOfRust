use nannou::prelude::*;
use rand::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const CELL_SIZE: u32 = 25;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .update(update)
        .run();
}

struct Model {
    cells: [[bool; (WIDTH / CELL_SIZE) as usize]; (HEIGHT / CELL_SIZE) as usize],
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    let mut cells = [[false; (WIDTH / CELL_SIZE) as usize]; (HEIGHT / CELL_SIZE) as usize];
    let mut rng = rand::rng();
    for row in cells.iter_mut() {
        for cell in row.iter_mut() {
            // Randomly set each cell to true or false
            *cell = rng.random_bool(0.5); // 50% chance to be true
        }
    }
    Model { cells }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut cells_to_die: Vec<(usize, usize)> = Vec::new();
    let mut cells_to_spawn: Vec<(usize, usize)> = Vec::new();
    for (row_index, row) in model.cells.iter().enumerate() {
        for (column_index, _cell) in row.iter().enumerate() {
            let neighbours = get_cell_neighbours(&model.cells, row_index, column_index);
            // Apply rules of Conway's Game of Life
            if model.cells[row_index][column_index] {
                // Cell is alive
                if neighbours < 2 || neighbours > 3 {
                    cells_to_die.push((row_index, column_index)); // Cell dies
                }
            } else {
                // Cell is dead
                if neighbours == 3 {
                    cells_to_spawn.push((row_index, column_index)); // Cell spawns
                }
            }
        }
    }

    for cell in cells_to_die {
        model.cells[cell.0][cell.1] = false; // Set cell to dead
    }

    for cell in cells_to_spawn {
        model.cells[cell.0][cell.1] = true; // Set cell to alive
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for (row_index, row) in model.cells.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            if *cell {
                // If the boolean is true, draw the cell
                let y = row_index as f32 * CELL_SIZE as f32
                    - (HEIGHT as f32 / 2.0 - CELL_SIZE as f32 / 2.0);
                let x = column_index as f32 * CELL_SIZE as f32
                    - (WIDTH as f32 / 2.0 - CELL_SIZE as f32 / 2.0);
                draw.rect()
                    .xy(vec2(x, y))
                    .wh(vec2(CELL_SIZE as f32, CELL_SIZE as f32))
                    .color(WHITE);
            } else {
                // If the boolean is false, draw an empty cell
                let y = row_index as f32 * CELL_SIZE as f32
                    - (HEIGHT as f32 / 2.0 - CELL_SIZE as f32 / 2.0);
                let x = column_index as f32 * CELL_SIZE as f32
                    - (WIDTH as f32 / 2.0 - CELL_SIZE as f32 / 2.0);
                draw.rect()
                    .xy(vec2(x, y))
                    .wh(vec2(CELL_SIZE as f32, CELL_SIZE as f32))
                    .color(BLACK);
            }
        }
        draw.to_frame(app, &frame).unwrap();
    }
}

fn get_cell_neighbours(
    cells: &[[bool; (WIDTH / CELL_SIZE) as usize]; (HEIGHT / CELL_SIZE) as usize],
    row_index: usize,
    column_index: usize,
) -> usize {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue; // Skip the cell itself
            }
            let new_row_index = row_index as isize + i;
            let new_column_index = column_index as isize + j;
            if new_row_index >= 0
                && new_row_index < cells.len() as isize
                && new_column_index >= 0
                && new_column_index < cells[new_row_index as usize].len() as isize
            {
                if cells[new_row_index as usize][new_column_index as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}
