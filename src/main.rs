use std::fmt::DebugList;

use macroquad::prelude::*;
// use std::{thread, time};
#[derive(PartialEq)]
pub enum FourSlots{LLTile, LTile, RTile, RRTile}




struct Player{
    x: f32,
    Pos: FourSlots,
    Hit: bool
}


impl Player {
    fn update(&mut self, delta:f32, Pspeed:f32) {

       // let speed:f32 = 10.;

        if is_key_pressed(KeyCode::Right) {
            self.x += Pspeed * delta;
        }
        
        if is_key_pressed(KeyCode::Left) {
            self.x -= Pspeed * delta;
        }


        if self.x < 100. && self.Pos != FourSlots::LLTile  {
            self.x = 0.;
            self.Pos = FourSlots::LLTile
        }
        else if self.x > 0. && self.x < 200. && self.Pos != FourSlots::LTile{
            self.x = 100.;
            self.Pos = FourSlots::LTile
        }
        else if self.x > 100. && self.x < 300. && self.Pos != FourSlots::RTile{
            self.x = 200.;
            self.Pos = FourSlots::RTile
        }
        else if self.x > 200. && self.x < 400. && self.Pos != FourSlots::RRTile{
            self.x = 300.;
            self.Pos = FourSlots::RRTile
        }

        if self.x < 0.{
            self.x = 300.;
            self.Pos = FourSlots::RRTile;
        }
        if self.x > 300.{
            self.x = 0.;
            self.Pos = FourSlots::LLTile;
        }


        if is_mouse_button_down(MouseButton::Right){
            println!("{}", self.x);

        }


        draw_rectangle(self.x, 500.0, 100.0, 100.0, RED);
    }
}

struct Enemy {
    EPos: FourSlots,
    x: f32,
    y: f32
}
impl Enemy {
    fn update(&mut self, delta:f32, speed:f32) {
        self.y += speed * delta;

        if self.x < 100. && self.EPos != FourSlots::LLTile  {
            self.x = 0.;
            self.EPos = FourSlots::LLTile
        }
        else if self.x > 0. && self.x < 200. && self.EPos != FourSlots::LTile{
            self.x = 100.;
            self.EPos = FourSlots::LTile
        }
        else if self.x > 100. && self.x < 300. && self.EPos != FourSlots::RTile{
            self.x = 200.;
            self.EPos = FourSlots::RTile
        }
        else if self.x > 200. && self.x < 400. && self.EPos != FourSlots::RRTile{
            self.x = 300.;
            self.EPos = FourSlots::RRTile
        }


        if self.y < 600.{
       draw_rectangle(self.x, self.y, 100.0, 100.0, BLUE);
        }
        else{
            self.x = 600.;
            self.y = 0.;
            self.x = rand::gen_range(0., 300.);
        }
    }

}




fn gameOver(FinalScore:&str){
    clear_background(WHITE);
    let text = "Game Over";
    let font_size = 30.;
    let text_size = measure_text(FinalScore, None, font_size as _, 1.0);


    

    draw_text(
        FinalScore,
        screen_width() / 2. - text_size.width / 2.,
        screen_height() / 2. - text_size.height / 2.,
        font_size,
        DARKGRAY,);


        draw_text(
        text,
        (screen_width() / 2. - text_size.width / 2.) - 50.,
        (screen_height() / 2. - text_size.height / 2.) -50.,
        font_size,
        DARKGRAY,);


        draw_text(
        "score of:",
        (screen_width() / 2. - text_size.width / 2.) - 50.,
        (screen_height() / 2. - text_size.height / 2.) -25.,
        font_size,
        DARKGRAY,);
            
        

}




#[macroquad::main("Dodge")]
async fn main(){
    let delta = get_frame_time();
    let mut Pspeed = 10.;


    let mut player1 = Player {
        x: 0.0,
        Pos: FourSlots::RRTile,
        Hit: false
    };


    let mut enemy1 = Enemy {
        EPos: FourSlots::LLTile,
        x: rand::gen_range(0., 300.),
        y: 0.0

    };

    let mut enemy2 = Enemy {
        EPos: FourSlots::LTile,
        x: rand::gen_range(0., 300.),
        y: 100.0

    };
    let mut enemy3 = Enemy {
        EPos: FourSlots::RTile,
        x: rand::gen_range(0., 300.),
        y: 200.0

    };
    let mut enemy4 = Enemy {
        EPos: FourSlots::RRTile,
        x: rand::gen_range(0., 300.),
        y: 300.0

    };


    let mut LiveSpeed = 100.;
    let mut score = 0;
    let mut hitCounter = 0.;
    let mut Dead = false;
    
    loop{
        request_new_screen_size(400.0, 600.0);

        clear_background(Color {
            r: 0.25,
            g: 0.85,
            b: 0.45,
            a: 1.0,
        });


        draw_text(
            format!("SCORE: {}", score, ).as_str(),
            10.,
            10.,
            20.,
            DARKGRAY,
        );


        if !Dead{score += 1;}
            

        if LiveSpeed < 400. && !Dead{
        LiveSpeed += 5. * delta;
        }


        player1.update(delta, Pspeed);
        enemy1.update(delta, LiveSpeed);
        enemy2.update(delta, LiveSpeed);
        enemy3.update(delta, LiveSpeed);
        enemy4.update(delta, LiveSpeed);


    if player1.x + 0.0 >= enemy1.x
    && player1.x <= enemy1.x + 0.0
    && 500. + 100.0 >= enemy1.y
    && 500. <= enemy1.y + 100.0
    {
        player1.Hit = true;
    }

    if player1.x + 0.0 >= enemy2.x
        && player1.x <= enemy2.x + 0.0
        && 500. + 100.0 >= enemy2.y
        && 500. <= enemy2.y + 100.0
    {
        player1.Hit = true;
    }

    if player1.x + 0.0 >= enemy3.x
    && player1.x <= enemy3.x + 0.0
    && 500. + 100.0 >= enemy3.y
    && 500. <= enemy3.y + 100.0
    {
        player1.Hit = true;
    }

    if player1.x + 0.0 >= enemy4.x
        && player1.x <= enemy4.x + 0.0
        && 500. + 100.0 >= enemy4.y
        && 500. <= enemy4.y + 100.0
    {
        player1.Hit = true;
    }



    if player1.Hit {
        hitCounter += 2500. * delta;
        if hitCounter >= 4.{
            Dead = true;
            gameOver(&score.to_string());
            LiveSpeed = 0.;
            Pspeed = 0.;
        }
    }

    if !player1.Hit{
        hitCounter = 0.;
    }


        next_frame().await;
       //println!("{}", hitCounter);


    }


}