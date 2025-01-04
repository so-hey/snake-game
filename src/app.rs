use crate::components::{Direction, Enemy, FlashMask, Food, Player, Position, Size, Snake};
use crate::constants::{
    in_arena, in_enemy_arena, ARENA_HEIGHT, ARENA_WIDTH, BODY_SIZE, DIRECTION_WEIGHT,
    ENEMY_BODY_COLOR, ENEMY_HEAD_COLOR, FOOD_COLOR, FOOD_SIZE, HEAD_SIZE, HOVERED_BUTTON,
    INITIAL_HEIGHT, INITIAL_WIDTH, NORMAL_BUTTON, PLAYER_BODY_COLOR, PLAYER_HEAD_COLOR,
    PRESSED_BUTTON, SNAKE_SPEED, WORLD_COLOR,
};
use crate::events::{EnemyDieEvent, GrowthEvent};
use crate::resources::{CounterSetting, FoodCenter, IntervalSetting, MenuData, PlayerScore};
use bevy::prelude::*;
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
    ShowScore,
}

pub fn play() {
    App::new()
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
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::Playing), spawn_player)
        .add_systems(
            Update,
            (
                spawn_enemy,
                snake_movement_input.before(snake_movement),
                snake_movement,
                (snake_growth, enemy_die).after(snake_movement),
                food_spawner,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, flash_mask.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), game_over)
        .add_systems(Update, show_score.run_if(in_state(GameState::ShowScore)))
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .insert_resource(ClearColor(WORLD_COLOR))
        .insert_resource(IntervalSetting::<Player>::default())
        .insert_resource(IntervalSetting::<Enemy>::default())
        .insert_resource(IntervalSetting::<Food>::default())
        .insert_resource(IntervalSetting::<FlashMask>::default())
        .insert_resource(CounterSetting::<Enemy>::default())
        .insert_resource(CounterSetting::<FlashMask>::default())
        .insert_resource(FoodCenter::default())
        .insert_resource(PlayerScore::default())
        .add_event::<GrowthEvent>()
        .add_event::<EnemyDieEvent>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_menu(mut commands: Commands) {
    let button_entity = commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}

fn size_scaling(window: Query<&Window>, mut q: Query<(&Size, &mut Transform)>) {
    let window = window.single();
    let window_size = window.width().min(window.height());
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.w() / ARENA_WIDTH as f32 * window_size,
            sprite_size.h() / ARENA_HEIGHT as f32 * window_size - 6.,
            1.0,
        );
    }
}

fn position_translation(window: Query<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = window.single();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x() as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y() as f32, window.height(), ARENA_HEIGHT as f32) - 3.,
            0.0,
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Sprite {
                color: Color::linear_rgba(0.8, 0.8, 0.8, 0.0),
                ..Default::default()
            },
            Transform::from_xyz(0., 0., 1.),
        ))
        .insert(FlashMask)
        .insert(Size::rectangle(INITIAL_WIDTH, INITIAL_HEIGHT));

    let mut snake = Snake::default();
    snake.add(
        commands
            .spawn((
                Mesh2d(meshes.add(Circle::default())),
                MeshMaterial2d(materials.add(PLAYER_HEAD_COLOR)),
            ))
            .insert(Position::new(
                ARENA_WIDTH as i32 / 2,
                ARENA_HEIGHT as i32 / 2,
            ))
            .insert(Size::square(HEAD_SIZE))
            .id(),
    );
    snake.add(
        commands
            .spawn((
                Mesh2d(meshes.add(Circle::default())),
                MeshMaterial2d(materials.add(PLAYER_BODY_COLOR)),
            ))
            .insert(Position::new(
                ARENA_WIDTH as i32 / 2,
                ARENA_HEIGHT as i32 / 2 - 1,
            ))
            .insert(Size::square(BODY_SIZE))
            .id(),
    );
    commands.spawn(snake).insert(Player).insert(Position::new(
        ARENA_WIDTH as i32 / 2,
        ARENA_HEIGHT as i32 / 2,
    ));
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut counter: ResMut<CounterSetting<Enemy>>,
) {
    if !counter.less_than(30) {
        return;
    }

    let mut rng = rand::thread_rng();
    let mut px = rng.gen_range(-1..3);
    let mut py = rng.gen_range(-1..3);
    while (px == 0 || px == 1) && (py == 0 || py == 1) {
        px = rng.gen_range(-1..3);
        py = rng.gen_range(-1..3);
    }
    let x = px * (ARENA_WIDTH as i32 / 2) + rng.gen_range(0..ARENA_WIDTH) as i32;
    let y = (ARENA_HEIGHT as i32 - 4)
        .min(py * (ARENA_HEIGHT as i32 / 2) + rng.gen_range(0..ARENA_HEIGHT) as i32); // 体が下に続くため

    let mut snake = Snake::default();
    snake.add(
        commands
            .spawn((
                Mesh2d(meshes.add(Circle::default())),
                MeshMaterial2d(materials.add(ENEMY_HEAD_COLOR)),
            ))
            .insert(Position::new(x, y))
            .insert(Size::square(HEAD_SIZE))
            .id(),
    );
    for i in 1..4 {
        snake.add(
            commands
                .spawn((
                    Mesh2d(meshes.add(Circle::default())),
                    MeshMaterial2d(materials.add(ENEMY_BODY_COLOR)),
                ))
                .insert(Position::new(x, y - i))
                .insert(Size::square(BODY_SIZE))
                .id(),
        );
    }

    commands
        .spawn(snake)
        .insert(Enemy::default())
        .insert(Position::new(x, y));

    counter.increment();
}

