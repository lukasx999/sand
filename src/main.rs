use macroquad::{miniquad::window::set_window_size, prelude::*};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const CELL_SIZE: f32 = 9.0;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
enum Cell {
    Filled,
    #[default] Empty,
}

impl Cell {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }

    #[must_use]
    pub fn is_filled(&self) -> bool {
        *self == Self::Filled
    }

}

#[derive(Debug, Clone)]
struct Grid {
    grid: [[Cell; WIDTH]; HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        let mut this = Self {
            grid: [[Default::default(); WIDTH]; HEIGHT],
        };
        this.grid[HEIGHT/2][WIDTH/2] = Cell::Filled;
        this
    }

    fn cell_at(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.grid[y][x]
    }

    pub fn update(&mut self) {

        let mut grid = self.grid.clone();

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {

                if y == HEIGHT-1 { return; }

                if cell.is_filled() {

                    if self.cell_at(x, y+1).is_filled() {

                    } else {
                        *self.cell_at(x, y) = Cell::Empty;
                        *self.cell_at(x, y+1) = Cell::Filled;

                    }

                }

            }
        }
    }

    pub fn draw(&self) {

        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_filled() {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        WHITE,
                    );
                }
            }
        }
    }

    pub fn handle_input(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            self.grid[(y/CELL_SIZE) as usize][(x/CELL_SIZE) as usize] = Cell::Filled;
        }
    }

}




#[macroquad::main("BasicShapes")]
async fn main() {

    set_window_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut grid = Grid::new();


    loop {
        clear_background(BLACK);
        draw_rectangle(0.0, 0.0, WIDTH as f32*CELL_SIZE, HEIGHT as f32 * CELL_SIZE, DARKGRAY);

        grid.update();
        grid.draw();
        grid.handle_input();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }


        next_frame().await
    }
}
