use macroquad::{miniquad::window::set_window_size, prelude::*};

const WIDTH: usize = 700;
const HEIGHT: usize = 700;
const CELL_WIDTH: f32 = 10.0;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;

#[derive(Debug, Clone)]
struct Grid {
    grid: [[bool; WIDTH]; HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        let mut this = Self {
            grid: [[false; WIDTH]; HEIGHT],
        };
        this.grid[HEIGHT/2][WIDTH/2] = true;
        this
    }

    pub fn update(&mut self) {

        let mut grid = self.grid.clone();

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {

                if y == HEIGHT-1 { return; }

                if *cell {
                    self.grid[y][x] = false;
                    self.grid[y+1][x] = true;
                }

            }
        }
    }

    pub fn draw(&self) {

        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    draw_rectangle(x as f32, y as f32, CELL_WIDTH, CELL_WIDTH, WHITE);
                }
            }
        }
    }

    pub fn handle_input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            self.grid[y as usize][x as usize] = true;
        }
    }

}




#[macroquad::main("BasicShapes")]
async fn main() {

    set_window_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut grid = Grid::new();


    loop {
        clear_background(BLACK);

        grid.update();
        grid.draw();
        grid.handle_input();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }


        draw_rectangle_lines(0.0, 0.0, WIDTH as f32, HEIGHT as f32, 1.0, BLUE);

        next_frame().await
    }
}
