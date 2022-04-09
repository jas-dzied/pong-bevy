use bevy::prelude::*;

const PLAYER1_WIDTH: f32 = 20.0;
const PLAYER1_HEIGHT: f32 = 100.0;
const PLAYER1_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const PLAYER1_SPEED: f32 = 5.0;

const PLAYER2_WIDTH: f32 = 20.0;
const PLAYER2_HEIGHT: f32 = 100.0;
const PLAYER2_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const PLAYER2_SPEED: f32 = 5.0;

const BALL_SIZE: f32 = 10.0;
const BALL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);


#[derive(Component)]
struct Player1;
#[derive(Component)]
struct Player2;
#[derive(Component)]
struct Ball;

#[derive(Component, Clone, Copy, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player1_movement)
        .add_system(player2_movement)
        .add_system(apply_velocity)
        .add_system(detect_collisions)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

fn setup(
    windows: Res<Windows>,
    mut commands: Commands
) {
    let window = windows.get_primary().unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.
        spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER1_COLOR,
                custom_size: Some(Vec2::new(PLAYER1_WIDTH, PLAYER1_HEIGHT)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-((window.width()/2.0)-PLAYER1_WIDTH/2.0), 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player1);

    commands.
        spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER2_COLOR,
                custom_size: Some(Vec2::new(PLAYER2_WIDTH, PLAYER2_HEIGHT)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(window.width()/2.0-PLAYER2_WIDTH/2.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player2);

    commands.
        spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Velocity {
            x: -2.0,
            y: 2.0,
        })
        .insert(Ball);
}

fn player1_movement(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut player1_positions: Query<(&Player1, &mut Transform)>
) {
    let window = windows.get_primary().unwrap();
    for (_, mut transform) in player1_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) && transform.translation.y < (window.height()/2.0-PLAYER1_HEIGHT/2.0) {
            transform.translation.y += PLAYER1_SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) && transform.translation.y > -(window.height()/2.0-PLAYER1_HEIGHT/2.0) {
            transform.translation.y -= PLAYER1_SPEED;
        }
    }
}

fn player2_movement(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut player2_positions: Query<(&Player2, &mut Transform)>
) {
    let window = windows.get_primary().unwrap();
    for (_, mut transform) in player2_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) && transform.translation.y < (window.height()/2.0-PLAYER2_HEIGHT/2.0) {
            transform.translation.y += PLAYER2_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) && transform.translation.y > -(window.height()/2.0-PLAYER2_HEIGHT/2.0) {
            transform.translation.y -= PLAYER2_SPEED;
        }
    }
}

fn apply_velocity(
    mut entities: Query<(&Velocity, &mut Transform)>
) {
    for (vel, mut transform) in entities.iter_mut() {
        transform.translation.x += vel.x;
        transform.translation.y += vel.y;
    }
}

fn detect_collisions(
    player1: Query<(&Player1, &Transform)>,
    player2: Query<(&Player2, &Transform)>,
    mut balls: Query<(&Ball, &Transform, &mut Velocity)>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    for (_, transform, mut velocity) in balls.iter_mut() {
        for (_, paddle) in player1.iter() {
            if transform.translation.x < PLAYER1_WIDTH-window.width()/2.0 {
                let paddle_upper = paddle.translation.y+PLAYER1_HEIGHT/2.0;
                let paddle_lower = paddle.translation.y-PLAYER1_HEIGHT/2.0;
                if transform.translation.y > paddle_lower && transform.translation.y < paddle_upper {
                    velocity.x *= -1.0;
                }
            }
        }
        for (_, paddle) in player2.iter() {
            if transform.translation.x > window.width()/2.0-PLAYER2_WIDTH {
                let paddle_upper = paddle.translation.y+PLAYER2_HEIGHT/2.0;
                let paddle_lower = paddle.translation.y-PLAYER2_HEIGHT/2.0;
                if transform.translation.y > paddle_lower && transform.translation.y < paddle_upper {
                    velocity.x *= -1.0;
                }
            }
        }
        if transform.translation.y-BALL_SIZE/2.0 < -(window.height()/2.0) || transform.translation.y+BALL_SIZE/2.0 > window.height()/2.0 {
            velocity.y *= -1.0;
        }
    }
}