fn spawn_body(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Position,
    scale: Size,
    is_player: bool,
) -> Entity {
    commands
        .spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(if is_player {
                PLAYER_BODY_COLOR
            } else {
                ENEMY_BODY_COLOR
            })),
        ))
        .insert(position)
        .insert(scale)
        .id()
}

fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snakes: Query<(&mut Snake, Entity)>,
    mut enemies: Query<&mut Enemy>,
    pos: Query<&Position, With<Enemy>>,
    center: Res<FoodCenter>,
) {
    let mut rng = rand::thread_rng();
    for (mut snake, ent) in snakes.iter_mut() {
        if let Ok(mut enemy) = enemies.get_mut(ent) {
            if enemy.get() > snake.len() {
                let diff = center.get_pos() - *pos.get(ent).unwrap();
                let food_weight = [
                    if diff.y() > 0 { diff.y() } else { 0 },
                    if diff.x() > 0 { diff.x() } else { 0 },
                    if diff.y() < 0 { -diff.y() } else { 0 },
                    if diff.x() < 0 { -diff.x() } else { 0 },
                ];
                // 自分が向いている方向
                let dir = snake.get_dir().get_num();
                // (今の向きを基準として)次に向く方向の確率
                let dist = WeightedIndex::new(
                    DIRECTION_WEIGHT
                        .iter()
                        .enumerate()
                        .map(|(i, &w)| {
                            if w == 0 {
                                0
                            } else {
                                food_weight[(i + 4 - dir as usize) % 4] + w
                            }
                        })
                        .collect::<Vec<i32>>(),
                )
                .unwrap();
                snake.set_dir(Direction::num2dir(dir + dist.sample(&mut rng) as u8));
                enemy.reset();
            }
        } else {
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
}

fn snake_movement(
    mut snakes: Query<(&mut Snake, Entity)>,
    mut positions: Query<&mut Position>,
    player: Query<&Player>,
    mut enemies: Query<&mut Enemy>,
    foods: Query<Entity, With<Food>>,
    mut enemy_die_writer: EventWriter<EnemyDieEvent>,
    mut growth_writer: EventWriter<GrowthEvent>,
    mut interval: ResMut<IntervalSetting<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut history: ResMut<PlayerScore>,
) {
    let now = Instant::now();
    if !interval.check(now, Duration::from_millis(600 / SNAKE_SPEED)) {
        return;
    }
    interval.update(now);

    let body_pos = snakes
        .iter()
        .map(|(snake, _ent)| {
            snake
                .iter()
                .map(|&body| *positions.get(body).unwrap())
                .collect::<Vec<Position>>()
        })
        .flatten()
        .collect::<Vec<Position>>();

    let food_pos = foods
        .iter()
        .map(|ent| (*positions.get(ent).unwrap(), ent))
        .collect::<Vec<(Position, Entity)>>();

    for (mut snake, snake_ent) in snakes.iter_mut() {
        let tail = *positions.get(*snake.get_tail()).unwrap();
        snake.set_tail_pos(tail);
        let now_pos = snake
            .iter()
            .map(|e| *positions.get(*e).unwrap())
            .collect::<Vec<Position>>();
        let head_pos = *positions.get(*snake.get_head()).unwrap() + snake.get_dir().to_pos(1);
        if let Ok(_) = player.get(snake_ent) {
            if !in_arena(head_pos.x(), head_pos.y()) {
                next_state.set(GameState::GameOver);
                break;
            }
            if body_pos.contains(&head_pos) {
                next_state.set(GameState::GameOver);
                break;
            }
            history.as_mut().add(head_pos);
        } else {
            if !in_enemy_arena(head_pos.x(), head_pos.y()) {
                enemy_die_writer.send(EnemyDieEvent { enemy: snake_ent });
                continue;
            }
            if body_pos.contains(&head_pos) {
                enemy_die_writer.send(EnemyDieEvent { enemy: snake_ent });
                continue;
            }
            // この個体が直進した長さ
            if let Ok(mut enemy) = enemies.get_mut(snake_ent) {
                enemy.increment();
            }
        }
        food_pos.iter().for_each(|&(pos, food_ent)| {
            if head_pos == pos {
                growth_writer.send(GrowthEvent {
                    snake: snake_ent,
                    food: (pos, food_ent),
                });
            }
        });
        *positions.get_mut(*snake.get_head()).unwrap() = head_pos;
        now_pos
            .iter()
            .zip(snake.iter().skip(1))
            .for_each(|(pos, body)| {
                *positions.get_mut(*body).unwrap() = *pos;
            });
    }
}

fn food_spawner(
    mut commands: Commands,
    mut interval: ResMut<IntervalSetting<Food>>,
    mut center: ResMut<FoodCenter>,
) {
    let now = Instant::now();
    if !interval.check(now, Duration::from_secs(1)) {
        return;
    }
    interval.update(now);

    let pos = Position::new(
        (rand::random::<f32>() * ARENA_WIDTH as f32) as i32,
        (rand::random::<f32>() * ARENA_HEIGHT as f32) as i32,
    );

    commands
        .spawn(Sprite {
            color: FOOD_COLOR,
            ..default()
        })
        .insert(Food)
        .insert(pos)
        .insert(Size::square(FOOD_SIZE));

    center.add(&pos);
}

fn snake_growth(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player: Query<&Player>,
    mut player_score: ResMut<PlayerScore>,
    mut snakes: Query<&mut Snake>,
    mut growth_reader: EventReader<GrowthEvent>,
    mut center: ResMut<FoodCenter>,
) {
    for event in growth_reader.read() {
        center.remove(&event.food.0);
        commands.entity(event.food.1).despawn();
        let mut snake = snakes.get_mut(event.snake).unwrap();
        let tail_pos = snake.get_tail_pos();
        snake.add(spawn_body(
            &mut commands,
            &mut meshes,
            &mut materials,
            tail_pos,
            Size::square(BODY_SIZE),
            player.get(event.snake).is_ok(),
        ));
        if player.get(event.snake).is_ok() {
            player_score.increment();
        }
    }
}

fn enemy_die(
    mut commands: Commands,
    snakes: Query<&Snake>,
    mut enemy_die_reader: EventReader<EnemyDieEvent>,
    mut counter: ResMut<CounterSetting<Enemy>>,
) {
    for event in enemy_die_reader.read() {
        snakes.get(event.enemy).unwrap().despawn(&mut commands);
        commands.entity(event.enemy).despawn();
        counter.decrement();
    }
}

fn flash_mask(
    mut mask: Single<&mut Sprite, With<FlashMask>>,
    mut interval: ResMut<IntervalSetting<FlashMask>>,
    mut counter: ResMut<CounterSetting<FlashMask>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let now = Instant::now();
    if !interval.check(now, Duration::from_millis(50)) {
        return;
    }
    interval.update(now);

    if counter.less_than(10) {
        if counter.is_even() {
            mask.color = Color::linear_rgba(0.8, 0.8, 0.8, 0.6);
        } else {
            mask.color = Color::linear_rgba(0.8, 0.8, 0.8, 0.0);
        }
        counter.increment();
        return;
    }

    if counter.less_than(13) {
        counter.increment();
        return;
    }

    next_state.set(GameState::ShowScore);
}

fn game_over(
    mut commands: Commands,
    snakes: Query<&Snake>,
    snake_entities: Query<Entity, With<Snake>>,
    food_entities: Query<Entity, With<Food>>,
    mask: Single<Entity, With<FlashMask>>,
    mut player_interval: ResMut<IntervalSetting<Player>>,
    mut enemy_interval: ResMut<IntervalSetting<Enemy>>,
    mut food_interval: ResMut<IntervalSetting<Food>>,
    mut flash_interval: ResMut<IntervalSetting<FlashMask>>,
    mut enemy_counter: ResMut<CounterSetting<Enemy>>,
    mut flash_counter: ResMut<CounterSetting<FlashMask>>,
) {
    for snake in snakes.iter() {
        snake.despawn(&mut commands);
    }
    for ent in snake_entities.iter().chain(food_entities.iter()) {
        commands.entity(ent).despawn();
    }
    commands.entity(*mask).despawn();
    player_interval.reset();
    enemy_interval.reset();
    food_interval.reset();
    flash_interval.reset();
    enemy_counter.reset();
    flash_counter.reset();
}

fn show_score(player_score: Res<PlayerScore>, mut next_state: ResMut<NextState<GameState>>) {
    let score = player_score.get_score();
    println!("{}", score);
    next_state.set(GameState::Menu);
}
