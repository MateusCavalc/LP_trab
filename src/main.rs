use macroquad::prelude::*;
use std::collections::HashSet;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui, Skin
};

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
const SHIP_LINE_THICKNESS: f32 = 2.;
const SHIP_ROTATION_SPEED: f32 = 5.;
const SHIP_MOVING_SPEED: f32 = 4.;
const SHEILD_SIZE: f32 = 28.;
const SHEILD_LINE_THICKNESS: f32 = 5.;

const ASTEROID_COUNT: u8 = 1;
const ASTEROID_LINE_THICKNESS: f32 = 2.;

const BULLET_SIZE: f32 = 2.;
const BULLET_RANGE: f64 = 0.125;
const BULLET_DELAY: f64 = 0.0;

const BACKGROUND_COLOR: Color = BLACK;
const SHIP_COLOR: Color = LIME;
const SHEILD_COLOR: Color = GOLD;
const ASTEROID_COLOR: Color = LIGHTGRAY;
const BULLET_COLOR: Color = RED;
const TEXT_COLOR: Color = BLACK;

const TEXT_SIZE: u16 = 30;

// pub fn draw_menu() {
//     let text = "< Simulador de jogos - Linguagens de Programação 2023 >";
//     let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);
//     draw_text(
//         text,
//         screen_width() / 2. - text_size.width / 2.,
//         screen_height() / 4. - text_size.height / 2.,
//         TEXT_SIZE,
//         TEXT_COLOR,
//     );
    
//     let text = "Asteroids";
//     let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);
//     let x_midscreen = screen_width() / 2. - text_size.width / 2.;
//     let y_midscreen = screen_height() / 2. - text_size.height / 2.;

//     widgets::Button::new(text)
//         .position(Vec2::new(x_midscreen, y_midscreen))
//         // .size(Vec2::new(150., 100.))
//         .ui(&mut *root_ui());

//     draw_text(
//         text,
//         x_midscreen,
//         y_midscreen,
//         TEXT_SIZE,
//         TEXT_COLOR,
//     );

//     let text = "Flappy Bird";
//     let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);

//     widgets::Button::new(text)
//         .position(Vec2::new(x_midscreen, y_midscreen + (y_midscreen / 4.)))
//         .ui(&mut *root_ui());

//     draw_text(
//         text,
//         x_midscreen,
//         y_midscreen + (y_midscreen / 4.),
//         TEXT_SIZE,
//         TEXT_COLOR,
//     );

// }

pub enum GameState {
    Menu,
    Asteroids,
    FlappyBird
}

struct Ship {
    position: Vec2,
    rotation: f32,
    bullets: Vec<Bullet>,
    has_sheild: bool,
    last_time_shot: f64,
}

impl Ship {
    fn new() -> Ship {
        rand::srand(macroquad::miniquad::date::now() as _);
        Ship {
            position: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rotation: 0.,
            bullets: vec![],
            has_sheild: false,
            last_time_shot: get_time(),
        }
    }

    fn draw(&mut self) {
        let rotation = self.rotation.to_radians();

        let v1 = Vec2::new(
            self.position.x + (rotation.sin() * SHIP_HEIGHT / 2.),
            self.position.y - (rotation.cos() * SHIP_HEIGHT / 2.),
        );

        let v2 = Vec2::new(
            self.position.x
                - (rotation.cos() * SHIP_BASE / 2.)
                - (rotation.sin() * SHIP_HEIGHT / 2.),
            self.position.y - (rotation.sin() * SHIP_BASE / 2.)
                + (rotation.cos() * SHIP_HEIGHT / 2.),
        );

        let v3 = Vec2::new(
            self.position.x + (rotation.cos() * SHIP_BASE / 2.)
                - (rotation.sin() * SHIP_HEIGHT / 2.),
            self.position.y
                + (rotation.sin() * SHIP_BASE / 2.)
                + (rotation.cos() * SHIP_HEIGHT / 2.),
        );

        draw_triangle_lines(v1, v2, v3, SHIP_LINE_THICKNESS, SHIP_COLOR);

        self.bullets.iter_mut().for_each(|b| b.update());
        self.bullets.iter_mut().for_each(|b| b.draw());
        // range of bullets
        self.bullets
            .retain(|b| get_time() - b.time_shot_out < BULLET_RANGE && !b.collided);
    }

    fn mv(&mut self) {
        if !is_key_down(KeyCode::Space) {
            self.has_sheild = false;
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            let rotation = self.rotation.to_radians();

            self.position.y += rotation.cos() * -SHIP_MOVING_SPEED;
            self.position.x += rotation.sin() * SHIP_MOVING_SPEED;
        }

        if is_mouse_button_down(MouseButton::Left) {
            self.shoot();
        }

        if is_key_down(KeyCode::Space) && !is_mouse_button_down(MouseButton::Left) {
            self.has_sheild = true;
            draw_poly_lines(
                self.position.x,
                self.position.y,
                255, // 255 is here to make it a circle since 255 is the max number that this can be
                SHEILD_SIZE,
                self.rotation,
                SHEILD_LINE_THICKNESS,
                SHEILD_COLOR,
            )
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.rotation -= SHIP_ROTATION_SPEED;
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.rotation += SHIP_ROTATION_SPEED;
        }

        // wraping:
        if self.position.x > screen_width() {
            self.position.x = 0.;
        } else if self.position.x < 0. {
            self.position.x = screen_width();
        }

        if self.position.y > screen_height() {
            self.position.y = 0.;
        } else if self.position.y < 0. {
            self.position.y = screen_height();
        }
    }

