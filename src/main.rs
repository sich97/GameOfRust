use nannou::prelude::*;
use rand::prelude::*;

const BORDER_MARGIN: u32 = 100; // Margin around the window
const WIDTH: u32 = 800 - BORDER_MARGIN * 2;
const HEIGHT: u32 = 600 - BORDER_MARGIN * 2;
const CELL_SIZE: u32 = 50;

fn main() {
    nannou::app(model).update(update).run();
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
    // You can update the model here, e.g., to move the rectangle
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
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
