use macroquad::{miniquad::window::set_window_size, prelude::*};

const WIDTH: usize = 160;
const HEIGHT: usize = 90;
const CELL_SIZE: f32 = 9.0;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;

const OFFSET: Vec2 = Vec2::new(
    SCREEN_WIDTH as f32 / 2.0 - WIDTH as f32 * CELL_SIZE / 2.0,
    SCREEN_HEIGHT as f32 / 2.0 - HEIGHT as f32 * CELL_SIZE / 2.0,
);

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
    pub fn is_blocking(&self) -> bool {
        *self != Self::Empty
    }
}

#[derive(Debug, Clone)]
struct Grid {
    // use box to prevent stack overflow
    grid: Box<[[Cell; WIDTH]; HEIGHT]>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: Box::new([[Default::default(); WIDTH]; HEIGHT]),
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.grid[y][x] = cell;
    }

    #[must_use]
    fn cell_at(&mut self, x: usize, y: usize) -> Cell {
        self.grid[y][x]
    }

    fn update_sand(&mut self, x: usize, y: usize) {
        self.set_cell(x, y, Cell::Empty);
        let down = self.cell_at(x, y + 1);

        if down.is_blocking() {
            let right = self.cell_at(x + 1, y);
            let left = self.cell_at(x - 1, y);
            let down_right = self.cell_at(x + 1, y + 1);
            let down_left = self.cell_at(x - 1, y + 1);

            // if down_right.is_empty() && right.is_empty() {
            //     self.set_cell(x + 1, y + 1, Cell::Sand);
            // } else if down_left.is_empty() && left.is_empty() {
            //     self.set_cell(x - 1, y + 1, Cell::Sand);
            // } else {
            //     self.set_cell(x, y, Cell::Sand);
            // }


            let random = rand::gen_range(0, 2);
            if random == 1 {
                if right.is_empty() && down_right.is_empty() {
                    self.set_cell(x + 1, y + 1, Cell::Sand);
                    return;
                }
            } else {
                if left.is_empty() && down_left.is_empty() {
                    self.set_cell(x - 1, y + 1, Cell::Sand);
                    return;
                }
            }
                self.set_cell(x, y, Cell::Sand);














        } else {
            self.set_cell(x, y + 1, Cell::Sand);
        }
    }

    fn update_water(&mut self, x: usize, y: usize) {
        self.set_cell(x, y, Cell::Empty);
        let down = self.cell_at(x, y + 1);

        if down.is_blocking() {
            let right = self.cell_at(x + 1, y);
            let down_right = self.cell_at(x + 1, y + 1);
            let left = self.cell_at(x - 1, y);
            let down_left = self.cell_at(x - 1, y + 1);

            // randomly decide which side to choose
            // TODO: better randomness
            let random = rand::gen_range(0, 2);
            if random == 1 {
                if right.is_empty() && down_right.is_empty() {
                    self.set_cell(x + 1, y + 1, Cell::Water);
                    return;
                } else if right.is_empty() {
                    self.set_cell(x + 1, y, Cell::Water);
                    return;
                }
            } else {
                if left.is_empty() && down_left.is_empty() {
                    self.set_cell(x - 1, y + 1, Cell::Water);
                    return;
                } else if left.is_empty() {
                    self.set_cell(x - 1, y, Cell::Water);
                    return;
                }
            }

            self.set_cell(x, y, Cell::Water);
        } else {
            self.set_cell(x, y + 1, Cell::Water);
        }
    }

    pub fn update(&mut self) {
        let mut grid = *self.grid;

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if y == HEIGHT - 1 || x == WIDTH - 1 || x == 0 {
                    continue;
                }

                match *cell {
                    Cell::Water => self.update_water(x, y),
                    Cell::Sand => self.update_sand(x, y),
                    Cell::Static | Cell::Empty => {}
                }
            }
        }
    }

    pub fn draw(&self) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                draw_rectangle(
                    x as f32 * CELL_SIZE + OFFSET.x,
                    y as f32 * CELL_SIZE + OFFSET.y,
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
            tools_idx: 0,
        }
    }

    pub fn update(&mut self) {
        self.grid.update();
    }

    pub fn draw(&mut self) {
        self.grid.draw();
        self.draw_ui();
    }

    fn draw_ui(&mut self) {
        let block_size = 50.0;
        let padding = 5.0;

        draw_rectangle(
            padding + OFFSET.x,
            padding + OFFSET.y,
            block_size,
            block_size,
            self.tools[self.tools_idx].as_color(),
        );

        draw_rectangle_lines(
            padding + OFFSET.x,
            padding + OFFSET.y,
            block_size,
            block_size,
            5.0,
            BLACK,
        );

        let wheel = mouse_wheel().1;

        if wheel > 0.0 {
            self.tools_idx += 1;
        } else if wheel < 0.0 {
            self.tools_idx = self.tools_idx.saturating_sub(1);
        }

        self.tools_idx %= self.tools.len();
    }

    fn set_tool(&mut self, cell: Cell) {
        self.tools_idx = self.tools.iter().position(|item| *item == cell).unwrap();
    }

    pub fn handle_input_tools(&mut self) {
        if is_key_pressed(KeyCode::W) {
            self.set_tool(Cell::Water);
        }

        if is_key_pressed(KeyCode::S) {
            self.set_tool(Cell::Sand);
        }

        if is_key_pressed(KeyCode::B) {
            self.set_tool(Cell::Static);
        }

        if is_key_pressed(KeyCode::E) {
            self.set_tool(Cell::Empty);
        }
    }

    pub fn handle_input(&mut self) {
        let (mut x, mut y) = mouse_position();
        x -= OFFSET.x;
        y -= OFFSET.y;

        if is_mouse_button_down(MouseButton::Left) {
            self.grid.set_cell(
                (x / CELL_SIZE) as usize,
                (y / CELL_SIZE) as usize,
                self.tools[self.tools_idx],
            );
        }
    }
}

#[macroquad::main("sand")]
async fn main() {
    set_window_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut app = Application::new();

    let delay_secs = 0.1;
    let mut fut = 0.0;

    loop {
        clear_background(BLACK);

        if get_time() >= fut {
            fut = get_time() + delay_secs;
        }
        app.handle_input();
        app.update();

        app.draw();
        app.handle_input_tools();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
