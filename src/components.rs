use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
}
impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}
impl Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}
impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}
impl Mul<i32> for Position {
    type Output = Position;
    fn mul(self, rhs: i32) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl MulAssign<i32> for Position {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl Mul<Position> for i32 {
    type Output = Position;
    fn mul(self, rhs: Position) -> Self::Output {
        Position {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
impl Div<i32> for Position {
    type Output = Position;
    fn div(self, rhs: i32) -> Self::Output {
        Position {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn w(&self) -> f32 {
        self.width
    }
    pub fn h(&self) -> f32 {
        self.height
    }
    pub fn rectangle(x: f32, y: f32) -> Self {
        Self {
            width: x,
            height: y,
        }
    }
    pub fn square(x: f32) -> Self {
        Self::rectangle(x, x)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::UP => Self::DOWN,
            Self::RIGHT => Self::LEFT,
            Self::DOWN => Self::UP,
            Self::LEFT => Self::RIGHT,
        }
    }
    pub fn to_pos(&self, step: i32) -> Position {
        match self {
            Self::UP => Position::new(0, step),
            Self::RIGHT => Position::new(step, 0),
            Self::DOWN => Position::new(0, -step),
            Self::LEFT => Position::new(-step, 0),
        }
    }
    pub fn dir2num(dir: &Direction) -> u8 {
        match dir {
            Self::UP => 0,
            Self::RIGHT => 1,
            Self::DOWN => 2,
            Self::LEFT => 3,
        }
    }
    pub fn num2dir(num: u8) -> Direction {
        match num % 4 {
            0 => Self::UP,
            1 => Self::RIGHT,
            2 => Self::DOWN,
            3 => Self::LEFT,
            _ => Self::UP,
        }
    }
    pub fn get_num(&self) -> u8 {
        Self::dir2num(self)
    }
}

#[derive(Component)]
pub struct Snake {
    direction: Direction,
    bodies: Vec<Entity>,
    pub tail_pos: Position,
}
impl Default for Snake {
    fn default() -> Self {
        Snake {
            direction: Direction::UP,
            bodies: Vec::new(),
            tail_pos: Position::default(),
        }
    }
}
impl Snake {
    pub fn add(&mut self, body: Entity) {
        self.bodies.push(body);
    }
    pub fn get_dir(&self) -> Direction {
        self.direction
    }
    pub fn set_dir(&mut self, dir: Direction) {
        self.direction = dir;
    }
    pub fn get_head(&self) -> &Entity {
        self.bodies.first().unwrap()
    }
    pub fn get_tail(&self) -> &Entity {
        self.bodies.last().unwrap()
    }
    pub fn set_tail_pos(&mut self, pos: Position) {
        self.tail_pos = pos;
    }
    pub fn get_tail_pos(&self) -> Position {
        self.tail_pos
    }
    pub fn despawn(&self, commands: &mut Commands) {
        for ent in self.bodies.iter() {
            commands.entity(*ent).despawn();
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.bodies.iter()
    }
    // pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
    //     self.bodies.iter_mut()
    // }
    pub fn len(&self) -> usize {
        self.bodies.len()
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Enemy(usize);

impl Enemy {
    pub fn get(&self) -> usize {
        self.0
    }
    pub fn increment(&mut self) {
        self.0 += 1;
    }
    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct FlashMask;