    fn shoot(&mut self) {
        if get_time() - self.last_time_shot > BULLET_DELAY {
            self.bullets.push(Bullet::new(self.position, self.rotation));
            self.last_time_shot = get_time();
        }
    }
}

#[derive(Copy, Clone)]
struct Bullet {
    position: Vec2,
    rotation: f32,
    time_shot_out: f64,
    collided: bool,
}

impl Bullet {
    fn new(position: Vec2, rotation: f32) -> Bullet {
        Bullet {
            position,
            rotation,
            time_shot_out: get_time(),
            collided: false,
        }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, BULLET_SIZE, BULLET_COLOR);
    }

    fn update(&mut self) {
        // spacing of bullets
        let rotation = self.rotation.to_radians();
        self.position.y += rotation.cos() * rand::gen_range(-30.0, -26.);
        self.position.x += rotation.sin() * rand::gen_range(25.0, 29.);
    }
}

struct Asteroid {
    position: Vec2,
    sides: u8,
    size: f32,
    rotation: f32,
}

impl Asteroid {
    fn new() -> Asteroid {
        Asteroid {
            position: Vec2::new(
                rand::gen_range(35.0, screen_width() - 36.),
                rand::gen_range(35.0, screen_height() - 36.),
            ),
            sides: rand::gen_range(12, 24),
            size: 100.,
            rotation: rand::gen_range(-360.0, 359.),
        }
    }

    fn draw(&self) {
        /* 12 - 25
         * 9 - 12
         * 6 - 9
         * 3 - 6
         */
        draw_poly_lines(
            self.position.x,
            self.position.y,
            self.sides,
            self.size,
            self.rotation,
            ASTEROID_LINE_THICKNESS,
            ASTEROID_COLOR,
        )
    }

    fn collided(&self, position: &Vec2) -> bool {
        if (self.position - *position).length() < self.size {
            return true;
        }
        false
    }

    fn resize(&mut self) -> Option<Asteroid> {
        let sides_range = match self.sides {
            (12..=24) => {
                self.sides = rand::gen_range(9, 11);
                (9, 11)
            }
            (9..=11) => {
                self.sides = rand::gen_range(6, 8);
                (6, 8)
            }
            (6..=8) => {
                self.sides = rand::gen_range(3, 5);
                (3, 5)
            }
            _ => return None,
        };

        let some = Some(Asteroid {
            position: Vec2::new(
                rand::gen_range(
                    self.position.x - (self.size * 4.),
                    self.position.x + (self.size * 4.),
                ),
                rand::gen_range(
                    self.position.y - (self.size * 4.),
                    self.position.y + (self.size * 4.),
                ),
            ),
            sides: rand::gen_range(sides_range.0, sides_range.1),
            size: self.size / 2.,
            rotation: self.rotation * 2.,
        });

        self.size /= 2.;
        self.rotation *= 2.;

        self.position.x = rand::gen_range(
            self.position.x - (self.size * 4.),
            self.position.x + (self.size * 4.),
        );
        self.position.y = rand::gen_range(
            self.position.y - (self.size * 4.),
            self.position.y + (self.size * 4.),
        );

        some
    }

    fn mv(&mut self) {
        let rotation = self.rotation.to_radians();

        match self.sides {
            (12..=24) => {
                let speed = rand::gen_range(4.0, 5.);
                self.position.y += rotation.cos() * -speed;
                self.position.x += rotation.sin() * speed;
            }
            (9..=11) => {
                let speed = rand::gen_range(5.0, 6.);
                self.position.y += rotation.cos() * -speed;
                self.position.x += rotation.sin() * speed;
            }
            (6..=8) => {
                let speed = rand::gen_range(6.0, 7.);
                self.position.y += rotation.cos() * -speed;
                self.position.x += rotation.sin() * speed;
            }
            _ => {
                let speed = rand::gen_range(7.0, 8.);
                self.position.y += rotation.cos() * -speed;
                self.position.x += rotation.sin() * speed;
            }
        }

        // wraping:
        if self.position.x > screen_width() {
            self.position.x = 0.;
        } else if self.position.x < 0. {
            self.position.x = screen_width();
        }

        if self.position.y > screen_height() {
            self.position.y = 0.;
        } else if self.position.y < 0. {
            self.position.y = screen_height();
        }
    }
}

