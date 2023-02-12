pub mod asteroids{
    use macroquad::prelude::*;
    use std::collections::HashSet;
    use macroquad::rand::gen_range;

    const SHIP_HEIGHT: f32 = 37.;
    const SHIP_BASE: f32 = 30.;
    const SHIP_LINE_THICKNESS: f32 = 2.;
    const SHIP_ROTATION_SPEED: f32 = 5.;
    const SHIP_MOVING_SPEED: f32 = 4.;
    const SHIELD_SIZE: f32 = 28.;
    const SHIELD_LINE_THICKNESS: f32 = 5.;

    const ASTEROID_COUNT: u8 = 3;
    const ASTEROID_LINE_THICKNESS: f32 = 2.;

    const BULLET_SIZE: f32 = 5.;
    const BULLET_RANGE: f64 = 1.0;
    const BULLET_DELAY: f64 = 0.1;

    const BACKGROUND_COLOR: Color = WHITE;
    const SHIP_COLOR: Color = LIME;
    const SHIELD_COLOR: Color = GOLD;
    const ASTEROID_COLOR: Color = LIGHTGRAY;
    const BULLET_COLOR: Color = RED;

    pub const TEXT_COLOR: Color = BLACK;
    pub const TEXT_SIZE: u16 = 30;   
    
    struct Ship {
        position: Vec2,
        rotation: f32,
        bullets: Vec<Bullet>,
        has_shield: bool,
        last_time_shot: f64,
    }
    
    impl Ship {
        fn new() -> Ship {
            rand::srand(macroquad::miniquad::date::now() as _);
            Ship {
                position: Vec2::new(screen_width() / 2., screen_height() / 2.),
                rotation: 0.,
                bullets: vec![],
                has_shield: false,
                last_time_shot: get_time(),
            }
        }
    
        fn draw(&mut self, ship_logo: Texture2D, bullet_logo: Texture2D) {
            let rotation = self.rotation.to_radians();
    
            //let v1 = Vec2::new(
            //    self.position.x + (rotation.sin() * SHIP_HEIGHT / 2.),
            //    self.position.y - (rotation.cos() * SHIP_HEIGHT / 2.),
            //);
    
            //let v2 = Vec2::new(
            //    self.position.x
            //        - (rotation.cos() * SHIP_BASE / 2.)
            //        - (rotation.sin() * SHIP_HEIGHT / 2.),
            //    self.position.y - (rotation.sin() * SHIP_BASE / 2.)
            //        + (rotation.cos() * SHIP_HEIGHT / 2.),
            //);
    
            //let v3 = Vec2::new(
            //    self.position.x + (rotation.cos() * SHIP_BASE / 2.)
            //        - (rotation.sin() * SHIP_HEIGHT / 2.),
            //    self.position.y
            //        + (rotation.sin() * SHIP_BASE / 2.)
            //        + (rotation.cos() * SHIP_HEIGHT / 2.),
            //);
    
            //draw_triangle_lines(v1, v2, v3, SHIP_LINE_THICKNESS, SHIP_COLOR);
            draw_texture_ex(
                ship_logo,
                self.position.x - 25.,
                self.position.y - 25.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(50., 62.5)),
                    rotation: self.rotation.to_radians(),
                    ..Default::default()
                },
            );
    
            self.bullets.iter_mut().for_each(|b| b.update());
            self.bullets.iter_mut().for_each(|b| b.draw(bullet_logo));
            // range of bullets
            self.bullets
                .retain(|b| get_time() - b.time_shot_out < BULLET_RANGE && !b.collided);
        }
    
        fn mv(&mut self) {
            if !is_key_down(KeyCode::Z) {
                self.has_shield = false;
            }
    
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                let rotation = self.rotation.to_radians();
    
                self.position.y += rotation.cos() * -SHIP_MOVING_SPEED;
                self.position.x += rotation.sin() * SHIP_MOVING_SPEED;
            }
    
            if is_key_pressed(KeyCode::X) {
                self.shoot();
            }
    
            if is_key_down(KeyCode::Z) && !is_key_pressed(KeyCode::X) {
                self.has_shield = true;
                draw_poly_lines(
                    self.position.x,
                    self.position.y,
                    255, // 255 is here to make it a circle since 255 is the max number that this can be
                    SHIELD_SIZE,
                    self.rotation,
                    SHIELD_LINE_THICKNESS,
                    SHIELD_COLOR,
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


                // let bullet_x = (self.position.x - 23.) + (self.rotation.sin() * 25.);
                // let bullet_y = self.position.y - (self.rotation.cos() * 25.);

                // let bullet_pos = Vec2::new(bullet_x, bullet_y);

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
    
        fn draw(&mut self, logo: Texture2D) {
            // draw_circle(self.position.x + SHIP_BASE/2., self.position.y + SHIP_HEIGHT/2., BULLET_SIZE, BULLET_COLOR);

            draw_texture_ex(
                logo,
                self.position.x + SHIP_BASE/2. - (5.),
                self.position.y + SHIP_HEIGHT/2. - (5.),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(10., 10.)),
                    rotation: self.rotation.to_radians(),
                    ..Default::default()
                },
            );

        }
    
        fn update(&mut self) {
            // spacing of bullets
            let rotation = self.rotation.to_radians();
            self.position.y += rotation.cos() * rand::gen_range(-10., -5.);
            self.position.x += rotation.sin() * rand::gen_range(5., 10.);
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
                   // rand::gen_range(35.0, screen_width() - 36.),
                   // rand::gen_range(35.0, screen_height() - 36.),
                   0.,
                   0.
                ),
                sides: rand::gen_range(12, 24),
                size: 100.,
                rotation: rand::gen_range(-360.0, 359.),
            }
        }
    
        fn draw(&self, logo: Texture2D) {
            /* 12 - 25
             * 9 - 12
             * 6 - 9
             * 3 - 6
             */
            //draw_poly_lines(
            //    self.position.x,
            //    self.position.y,
            //    self.sides,
            //    self.size,
            //    self.rotation,
            //    ASTEROID_LINE_THICKNESS,
            //    ASTEROID_COLOR,
            //)
    
            draw_texture_ex(
                logo,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(self.size, self.size)),
                    rotation: self.rotation.to_radians(),
                    ..Default::default()
                },
            );
    
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
                position: self.position,
                sides: rand::gen_range(sides_range.0, sides_range.1),
                size: self.size / 2.,
                rotation: self.rotation * 3.,
            });
    
            self.size /= 2.;
            self.rotation *= 2.;
    
            // self.position.x = rand::gen_range(
            //     self.position.x - (self.size * 4.),
            //     self.position.x + (self.size * 4.),
            // );
            // self.position.y = rand::gen_range(
            //     self.position.y - (self.size * 4.),
            //     self.position.y + (self.size * 4.),
            // );
    
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

    pub(crate) async fn asteroids_game() -> bool{
        let bg_texture: Texture2D = load_texture("res/asteroids_bg.png").await.unwrap();
        let win_bg_texture: Texture2D = load_texture("res/asteroids_lose_bg.png").await.unwrap();
        let lose_bg_texture: Texture2D = load_texture("res/asteroids_win_bg.png").await.unwrap();
        let bullet_texture: Texture2D = load_texture("res/bola.png").await.unwrap();
        let cruzeiro_logo: Texture2D = load_texture("res/cruzeiro-logo.png").await.unwrap();
        let atletico_logo: Texture2D = load_texture("res/atletico-logo.png").await.unwrap();
        
        let mut ship = Ship::new();
        let mut asteroids: Vec<_> = (0..ASTEROID_COUNT).map(|_| Asteroid::new()).collect();
        let mut did_win = true;

        loop {
            if is_key_down(KeyCode::Escape) {
                return false;
            }

            clear_background(BACKGROUND_COLOR);

            // Desenha o fundo
            draw_texture_ex(
                bg_texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );

            if asteroids.is_empty() && did_win {

                // Desenha o fundo de vitoria
                draw_texture_ex(
                    win_bg_texture,
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_width(), screen_height())),
                        ..Default::default()
                    },
                );

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

                // Desenha o fundo de derrota
                draw_texture_ex(
                    lose_bg_texture,
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_width(), screen_height())),
                        ..Default::default()
                    },
                );

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
            let mut bullets_to_remove = vec![];
            for (i, asteroid) in asteroids.iter_mut().enumerate() {
                for (b, bullet) in ship.bullets.iter().enumerate() {
                    if asteroid.collided(&bullet.position) {
                        println!("collided");
                        bullets_to_remove.push(b);
                        let a = asteroid.resize();
                        match a {
                            Some(asteroid) => asteroids_to_add.push(asteroid),
                            None => {
                                indexes_to_remove.insert(i);
                            }
                        }
                    }
                }

                if asteroid.collided(&ship.position) && !ship.has_shield {
                    did_win = false;
                }
            }

            if !did_win {
                asteroids = vec![];
            } else {
                for (num_removed, i) in indexes_to_remove.into_iter().enumerate() {
                    asteroids.remove(i - num_removed);
                }
                for (num_removed, i) in bullets_to_remove.into_iter().enumerate() {
                    ship.bullets.remove(i - num_removed);
                }
                for i in asteroids_to_add {
                    asteroids.push(i);
                }
            }

            ship.draw(cruzeiro_logo, bullet_texture);
            ship.mv();
            asteroids.iter().for_each(|a| a.draw(atletico_logo));
            asteroids.iter_mut().for_each(|a| a.mv());

            next_frame().await;
        }
    }
}