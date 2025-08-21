use nannou::prelude::*;
use rand::prelude::*;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_CELL_SIZE: u32 = 25;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .update(update)
        .run();
}

struct Model {
    width: u32,
    height: u32,
    cell_size: u32,
    cells: Vec<Vec<bool>>,
}

fn model(app: &App) -> Model {
    let args: Vec<String> = std::env::args().collect();
    let mut width = DEFAULT_WIDTH;
    let mut height = DEFAULT_HEIGHT;
    if args.len() > 2 {
        width = args[1].parse::<u32>().unwrap();
        height = args[2].parse::<u32>().unwrap();
    }
    let mut cell_size = DEFAULT_CELL_SIZE;
    if args.len() > 3 {
        // If a cell size is provided, use it; otherwise, use the default
        cell_size = args[3].parse::<u32>().unwrap();
    }

    let _window = app
        .new_window()
        .size(width, height)
        .view(view)
        .build()
        .unwrap();
    let mut cells = vec![vec![false; (width / cell_size) as usize]; (height / cell_size) as usize];
    let mut rng = rand::rng();
    for row in cells.iter_mut() {
        for cell in row.iter_mut() {
            // Randomly set each cell to true or false
            *cell = rng.random_bool(0.5); // 50% chance to be true
        }
    }
    Model {
        width,
        height,
        cell_size,
        cells,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut cells_to_die: Vec<(usize, usize)> = Vec::new();
    let mut cells_to_spawn: Vec<(usize, usize)> = Vec::new();
    for (row_index, row) in model.cells.iter().enumerate() {
        for (column_index, _cell) in row.iter().enumerate() {
            let neighbours = get_cell_neighbours(&model, row_index, column_index);
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
                let y = row_index as f32 * model.cell_size as f32
                    - (model.height as f32 / 2.0 - model.cell_size as f32 / 2.0);
                let x = column_index as f32 * model.cell_size as f32
                    - (model.width as f32 / 2.0 - model.cell_size as f32 / 2.0);
                draw.rect()
                    .xy(vec2(x, y))
                    .wh(vec2(model.cell_size as f32, model.cell_size as f32))
                    .color(WHITE);
            } else {
                // If the boolean is false, draw an empty cell
                let y = row_index as f32 * model.cell_size as f32
                    - (model.height as f32 / 2.0 - model.cell_size as f32 / 2.0);
                let x = column_index as f32 * model.cell_size as f32
                    - (model.width as f32 / 2.0 - model.cell_size as f32 / 2.0);
                draw.rect()
                    .xy(vec2(x, y))
                    .wh(vec2(model.cell_size as f32, model.cell_size as f32))
                    .color(BLACK);
            }
        }
        draw.to_frame(app, &frame).unwrap();
    }
}

fn get_cell_neighbours(model: &Model, row_index: usize, column_index: usize) -> usize {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue; // Skip the cell itself
            }
            let new_row_index = row_index as isize + i;
            let new_column_index = column_index as isize + j;
            if new_row_index >= 0
                && new_row_index < model.cells.len() as isize
                && new_column_index >= 0
                && new_column_index < model.cells[new_row_index as usize].len() as isize
            {
                if model.cells[new_row_index as usize][new_column_index as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}
