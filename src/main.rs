use macroquad::prelude::*;

struct Cell {
    current_pos: (f32, f32),
    last_pos: (f32, f32),
}

impl Default for Cell {
    fn default() -> Self {
        Self{current_pos: (100.0, 100.0), last_pos: (100.0, 100.0)}
    }
}

struct Snake {
    snake: Vec<Cell>,
    dir: KeyCode
}

impl Default for Snake {
    fn default() -> Self {
        Self{snake:vec![Cell::default()],
             dir: KeyCode::Space}
    }
}

struct Apple {
    pos: (f32, f32),
    eaten: bool,
}

impl Apple {
    fn spawn_new(&mut self) {
        rand::srand(macroquad::miniquad::date::now() as _);
        if self.eaten == true {
            self.pos.0 = rand::gen_range(1, 10) as f32 * 50.0 as f32;
            self.pos.1 = rand::gen_range(1, 10) as f32 * 50.0 as f32;
        }
        self.eaten = false;
    }

    fn draw(&self) {
        draw_rectangle(self.pos.0, self.pos.1, 49.0, 49.0, BLUE);
    }
}

impl Snake {
    fn check_direction(&mut self) {
        self.dir = get_last_key_pressed().unwrap_or(self.dir);
    }

    fn walk(&mut self) {
        self.snake[0].last_pos = self.snake[0].current_pos;

        match self.dir {
            KeyCode::W => self.snake[0].current_pos.1 -= 50.0,
            KeyCode::A => self.snake[0].current_pos.0 -= 50.0,
            KeyCode::S => self.snake[0].current_pos.1 += 50.0,
            KeyCode::D => self.snake[0].current_pos.0 += 50.0,
            _ => return 
        }
        
        for i in 1..self.snake.len() {
            self.snake[i].last_pos = self.snake[i].current_pos;
            self.snake[i].current_pos = self.snake[i - 1].last_pos;
        }
    }
    fn check_borders(&self) -> bool {
        let x_col = match self.snake.first().unwrap().current_pos.0 as i32 {
            50..=500 => false,
            _ => true
        };
        let y_col = match self.snake.first().unwrap().current_pos.1 as i32{
            50..=500 => false,
            _ => true
        };
        return x_col || y_col
    }
    fn check_apple(&mut self, apple: &mut Apple) {
        if self.snake[0].current_pos == apple.pos {
            apple.eaten = true;
            let lp = self.snake.last().unwrap().last_pos;
            self.snake.push(Cell{current_pos: lp, last_pos: lp});
        }
    }
    
    fn draw(&self) {
        for i in 0..self.snake.len() {
            let mut color = RED;
            if i == 0 {color = DARKGRAY}
            draw_rectangle(self.snake[i].current_pos.0, 
                self.snake[i].current_pos.1, 
                49.0, 49.0, color);
        }
    }
}

fn draw_field() {
    for i in 1..=10 {
    for j in 1..=10 {
        draw_rectangle((j * 50) as f32, (i * 50) as f32,
            49.0, 49.0, WHITE);
    }}
}

#[macroquad::main("SnakeGame")]
async fn main() {
    let mut snake = Snake::default();
    let mut delta: f32 = 0.0;
    let mut apple = Apple{pos: (200.0, 200.0), eaten: false};
    loop {
        clear_background(BLACK);
        
        draw_field();

        snake.draw();
        snake.check_direction();
        
        delta += get_frame_time();
        if delta >= 0.25 {
            snake.walk();
            snake.check_apple(&mut apple);
            apple.spawn_new();
            delta -= 0.25;
        } 
        
        if snake.check_borders() == true {
            println!("Your score {}", snake.snake.len());
            std::process::exit(1);
        }
        
        apple.draw();
        next_frame().await
    }
}
