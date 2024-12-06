use crate::components::{Direction, Food, Player, Position, Size, Snake};
use crate::events::{GameOverEvent, GrowthEvent};
use crate::resources::IntervalSetting;
use bevy::prelude::*;
use rand::random;
use std::time::{Duration, Instant};

const INITIAL_WIDTH: f32 = 1280.;
const INITIAL_HEIGHT: f32 = 720.;

const SNAKE_HEAD_COLOR: Color = Color::linear_rgb(0.7, 0.7, 0.7);
const SNAKE_BODY_COLOR: Color = Color::linear_rgb(0.3, 0.3, 0.3);
const FOOD_COLOR: Color = Color::linear_rgb(1.0, 0.0, 1.0);

const ARENA_WIDTH: u32 = 60;
const ARENA_HEIGHT: u32 = 60;

const SNAKE_SPEED: u64 = 10;

fn in_arena(x: i32, y: i32) -> bool {
    0 <= x && x < ARENA_WIDTH as i32 && 0 <= y && y < ARENA_HEIGHT as i32
}

pub fn play() {
    App::new()
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(
            Update,
            (
                snake_movement_input.before(snake_movement),
                snake_movement,
                (snake_eating, game_over).after(snake_movement),
                snake_growth.after(snake_eating),
                food_spawner,
            ),
        )
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "snake-game".into(),
                name: Some("snake.app".into()),
                resolution: (INITIAL_WIDTH, INITIAL_HEIGHT).into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(IntervalSetting::<Player>::default())
        .insert_resource(ClearColor(Color::linear_rgb(0.04, 0.04, 0.04)))
        .insert_resource(IntervalSetting::<Food>::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn size_scaling(window: Query<&Window>, mut q: Query<(&Size, &mut Transform)>) {
    let window = window.single();
    let window_size = window.width().min(window.height());
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.get_w() / ARENA_WIDTH as f32 * window_size,
            sprite_size.get_h() / ARENA_HEIGHT as f32 * window_size,
            1.0,
        );
    }
}

fn position_translation(
    window: Query<&Window>,
    mut q: Query<(&Position, &mut Transform)>,
    camera: Single<Entity, With<Camera>>,
    camera2d: Single<Entity, With<Camera2d>>,
) {
    if q.contains(camera.into_inner()) {
        println!("There is a camera transform");
    }
    if q.contains(camera2d.into_inner()) {
        println!("There is a camera2d transform");
    }
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = window.single();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.get_x() as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.get_y() as f32, window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut snake = Snake::default();
    snake.add(
        commands
            .spawn((
                Mesh2d(meshes.add(Circle::default())),
                MeshMaterial2d(materials.add(SNAKE_HEAD_COLOR)),
            ))
            .insert(Position::new(
                ARENA_WIDTH as i32 / 2,
                ARENA_HEIGHT as i32 / 2,
            ))
            .insert(Size::square(0.8))
            .id(),
    );
    snake.add(
        commands
            .spawn((
                Mesh2d(meshes.add(Circle::default())),
                MeshMaterial2d(materials.add(SNAKE_BODY_COLOR)),
            ))
            .insert(Position::new(
                ARENA_WIDTH as i32 / 2,
                ARENA_HEIGHT as i32 / 2 - 1,
            ))
            .insert(Size::square(0.6))
            .id(),
    );
    commands.spawn(Player).insert(snake).insert(Position::new(
        ARENA_WIDTH as i32 / 2,
        ARENA_HEIGHT as i32 / 2,
    ));
}

fn spawn_body(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: Position,
    scale: Size,
) -> Entity {
    commands
        .spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(SNAKE_BODY_COLOR)),
        ))
        .insert(position)
        .insert(scale)
        .id()
}

fn snake_movement_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut snakes: Query<&mut Snake>) {
    if let Some(mut snake) = snakes.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowUp) {
            Direction::UP
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Direction::DOWN
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Direction::RIGHT
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Direction::LEFT
        } else {
            snake.get_dir()
        };
        if dir != snake.get_dir().opposite() {
            snake.set_dir(dir);
        }
    }
}

fn snake_movement(
    mut snakes: Query<&mut Snake>,
    mut positions: Query<&mut Position>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut time: ResMut<IntervalSetting<Player>>,
) {
    let now = Instant::now();
    if !time.check(now, Duration::from_millis(600 / SNAKE_SPEED)) {
        return;
    }
    time.update(now);

    if let Some(mut snake) = snakes.iter_mut().next() {
        let tail = *positions.get(*snake.get_tail()).unwrap();
        snake.set_tail_pos(tail);
        let now_pos = snake
            .iter()
            .map(|e| *positions.get(*e).unwrap())
            .collect::<Vec<Position>>();
        *positions.get_mut(*snake.get_head()).unwrap() += snake.get_dir().get_dx_dy(1);
        let head_pos = *positions.get(*snake.get_head()).unwrap();
        if !in_arena(head_pos.get_x(), head_pos.get_y()) {
            game_over_writer.send(GameOverEvent);
        }
        if now_pos.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        now_pos
            .iter()
            .zip(snake.iter().skip(1))
            .for_each(|(pos, body)| {
                *positions.get_mut(*body).unwrap() = *pos;
            });
    }
}

fn food_spawner(mut commands: Commands, mut time: ResMut<IntervalSetting<Food>>) {
    let now = Instant::now();
    if !time.check(now, Duration::from_secs(1)) {
        return;
    }
    time.update(now);

    commands
        .spawn(Sprite {
            color: FOOD_COLOR,
            ..default()
        })
        .insert(Food)
        .insert(Position::new(
            (random::<f32>() * ARENA_WIDTH as f32) as i32,
            (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        ))
        .insert(Size::square(0.8));
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_position: Query<(Entity, &Position), With<Food>>,
    snakes: Query<(Entity, &Snake)>,
    positions: Query<&Position>,
) {
    if let Some((snake_ent, snake)) = snakes.iter().next() {
        let head_pos = positions.get(*snake.get_head()).unwrap();
        for (food_ent, food_pos) in food_position.iter() {
            if food_pos == head_pos {
                commands.entity(food_ent).despawn();
                growth_writer.send(GrowthEvent { snake: snake_ent });
            }
        }
    }
}

fn snake_growth(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut snakes: Query<&mut Snake>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if let Some(event) = growth_reader.read().next() {
        let mut snake = snakes.get_mut(event.snake).unwrap();
        let tail_pos = snake.get_tail_pos();
        snake.add(spawn_body(
            &mut commands,
            meshes,
            materials,
            tail_pos,
            Size::square(0.6),
        ));
    }
}

fn game_over(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut reader: EventReader<GameOverEvent>,
    snakes: Query<&Snake>,
    snake_entities: Query<Entity, With<Snake>>,
    food_entities: Query<Entity, With<Food>>,
) {
    if reader.read().next().is_some() {
        for snake in snakes.iter() {
            snake.despawn(&mut commands);
        }
        for ent in snake_entities.iter().chain(food_entities.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_player(commands, meshes, materials);
    }
}
