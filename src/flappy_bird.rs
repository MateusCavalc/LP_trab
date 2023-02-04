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
    
    fn death_pipe(v1: &Vec2,v2: &Vec2,v3: &Vec2, p: &mut Pipe) -> bool {//Código de colisão com as pipes, se alguma ponta do triângulo tocar no 'pipe' perde
        let mut life = false;
    
        if v1.x > p.x && v1.x < p.x + p.w && v1.y > p.y && v1.y < p.y + p.h {
            life = true;
        }else if v2.x > p.x && v2.x < p.x + p.w && v2.y > p.y && v2.y < p.y + p.h {
            life = true;
        }else if v3.x > p.x && v3.x < p.x + p.w && v3.y > p.y && v3.y < p.y + p.h {
            life = true;
        }
        life
    }
    pub(crate) async fn flappy_bird_game() -> bool{
        let mut bird = Bird {//Criação da Bird
            pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
            vel: Vec2::new(0., 0.),
        };
        let mut pipes: Vec<Pipe> = vec![//Criação Inicial dos Pipes(duas duplas de pipes)
            Pipe {x: screen_width(), y: 0.0, w: 50.0, h: 200.0},
            Pipe {x: screen_width(), y: screen_height()-75., w: 50.0, h: 75.0},
            Pipe {x: screen_width()+200., y: 0.0, w: 50.0, h: 200.0},
            Pipe {x: screen_width()+200., y: screen_height()-75., w: 50.0, h: 75.0},
        ];
        let mut rng = rand::thread_rng(); //Para gerar um número randômico
        let mut gameover = false;//gameover = true => fim de jogo
        let mut contador = 0;//contador serve para aumentar dificuldade a cada 10 pontos e ajuda na geração de novos pipes, igual pontuação porém zera após aumentar dificuldade, para não aumentar todo frame a dificuldade
        let mut pontuacao = 0;//pontuacao do jogador
        let mut dificuldade = 1.5; //velocidade dos pipes de irem para esquerda
        let distancia_pipe = 350.;//quanto maior,menor a distância
        let vel_pipe_baixo = 0.5;//velocidade do pipe de ir para baixo e para cima quando passar de 20/40 pontos
        loop {
            if gameover { //Se perder  o jogo
                clear_background(LIGHTGRAY);
                let text = &format!("Voce fez {} pontos.",pontuacao);
                let font_size = 30.;
                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 2. - text_size.height / 2.,
                    font_size,
                    DARKGRAY,
                );
                let text2 = "Aperte [enter] para jogar novamente";
                draw_text(
                    text2,
                    screen_width() / 2. - text_size.width,
                    screen_height() / 2. - text_size.height / 2. + 50.,
                    font_size,
                    DARKGRAY,
                );
                if is_key_down(KeyCode::Enter) {//Após perder o jogo, se apertar enter,reseta as variáveis
                    bird = Bird {
                        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
                        vel: Vec2::new(0., 0.),
                    };
                    pipes = vec![
                        Pipe {x: screen_width(), y: 0.0, w: 50.0, h: 200.0},
                        Pipe {x: screen_width(), y: screen_height()-75., w: 50.0, h: 75.0},
                        Pipe {x: screen_width()+200., y: 0.0, w: 50.0, h: 200.0},
                        Pipe {x: screen_width()+200., y: screen_height()-75., w: 50.0, h: 75.0},
                    ];
                    rng = rand::thread_rng();
                    gameover = false;
                    contador = 0;
                    pontuacao = 0;
                    dificuldade = 1.5;
                }
                if is_key_down(KeyCode::Escape) {
                    return false;
                }
                next_frame().await;
                continue;
            }
    
            let mut acc = -bird.vel / 100.; // Fricçãi
    
            // Pulo
            if is_key_down(KeyCode::Space) {
                acc = Vec2::new(0., -15.);
            }
            
            acc.y += 0.5;//gravidade
            bird.vel += acc;
            if bird.vel.length() > 5. {
                bird.vel = bird.vel.normalize() * 5.;
            }
            bird.pos += bird.vel;
    
            if pontuacao > 20 {//1 dos 2 pipes descer
                if pipes[3].y < screen_height(){
                    pipes[2].h += vel_pipe_baixo;
                    pipes[3].h += vel_pipe_baixo;
                    pipes[3].y += vel_pipe_baixo;
                }
            }
    
            if pontuacao > 40 {//segundo pipe subir
                if pipes[0].h > 0.{
                    pipes[0].h -= vel_pipe_baixo;
                    pipes[1].h += vel_pipe_baixo;
                    pipes[1].y -= vel_pipe_baixo;
                }
            }
    
    
    
            clear_background(LIGHTGRAY);
    
            let text = &format!("{}",pontuacao);//Mostrar pontuação
            let font_size = 60.;
            draw_text(
                text,
                screen_width()/2.  - 150.,
                screen_height()/2. - 150.,
                font_size,
                RED,
            );
    
            //3 pontas do triângulo do Bird
            let v1 = Vec2::new(
                bird.pos.x - BIRD_HEIGHT / 2. ,
                bird.pos.y  + BIRD_BASE / 2. ,
            );
            let v2 = Vec2::new(
                bird.pos.x -  BIRD_HEIGHT / 2. ,
                bird.pos.y -  BIRD_BASE / 2. ,
            );
            let v3 = Vec2::new(
                bird.pos.x + BIRD_HEIGHT / 2.,
                bird.pos.y ,
            );
    
            draw_triangle_lines(v1, v2, v3, 2., BLACK);
    
            let pipes_iter = pipes.iter_mut();//iterar todos os pipes
            let mut gameover_pipes = false;//variável para ajudar em dar o gameover
    
            if contador % 10 == 0 && contador > 0{//aumentar velocidade dos pipes de irem para a esquerda a cada 10 pontos
                dificuldade = dificuldade + 0.5;
                contador = 0;
            }
    
            for p in pipes_iter {//desenhar os pipes, fazer eles irem paa esquerda e também colisão do pipe com o bird
                p.x = p.x - dificuldade as f32;
                draw_rectangle(p.x, p.y, p.w, p.h, BLACK);
                gameover_pipes = death_pipe(&v1,&v2,&v3,p);
                if gameover_pipes {
                    break;
                }
            }
            //Geração de Novos Pipes, modifica os Pipes Originais para voltarem pro lado direito, funciona bem na tela pequena, tela grande fica ruim
            if pipes[0].x < bird.pos.x - 80. || pipes[1].x < bird.pos.x - 80. || pipes[2].x < bird.pos.x - 80. || pipes[3].x < bird.pos.x - 80. {
                let valor = rng.gen_range(0..((screen_height() as i64/2)-50));
                if contador % 2 == 0 {
                    pipes[0] = Pipe {x: screen_width(), y: 0.0, w: 50.0, h: valor as f32};
                    pipes[1] = Pipe {x: screen_width(), y: screen_height() + valor as f32-distancia_pipe, w: 50.0, h: -valor as f32 + distancia_pipe};
                }else{
                    pipes[2] = Pipe {x: screen_width(), y: 0.0, w: 50.0, h: valor as f32};
                    pipes[3] = Pipe {x: screen_width(), y: screen_height() + valor as f32-distancia_pipe, w: 50.0, h: -valor as f32 + distancia_pipe};
                }
                contador+=1;
                pontuacao+=1;
            }
    
            gameover = death_screen(&bird.pos) || gameover_pipes;//gameover
            next_frame().await
        }
    }
    }