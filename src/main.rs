use macroquad::prelude::*;
use std::collections::HashSet;

// flappy_bird.rs
mod flappy_bird;
use flappy_bird::flappy_bird::*;

// asteroids.rs
mod asteroids;
use asteroids::asteroids::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui, Skin
};

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

// Window config for macroquad
fn window_conf() -> Conf {
    Conf {
      window_title: "LP Game Emulator".to_owned(),
      window_width: 1000,
      window_height: 600,
      high_dpi: true,
      window_resizable: false,
      ..Default::default()
    }
  }

#[macroquad::main(window_conf)]
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
                    if !asteroids_game().await {
                        break;
                    }
                }
                game_state = GameState::Menu;
            }
            GameState::FlappyBird => {
                clear_background(WHITE);

                let text = "Carregando ...";
                let font_size = 30.;
                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 2. - text_size.height / 2.,
                    font_size,
                    BLACK,
                );

                next_frame().await;

                loop {
                    if !flappy_bird_game().await {
                        break;
                    }
                }
                game_state = GameState::Menu;
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