#[macroquad::main("LP Game Emulator")]
async fn main() {

    let texture: Texture2D = load_texture("res/top.png").await.unwrap();

    let mut game_state = GameState::Menu;

    let text = "Asteroids";
    let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);
    let x_midscreen = screen_width() / 2. - text_size.width / 2.;
    let y_midscreen = screen_height() / 2. - text_size.height / 2.;

    let skin = {
        let button_style = root_ui()
        .style_builder()
        .text_color(Color::from_rgba(180, 180, 100, 255))
        .font_size(TEXT_SIZE)
        .build();

        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };

    let skin_schema = skin.clone();

    root_ui().push_skin(&skin_schema);

    loop {
        clear_background(WHITE);
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        match game_state {
            GameState::Menu => {
                let text = "< Simulador de jogos - Linguagens de Programação 2023 >";
                let text_size = measure_text(text, None, TEXT_SIZE, 1.0);
                draw_text(
                    text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 4. - text_size.height / 2.,
                    TEXT_SIZE as f32,
                    TEXT_COLOR,
                );

                // widgets::Button::new(text)
                //     .position(Vec2::new(x_midscreen, y_midscreen))
                //     // .size(Vec2::new(150., 100.))
                //     .ui(&mut *root_ui());

                // draw_text(
                //     text,
                //     x_midscreen,
                //     y_midscreen,
                //     TEXT_SIZE,
                //     TEXT_COLOR,
                // );

                // let text = "Flappy Bird";
                // let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);

                // widgets::Button::new(text)
                //     .position(Vec2::new(x_midscreen, y_midscreen + (y_midscreen / 4.)))
                //     .ui(&mut *root_ui() => );

                // draw_text(
                //     text,
                //     x_midscreen,
                //     y_midscreen + (y_midscreen / 4.),
                //     TEXT_SIZE,
                //     TEXT_COLOR,
                // );
            }
            GameState::Asteroids => {
                loop {
                        if !play().await {
                            break;
                        }
                }
            }
            GameState::FlappyBird => {
                loop {
                    // Flappy Bird
                }
            }
        }

        if root_ui().button(Vec2::new(x_midscreen, y_midscreen), "Asteroids") {
            println!("Play asteroids");
            game_state = GameState::Asteroids;
        }

        if root_ui().button(Vec2::new(x_midscreen, y_midscreen + (y_midscreen / 4.)), "Flappy Bird") {
            println!("Play Flappy Bird");
            game_state = GameState::FlappyBird;
        }

        // widgets::Label::new("abc")
        //         .position(Vec2::new(100., 100.))
        //         .ui(&mut *root_ui());

        // widgets::Window::new(hash!(), vec2(100., 100.), vec2(600., 400.))
        //         .label("Teste")
        //         .titlebar(true)
        //         .ui(&mut *root_ui(), |ui| {
        //             ui.label(Vec2::new(230., 50.), "Ladeira corno");
        //             if ui.button(Vec2::new(260., 100.), "Asteroids") {
        //                 println!("Play asteroids");
        //                 // loop {
        //                 //         if !play().await {
        //                 //             break;
        //                 //         }
        //                 // }
        //             }
        //             if ui.button(Vec2::new(260., 150.), "Flappy Bird") {
        //                 println!("Play Flappy Bird");
        //             }
        //         });

        next_frame().await;
    }

}

async fn play() -> bool {
    let mut ship = Ship::new();
    let mut asteroids: Vec<_> = (0..ASTEROID_COUNT).map(|_| Asteroid::new()).collect();
    let mut did_win = true;

    loop {
        if is_key_down(KeyCode::Escape) {
            return false;
        }

        clear_background(BACKGROUND_COLOR);

        if asteroids.is_empty() && did_win {
            let text = "You Win!. Press [enter] to play again.";
            let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                TEXT_SIZE as f32,
                TEXT_COLOR,
            );

            if is_key_down(KeyCode::Enter) {
                return true;
            }
        } else if asteroids.is_empty() {
            let text = "Game Over. Press [enter] to play again.";
            let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                TEXT_SIZE as f32,
                TEXT_COLOR,
            );

            if is_key_down(KeyCode::Enter) {
                return true;
            }
        }

        let mut indexes_to_remove = HashSet::new();
        let mut asteroids_to_add = vec![];
        for (i, asteroid) in asteroids.iter_mut().enumerate() {
            for bullet in ship.bullets.iter() {
                if asteroid.collided(&bullet.position) {
                    let a = asteroid.resize();
                    match a {
                        Some(asteroid) => asteroids_to_add.push(asteroid),
                        None => {
                            indexes_to_remove.insert(i);
                        }
                    }
                }
            }

            if asteroid.collided(&ship.position) && !ship.has_sheild {
                did_win = false;
            }
        }

        if !did_win {
            asteroids = vec![];
        } else {
            for (num_removed, i) in indexes_to_remove.into_iter().enumerate() {
                asteroids.remove(i - num_removed);
            }
            for i in asteroids_to_add {
                asteroids.push(i);
            }
        }

        ship.draw();
        ship.mv();
        asteroids.iter().for_each(|a| a.draw());
        asteroids.iter_mut().for_each(|a| a.mv());

        next_frame().await;
    }
}
