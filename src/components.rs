use std::ops::{Add, AddAssign, Sub, SubAssign};

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
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
}
impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.get_x() + rhs.get_x(), self.get_y() + rhs.get_y())
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
        Position::new(self.get_x() - rhs.get_x(), self.get_y() - rhs.get_y())
    }
}
impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn get_w(&self) -> f32 {
        self.width
    }
    pub fn get_h(&self) -> f32 {
        self.height
    }
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::UP => Self::DOWN,
            Self::DOWN => Self::UP,
            Self::LEFT => Self::RIGHT,
            Self::RIGHT => Self::LEFT,
        }
    }
    pub fn get_dx_dy(&self, step: i32) -> Position {
        match self {
            Self::UP => Position::new(0, step),
            Self::DOWN => Position::new(0, -step),
            Self::RIGHT => Position::new(step, 0),
            Self::LEFT => Position::new(-step, 0),
        }
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
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Food;
