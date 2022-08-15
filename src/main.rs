use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::ffi::GetRandomValue;
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

#[derive(Clone, Copy)]
struct Ball {
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color,
}

struct State {
    frame: i32,
    value: i32,
    ball: Ball,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Raylib App")
        .vsync()
        .build();

    let mut state = State {
        ball: Ball {
            position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            speed: 3.0,
            radius: 40.0,
            color: Color::GREEN,
        },
        frame: 0,
        value: get_random_value(-100, 100),
    };

    while !rl.window_should_close() {
        update(&rl, &mut state);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        draw(&mut d, &state);
    }
}

fn update(rl: &RaylibHandle, state: &mut State) {
    state.frame = (state.frame + 1) % 60;
    if state.frame == 0 {
        state.value = get_random_value(-100, 100);
    }

    if rl.is_key_down(KEY_RIGHT) {
        state.ball.position.x += state.ball.speed;
    } else if rl.is_key_down(KEY_LEFT) {
        state.ball.position.x -= state.ball.speed;
    } else if rl.is_key_down(KEY_DOWN) {
        state.ball.position.y += state.ball.speed;
    } else if rl.is_key_down(KEY_UP) {
        state.ball.position.y -= state.ball.speed;
    }

    if rl.is_key_pressed(KEY_SPACE) {
        state.ball.color = if state.ball.color == Color::GREEN {
            Color::YELLOW
        } else {
            Color::GREEN
        }
    }

    if rl.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
        state.ball.position = state.ball.position.lerp(rl.get_mouse_position(), 0.1)
    }

    if rl.is_mouse_button_pressed(MOUSE_RIGHT_BUTTON) {
        state.ball.color = if state.ball.color == Color::GREEN {
            Color::RED
        } else {
            Color::GREEN
        }
    }
}

fn draw(d: &mut RaylibDrawHandle, state: &State) {
    draw_text_center(d, format!("{}", state.value).as_str(), 10, 32, Color::WHITE);
    d.draw_circle_v(state.ball.position, state.ball.radius, state.ball.color);
}

fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color) {
    let text_left = measure_text(text, font_size) / 2;
    d.draw_text(
        text,
        (SCREEN_WIDTH as i32) / 2 - text_left,
        y,
        font_size,
        color,
    )
}
