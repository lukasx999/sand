use macroquad::{miniquad::window::set_window_size, prelude::*};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const CELL_SIZE: f32 = 9.0;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
enum Cell {
    Sand,
    Water,
    Static,
    #[default]
    Empty,
}

impl Cell {
    #[must_use]
    pub fn as_color(&self) -> Color {
        match self {
            Cell::Sand => WHITE,
            Cell::Water => BLUE,
            Cell::Static => BLACK,
            Cell::Empty => GRAY,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }

    #[must_use]
    pub fn is_filled(&self) -> bool {
        *self == Self::Sand
    }

    #[must_use]
    pub fn is_blocking(&self) -> bool {
        *self != Self::Empty
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: [[Cell; WIDTH]; HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: [[Default::default(); WIDTH]; HEIGHT],
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.grid[y][x] = cell;
    }

    #[must_use]
    fn cell_at(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.grid[y][x]
    }

    pub fn update(&mut self) {
        let mut grid = self.grid;

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if y == HEIGHT - 1 || x == WIDTH - 1 || x == 1 {
                    continue;
                }

                match *cell {
                    Cell::Water => {
                        self.set_cell(x, y, Cell::Empty);

                        let down = self.cell_at(x, y + 1).is_blocking();

                        if down {
                            let right = self.cell_at(x + 1, y).is_empty();
                            let down_right = self.cell_at(x + 1, y + 1).is_empty();
                            let left = self.cell_at(x - 1, y).is_empty();
                            let down_left = self.cell_at(x - 1, y + 1).is_empty();

                            if right && down_right {
                                self.set_cell(x + 1, y + 1, Cell::Water);
                            } else if right && !down_right {
                                self.set_cell(x + 1, y, Cell::Water);
                            } else if left && down_left {
                                self.set_cell(x - 1, y + 1, Cell::Water);
                            } else if left && !down_left {
                                self.set_cell(x - 1, y, Cell::Water);
                            } else {
                                self.set_cell(x, y, Cell::Water);
                            }
                        } else {
                            *self.cell_at(x, y + 1) = Cell::Water;
                        }
                    }

                    Cell::Sand => {
                        self.set_cell(x, y, Cell::Empty);

                        let down = self.cell_at(x, y + 1).is_blocking();

                        if down {
                            let right = self.cell_at(x + 1, y).is_empty();
                            let left = self.cell_at(x - 1, y).is_empty();
                            let down_right = self.cell_at(x + 1, y + 1).is_empty();
                            let down_left = self.cell_at(x - 1, y + 1).is_empty();

                            if down_right && right {
                                self.set_cell(x + 1, y + 1, Cell::Sand);
                            } else if down_left && left {
                                self.set_cell(x - 1, y + 1, Cell::Sand);
                            } else {
                                self.set_cell(x, y, Cell::Sand);
                            }
                        } else {
                            self.set_cell(x, y + 1, Cell::Sand);
                        }
                    }

                    Cell::Static | Cell::Empty => {}
                }
            }
        }
    }

    pub fn draw(&self) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    cell.as_color(),
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Application {
    grid: Grid,
    tools: [Cell; 4],
    tools_idx: usize,
}

impl Application {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
            tools: [Cell::Sand, Cell::Water, Cell::Static, Cell::Empty],
            // TODO: use cycling iterator
            tools_idx: 0,
        }
    }

    pub fn update(&mut self) {
        self.grid.update();
        self.grid.draw();
        self.handle_input();
        self.draw_ui();
    }

    fn draw_ui(&mut self) {
        let block_size = 50.0;
        let padding = 5.0;
        draw_rectangle(
            padding,
            padding,
            block_size,
            block_size,
            self.tools[self.tools_idx].as_color(),
        );
        draw_rectangle_lines(padding, padding, block_size, block_size, 5.0, BLACK);

        let wheel = mouse_wheel().1;
        if wheel > 0.0 {
            self.tools_idx += 1;
        } else if wheel < 0.0 {
            self.tools_idx = self.tools_idx.saturating_sub(1);
        }
        self.tools_idx %= self.tools.len();
    }

    fn handle_input(&mut self) {
        let (x, y) = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            let cell = &mut self.grid.grid[(y / CELL_SIZE) as usize][(x / CELL_SIZE) as usize];
            *cell = self.tools[self.tools_idx];
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    set_window_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut app = Application::new();

    loop {
        clear_background(BLACK);
        draw_rectangle(
            0.0,
            0.0,
            WIDTH as f32 * CELL_SIZE,
            HEIGHT as f32 * CELL_SIZE,
            DARKGRAY,
        );

        app.update();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
