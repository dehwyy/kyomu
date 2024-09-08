use std::thread;
use std::time::Duration;

use std::collections::VecDeque;

use crossterm::event::{poll, read, Event, KeyCode};

use crate::core::io::{styled::StyledOutput, Color, Terminal};
use crate::app::{Scenario, GameResults};
use crate::core::random::Random;

const X_KOEF: u16 = 2;
const FIELD_COLOR: Color = Color::DarkBlue;
const SNAKE_COLOR: Color = Color::Magenta;
const GROWER_COLOR: Color = Color::Red;

type Position = (u16, u16);

struct Field {
  terminal_size: Position,
  side_size: u16,
  x_pad: u16,
}

impl Field {
  fn new(terminal_size: Position, side_size: u16, x_pad: u16) -> Self {
    Self { terminal_size, side_size, x_pad }
  }

  fn generate_random_cell(&self) -> Position {
    let x: [u16; 1] = Random::generate(self.x_pad..(self.x_pad + self.side_size * X_KOEF) + 1);
    let y: [u16; 1] = Random::generate(1..self.side_size);

    (x[0], y[0])
  }
}

pub struct SnakeGame<'a> {
  t: &'a mut Terminal,
  field: Field,
  snake: Snake,
  grower: Grower
}

impl<'a> SnakeGame<'a> {
  pub fn new(t: &'a mut Terminal) -> Self {
    let terminal_size = crossterm::terminal::size().unwrap_or((1, 1));
    let playfield_side = terminal_size.0.min(terminal_size.1);
    // TODO: vertical oriented terminal
    let x_pad = (terminal_size.0 / 2 - playfield_side / 2 * X_KOEF).max(0);

    let field = Field::new(terminal_size, playfield_side, x_pad);

    let _snake_pos = (x_pad + playfield_side / 3 * X_KOEF, playfield_side / 2);


    let snake = Snake::new(_snake_pos);

    let grower = Grower::new(
      (0..3).into_iter().map(|_| field.generate_random_cell()).collect::<Vec<_>>()
    );

    Self { t,field, snake, grower }
  }



  fn render_field(&mut self) {
    self.t.clear_all();

    for x in 1..=self.field.side_size {
      for y in 1..=self.field.side_size {
        self.t.queue_move_with_print(
          (x * X_KOEF + self.field.x_pad, y),
          create_colored_cell(FIELD_COLOR)
        );
      }
    }

    self.grower.growers_pos.iter().for_each(|pos| {
      self.t.queue_move_with_print(
        (pos.0, pos.1),
        create_colored_cell(GROWER_COLOR)
      );
    });

    self.t.flush();
  }

  fn render(&mut self) {
      let new_pos = self.snake.move_fd(&mut self.t);
      if let Some(_) = self.grower.remove_on_intersect(new_pos) {
        self.snake.grow(1);
        self.grower.add_new_grower(&mut self.t, self.field.generate_random_cell());
      }

      self.t.flush();

      // self.t.println(StyledOutput::new()
      // .with_text(format!("terminal size is {} x {}", self.terminal_size.0, self.terminal_size.1))
      // .with_color(Color::Magenta));
      // let random_cell: [u16; 2] = random::Random::generate(1..self.playfield_side);

      // self.t.move_with_print(
      //   (random_cell[0] * X_KOEF + self.left_padding, random_cell[1]),
      //   Self::create_colored_cell(Color::Magenta)
      // );
  }
}

impl Scenario for SnakeGame<'_> {
  fn start(&mut self) -> GameResults {

    self.render_field();

    let is_alive = true;

    // TODO: fix bugs
    while is_alive {
      if poll(Duration::from_millis(170)).unwrap() {
        match read().unwrap() {
          Event::Key(event) => {
            match event.code {
              KeyCode::Left | KeyCode::Char('a') => self.snake.change_direction(SnakeDirection::Left),
              KeyCode::Right | KeyCode::Char('d') => self.snake.change_direction(SnakeDirection::Right),
              KeyCode::Up | KeyCode::Char('w') => self.snake.change_direction(SnakeDirection::Up),
              KeyCode::Down | KeyCode::Char('s') => self.snake.change_direction(SnakeDirection::Down),
              _ => {}
            }
          },
          _ => {}
        }
      }

      // thread::sleep(Duration::from_millis(100));


      self.render();
    }

    GameResults::new()
  }
}

enum SnakeDirection {
  Up,
  Down,
  Left,
  Right
}

struct Snake {
  pos: Position,
  direction: SnakeDirection,
  body: VecDeque<Position>,
  move_count: u16,
  previous_cells: Vec<Position>,
}

impl Snake {
  fn new(initial_position: (u16, u16)) -> Self {
    Self {
      pos: initial_position,
      direction: SnakeDirection::Right,
      body: VecDeque::from([initial_position]),
      move_count: 0,
      previous_cells: vec!()
    }
  }

  fn change_direction(&mut self, new_direction: SnakeDirection) {
    self.direction = new_direction;
  }

  fn move_fd(&mut self, t: &mut Terminal) -> Position {
    match self.direction {
      SnakeDirection::Up => self.pos.1 -= 1,
      SnakeDirection::Down => self.pos.1 += 1,
      SnakeDirection::Left => self.pos.0 -= 1,
      SnakeDirection::Right => self.pos.0 += 1
    };


    self.body.push_front(self.pos);
    t.queue_move_with_print(self.pos, create_colored_cell(SNAKE_COLOR));

    // I guess, it should always return Some(v), so nested code will always be executed
    if let Some(v) = self.body.pop_back() {
      self.previous_cells.push(v);
      t.queue_move_with_print(v, create_colored_cell(FIELD_COLOR));
    }

    self.move_count += 1;

    self.pos
  }

  fn grow(&mut self, grow_by: usize) {
    for _ in 0..(grow_by.min(self.previous_cells.len())) {
      if let Some(cell) = self.previous_cells.pop() {
        self.body.push_back(cell);
      }
    }
  }
}

pub struct Grower {
  growers_ate: u16,
  growers_pos: Vec<Position>
}

impl Grower {
  fn new(growers_pos: Vec<Position>) -> Self {
    Self {
      growers_ate: 0,
      growers_pos
    }
  }

  fn remove_on_intersect(&mut self, pos: Position) -> Option<Position> {
    match self.growers_pos.iter_mut().position(|x| *x == pos) {
      Some(idx) => {
        self.growers_ate += 1;
        Some(self.growers_pos.remove(idx))
      },
      None => None
    }
  }

  fn add_new_grower(&mut self, t: &mut Terminal, new_grower_pos: Position) {
    self.growers_pos.push(new_grower_pos);
    t.move_with_print(new_grower_pos, create_colored_cell(GROWER_COLOR));
  }
}

fn create_colored_cell(color: Color) -> StyledOutput {
  StyledOutput::new().with_text("  ").with_bg(color)
}
