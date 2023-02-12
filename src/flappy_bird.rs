pub mod flappy_bird{
    use macroquad::prelude::*;
    use ::rand;
    use rand::Rng;
    
    
    const BIRD_HEIGHT: f32 = 25.;//tamanho do bird
    const BIRD_BASE: f32 = 22.;
    struct Bird {
        pos: Vec2,
        vel: Vec2,
    }
    
    struct Pipe {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    }

    struct Trophy {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    }

    fn draw_screen(campo_texture: Texture2D, cruzeiro_texture: Texture2D, galo_logo_texture: Texture2D, bird: &Bird, pipes: &Vec<Pipe>, pontuacao: i64) {
        // Desenha o campo
        draw_texture_ex(
            campo_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        let text = &format!("{}", pontuacao);//Mostrar pontuação
        let font_size = 90.;
        draw_text(
            text,
            screen_width()/2.  - 200.,
            screen_height()/2. - 50.,
            font_size,
            RED,
        );

        // draw_circle_lines(bird.pos.x, bird.pos.y, 41., 2., BLACK);

        // Desenha logo cruzeiro
        draw_texture_ex(
            cruzeiro_texture,
            bird.pos.x - 40.0,
            bird.pos.y - 40.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(80., 80.)),
                ..Default::default()
            },
        );

        let pipes_iter = pipes.iter();//iterar todos os pipes

        for p in pipes_iter {//desenhar os pipes, fazer eles irem paa esquerda e também colisão do pipe com o bird
            // draw_rectangle(p.x, p.y, p.w, p.h, BLACK);

            // Desenha pipe do galo
            draw_texture_ex(
                galo_logo_texture,
                p.x,
                p.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(p.w, p.h)),
                    ..Default::default()
                },
            );
        }
    }
    
    fn death_screen(v: &Vec2) -> bool {//Código para GameOver se tocar embaixo ou emcima da Tela
        let mut life = false;
        if v.y > screen_height() {
            life = true;
        }
        if v.y < 0. {
            life = true;
        }
        life
    }
    
    fn death_pipe(bird: &Bird, p: &mut Pipe) -> bool {//Código de colisão com as pipes, se alguma ponta do triângulo tocar no 'pipe' perde

        let closest_x = clamp(bird.pos.x, p.x, p.x + p.w);
        let closest_y = clamp(bird.pos.y, p.y, p.y + p.h);
        let distance_x = bird.pos.x - closest_x;
        let distance_y = bird.pos.y - closest_y;
        let distance_squared = distance_x * distance_x + distance_y * distance_y;
        distance_squared < 40. * 40.
    }

    fn hit_trophy(bird: &Bird, p: &mut Trophy) -> bool {//Código de colisão com as pipes, se alguma ponta do triângulo tocar no 'pipe' perde

        let closest_x = clamp(bird.pos.x, p.x, p.x + p.w);
        let closest_y = clamp(bird.pos.y, p.y, p.y + p.h);
        let distance_x = bird.pos.x - closest_x;
        let distance_y = bird.pos.y - closest_y;
        let distance_squared = distance_x * distance_x + distance_y * distance_y;
        distance_squared < 40. * 40.
    }

    pub(crate) async fn flappy_bird_game() -> bool{

        // Textures
        let cruzeiro_texture: Texture2D = load_texture("res/cruzeiro.png").await.unwrap();
        let campo_texture: Texture2D = load_texture("res/campo.png").await.unwrap();
        let galo_campeao_texture: Texture2D = load_texture("res/galo_campeao.png").await.unwrap();
        let cruzeiro_campeao_texture: Texture2D = load_texture("res/cruzeiro_campeao.png").await.unwrap();
        let galo_logo_texture: Texture2D = load_texture("res/galo_logo.png").await.unwrap();
        let rounded_box_texture: Texture2D = load_texture("res/rounded_box.png").await.unwrap();
        let trofeu_texture: Texture2D = load_texture("res/trofeu.png").await.unwrap();

        let mut bird = Bird {//Criação da Bird
            pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
            vel: Vec2::new(0., 0.),
        };
        let mut pipes: Vec<Pipe> = vec![//Criação Inicial dos Pipes(duas duplas de pipes)
            Pipe {x: screen_width(), y: 0.0-200.+40., w: 100.0, h: 300.0},
            Pipe {x: screen_width(), y: screen_height()-200.+40., w: 100.0, h: 300.0},
            Pipe {x: screen_width()+250., y: 0.0-200.+40., w: 100.0, h: 300.0},
            Pipe {x: screen_width()+250., y: screen_height()-200.+40., w: 100.0, h: 300.0},
        ];
        let mut trofeu = Trophy {x: screen_width(), y: screen_height()/2.-160.0/2., w: 50., h: 160.};
        let mut rng = rand::thread_rng(); //Para gerar um número randômico
        let mut gameover = false;//gameover = true => fim de jogo
        let mut winner = false;
        let mut paused = false;//paused = true => pausa jogo
        let mut contador = 0;//contador serve para aumentar dificuldade a cada 10 pontos e ajuda na geração de novos pipes, igual pontuação porém zera após aumentar dificuldade, para não aumentar todo frame a dificuldade
        let mut pontuacao = 0;//pontuacao do jogador
        let mut dificuldade = 1.5; //velocidade dos pipes de irem para esquerda
        let pontuacao_max = 4;//quanto maior,menor a distância
        let distancia_pipe = 70.;//quanto maior,menor a distância
        let vel_pipe_baixo = 0.5;//velocidade do pipe de ir para baixo e para cima quando passar de 20/40 pontos
        loop {
            if gameover { //Se perder  o jogo
                
                // Desenha galo campeao
                if winner == false {
                    draw_texture_ex(
                        galo_campeao_texture,
                        0.0,
                        0.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(screen_width(), screen_height())),
                            ..Default::default()
                        },
                    );
                }else{
                    draw_texture_ex(
                        cruzeiro_campeao_texture,
                        0.0,
                        0.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(screen_width(), screen_height())),
                            ..Default::default()
                        },
                    );
                }
        

                // Desenha caixa para texto
                draw_texture_ex(
                    rounded_box_texture,
                    screen_width() / 32. - 20.,
                    screen_height() / 16. - 30.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(500., 130.)),
                        ..Default::default()
                    },
                );

                let text = &format!("Voce fez {} pontos",pontuacao);
                let font_size = 30.;
                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 32.,
                    screen_height() / 16.,
                    font_size,
                    BLACK,
                );
                let text2 = "Aperte [enter] para jogar novamente";
                draw_text(
                    text2,
                    screen_width() / 32.,
                    screen_height() / 16. + 50.,
                    font_size,
                    BLACK,
                );
                let text2 = "Aperte [q] para voltar ao menu";
                draw_text(
                    text2,
                    screen_width() / 32.,
                    screen_height() / 16. + 80.,
                    font_size,
                    BLACK,
                );
                if is_key_down(KeyCode::Enter) {//Após perder o jogo, se apertar enter,reseta as variáveis
                    bird = Bird {
                        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
                        vel: Vec2::new(0., 0.),
                    };
                    pipes = vec![
                        Pipe {x: screen_width(), y: 0.0-200.+40., w: 100.0, h: 300.0},
            Pipe {x: screen_width(), y: screen_height()-200.+40., w: 100.0, h: 300.0},
            Pipe {x: screen_width()+250., y: 0.0-200.+40., w: 100.0, h: 300.0},
            Pipe {x: screen_width()+250., y: screen_height()-200.+40., w: 100.0, h: 300.0},
                    ];
                    rng = rand::thread_rng();
                    trofeu = Trophy {x: screen_width(), y: screen_height()/2.-160.0/2., w: 50., h: 160.};
                    rng = rand::thread_rng(); //Para gerar um número randômico
                    gameover = false;//gameover = true => fim de jogo
                    winner = false;
                    paused = false;//paused = true => pausa jogo
                    contador = 0;//contador serve para aumentar dificuldade a cada 10 pontos e ajuda na geração de novos pipes, igual pontuação porém zera após aumentar dificuldade, para não aumentar todo frame a dificuldade
                    pontuacao = 0;//pontuacao do jogador
                    dificuldade = 1.5; //velocidade dos pipes de irem para esquerda
                }
                if is_key_down(KeyCode::Q) {
                    return false;
                }
                next_frame().await;
                continue;
            }

            // Desenha todos os elementos da tela
            draw_screen(campo_texture, cruzeiro_texture, galo_logo_texture, &bird, &pipes, pontuacao);

            if paused { //Se está pausado
                let text = "PAUSADO";
                let font_size = 60.;
                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 4. - text_size.width / 2.,
                    screen_height() / 5. - text_size.height / 2.,
                    font_size,
                    WHITE,
                );

                let text = &format!("Você está com {} pontos.", pontuacao);
                let font_size = 30.;
                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 4. - text_size.width / 2.,
                    screen_height() * 3. / 4. - text_size.height / 2.,
                    font_size,
                    WHITE,
                );
                let text2 = "Aperte [esc] para continuar";
                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text2,
                    screen_width() / 4. - text_size.width / 2.,
                    screen_height() * 3. / 4. - text_size.height / 2. + 50.,
                    font_size,
                    WHITE,
                );
                let text2 = "Aperte [q] para voltar ao menu";
                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text2,
                    screen_width() / 4. - text_size.width / 2.,
                    screen_height() * 3. / 4. - text_size.height / 2. + 80.,
                    font_size,
                    WHITE,
                );
                if is_key_pressed(KeyCode::Q) {
                    return false;
                }
                if is_key_pressed(KeyCode::Escape) {
                    let mut timer_count = 3;

                    loop {
                        if timer_count == 0 {
                            break;
                        }

                        draw_screen(campo_texture, cruzeiro_texture, galo_logo_texture, &bird, &pipes, pontuacao);


                        let text = &format!("Retomando em {} ...", timer_count);
                        let text_size = measure_text(text, None, font_size as _, 1.0);
                        draw_text(
                            text,
                            screen_width() / 2. - text_size.width / 2.,
                            screen_height() * 3. / 4. - text_size.height / 2.,
                            font_size,
                            WHITE,
                        );

                        next_frame().await;
                        
                        let old = macroquad::time::get_time();

                        loop {
                            let now = macroquad::time::get_time();
                            if now - old >= 1.0 {
                                break;
                            }
                        }

                        timer_count -= 1;
                        
                    }

                    paused = false;
                    continue;
                }

                next_frame().await;
                continue;
            }
    
            let mut acc = -bird.vel / 100.; // Fricçãi

            if is_key_pressed(KeyCode::Escape) {
                paused = true;
                next_frame().await;
                continue;
            }
    
            // Pulo
            if is_key_pressed(KeyCode::Space) {
                acc = Vec2::new(0., -15.);
            }
            
            acc.y += 0.5;//gravidade
            bird.vel += acc;
            if bird.vel.length() > 10. {
                bird.vel = bird.vel.normalize() * 10.;
            }
            bird.pos += bird.vel;
    
            if pontuacao > (pontuacao_max/2) {//1 dos 2 pipes descer
                if pipes[3].y < screen_height()-10.{
                    pipes[2].y += vel_pipe_baixo;
                    pipes[3].y += vel_pipe_baixo;
                }
            }
    
            if pontuacao > ((pontuacao_max*3)/4) {//segundo pipe subir
                if pipes[0].y > -screen_height()+distancia_pipe+120.{
                    pipes[0].y -= vel_pipe_baixo;
                    pipes[1].y -= vel_pipe_baixo;
                }
            }

            let pipes_iter_mut = pipes.iter_mut();//iterar todos os pipes
            let mut gameover_pipes = false;//variável para ajudar em dar o gameover

            for p in pipes_iter_mut {//desenhar os pipes, fazer eles irem paa esquerda e também colisão do pipe com o bird
                p.x = p.x - dificuldade as f32;
                // draw_rectangle(p.x, p.y, p.w, p.h, BLACK);
                // Desenha pipe do galo
                draw_texture_ex(
                    galo_logo_texture,
                    p.x,
                    p.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(p.w, p.h)),
                        ..Default::default()
                    },
                );

                gameover_pipes = death_pipe(&bird ,p);
                if gameover_pipes {
                    break;
                }
            }
    
            if contador % 10 == 0 && contador > 0{//aumentar velocidade dos pipes de irem para a esquerda a cada 10 pontos
                dificuldade = dificuldade + 0.5;
                contador = 0;
            }
    
            //Geração de Novos Pipes, modifica os Pipes Originais para voltarem pro lado direito, funciona bem na tela pequena, tela grande fica ruim
            if (pipes[0].x < bird.pos.x - 80. || pipes[1].x < bird.pos.x - 80. || pipes[2].x < bird.pos.x - 80. || pipes[3].x < bird.pos.x - 80.) && pontuacao <= pontuacao_max {
                if pontuacao < pontuacao_max{
                    let mut valor = rng.gen_range(0..(screen_height() as i64/2)+20);
                    if contador % 2 == 0 {
                        pipes[0] = Pipe {x: screen_width(), y: 0.0 - valor as f32 - distancia_pipe + 40., w: 100.0, h: 300.};
                        pipes[1] = Pipe {x: screen_width(), y: screen_height() - valor as f32 - distancia_pipe + 40., w: 100.0, h: 300.};
                    }else{
                        pipes[2] = Pipe {x: screen_width(), y: 0.0 - valor as f32 - distancia_pipe + 40., w: 100.0, h: 300.};
                        pipes[3] = Pipe {x: screen_width(), y: screen_height() - valor as f32 - distancia_pipe+ 40., w: 100.0, h: 300.};
                    }
                }
                contador+=1;
                pontuacao+=1;
            } 
            if pontuacao >= pontuacao_max{
                trofeu.x -= 0.8;
                draw_texture_ex(
                    trofeu_texture,
                    trofeu.x,
                    trofeu.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(trofeu.w, trofeu.h)),
                        ..Default::default()
                    },
                );
                winner = hit_trophy(&bird ,&mut trofeu);
            }
    
            gameover = death_screen(&bird.pos) || gameover_pipes || winner;//gameover
            next_frame().await
        }
    }
}