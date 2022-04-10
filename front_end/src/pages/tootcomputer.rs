use requests::request;
use games::toot::TootAndOttoState;
use yew::prelude::*;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, ResizeEvent};
use stdweb::web::html_element::{CanvasElement, SelectElement};
use stdweb::web::{document, window, CanvasRenderingContext2d, FillRule};
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::json;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;

use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use chrono::Local;
use instant::Instant;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub struct TOOTComputer {
    player1: String,
    player2: String,
    difficulty: i32,
    TO: i32,
    // used to update name
    name_callback: Callback<InputEvent>,
    // used to update diff
    diff_callback: Callback<InputEvent>,
    // used to change T or O
    TO_callback: Callback<InputEvent>,
    // used to start the game
    start_callback: Callback<MouseEvent>,
    // used to end the game
    end_callback: Callback<MouseEvent>,
    // true => start
    // false => end
    start_or_end: bool,
    new_select_col: usize,
    game: Rc<RefCell<TootAndOttoState>>,
    winner: i32,
    is_draw: bool,
    col: usize,
    row: usize,
}

// draw the background for you
fn background(game: Rc<RefCell<TootAndOttoState>>) {
    let canvas: CanvasElement = document()
        .query_selector("#background")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    // log::info!("im here");
    context.save();
    context.set_fill_style_color("#00bfff");
    context.begin_path();
    let rows = game.clone().borrow().rows;
    let cols = game.clone().borrow().cols;
    for y in 0..rows {
        for x in 0..cols {
            context.arc(
                (75 * x + 100) as f64,
                (75 * y + 50) as f64,
                25.0,
                0.0,
                2.0 * PI,
                false,
            );
            context.rect((75 * x + 150) as f64, (75 * y) as f64, -100.0, 100.0);
        }
    }

    context.fill(FillRule::NonZero);
    context.restore();
}
// for player1 draw
fn test_draw(game: Rc<RefCell<TootAndOttoState>>, col: usize, TO: i32) {

    let canvas: CanvasElement = document()
        .query_selector("#background")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context.save();
    context.set_fill_style_color("#ff0051");
    context.begin_path();

    let mut row = 0;
    let temp_h_map = game.borrow().h_map.clone();
    match temp_h_map.get(&col) {
        Some(&number) => row = number,
        _ => println!("Don't have this col"),
    }
    row = row+1;
    let rows = game.clone().borrow().rows;
    if row <=rows {
        game.borrow_mut().h_map.insert(col,row);
        row = rows-row;
        context.arc(
            (75 * col + 100) as f64,
            (75 * row + 50) as f64,
            25.0,
            0.0,
            2.0 * PI,
            false,
        );
        context.fill(FillRule::NonZero);
    
        context.restore();
        let mut text = "T";
        if TO == 1 {
            text = "T";
        }
        else if TO == -1{
            text = "O";
        }
        context.set_font("bold 25px serif");
        // context.fill_text(text, ((col as f64)- 8.5) as f64, ((row as f64) + 8.0) as f64, None);
        context.fill_text(text, (75 * col + 100 -9) as f64, (75 * row + 50 + 8) as f64, None);
        context.restore();
        context.close_path();
    }

}
// draw after we have a winner
fn winner_draw(game: Rc<RefCell<TootAndOttoState>>, winner: i32) {
    let canvas: CanvasElement = document().query_selector("#background")
    .unwrap().unwrap().try_into().unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    // println!("I'mghere");
    // log::info!("im here");
    context.save();
    context.set_fill_style_color("#d000ff");
    context.begin_path();

    context.set_font("bold 25px serif");
    let mut text = "Computer won, click to restart.";
    if winner == -1 {
        text = "You won, click to restart.";
    }
    else if winner == 1 {
        text = "Computer won, click to restart.";
    }
    else if winner == 2 {
        text = "Game is draw";
    }
    context.fill_text(text, (75+100) as f64, (75) as f64, None);
    context.restore();
    

}


// for computer draw
fn computer_draw(game: Rc<RefCell<TootAndOttoState>>, col: usize, TO: i32) {
    let canvas: CanvasElement = document().query_selector("#background")
    .unwrap().unwrap().try_into().unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    // println!("I'mghere");
    // log::info!("im here");
    context.save();
    context.set_fill_style_color("#00ff59");
    context.begin_path();

    let mut row = 0;
    let temp_h_map = game.borrow().h_map.clone();
    match temp_h_map.get(&col) {
        Some(&number) => row = number,
        _ => println!("Don't have this col"),
    }
    row = row+1;
    let rows = game.clone().borrow().rows;
    if row <=rows {
        game.borrow_mut().h_map.insert(col,row);
        row = rows-row;
        context.arc(
            (75 * col + 100) as f64,
            (75 * row + 50) as f64,
            25.0,
            0.0,
            2.0 * PI,
            false,
        );
        context.fill(FillRule::NonZero);
    
        context.restore();
        let mut text = "T";
        if TO == 1 {
            text = "T";
        }
        else if TO == -1{
            text = "O";
        }
        context.set_font("bold 25px serif");
        context.fill_text(text, (75 * col + 100 -9) as f64, (75 * row + 50 + 8) as f64, None);
        context.restore();
        context.close_path();
    }

}

#[derive(Debug)]
pub enum Msg {
    setPlayerName(InputEvent),
    setDifficulty(InputEvent),
    // https://yew.rs/docs/concepts/components/callbacks
    // this link shows how to create callback for clickevent
    StartGame,
    EndGame,
    setTO(InputEvent),
    clicked(Option<usize>),
    record(),
}

impl Component for TOOTComputer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let rows = 6;
        let cols = 7;
        let max_search_depth = 3;

        let game = Rc::new(RefCell::new(TootAndOttoState::new(rows, cols, max_search_depth, true, &"".to_string(), &"Computer".to_string())));

        TOOTComputer {
            player1: "".to_string(),
            player2: "Computer".to_string(),
            difficulty: 1,
            TO: 1,
            name_callback: _ctx.link().callback(|e: InputEvent| Msg::setPlayerName(e)),
            diff_callback: _ctx.link().callback(|e: InputEvent| Msg::setDifficulty(e)),
            TO_callback: _ctx.link().callback(|e: InputEvent| Msg::setTO(e)),
            start_callback: _ctx.link().callback(|_| Msg::StartGame),
            end_callback: _ctx.link().callback(|e| Msg::EndGame),
            start_or_end: false,
            new_select_col: 0,
            game: game.clone(),
            winner: 0,
            is_draw: false,
            col:7,
            row:6,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
        
            Msg::setPlayerName(val) => {

                let mut owned = self.player1.to_owned();
                if val.data().is_some() {
                    let another_owned =  val.data().unwrap().to_owned();
                    owned.push_str(&another_owned);
                    self.player1 = owned;
                }
                else {
                    let mut chars = self.player1.chars();
                    chars.next_back();
                    chars.as_str();
                    self.player1 = chars.as_str().to_string();
                }
            }

            Msg::setDifficulty(val) => {self.difficulty = 1;}
            Msg::setTO(val) => {self.TO = 1;}
            Msg::StartGame => {
                self.start_or_end = true;


                let difficulty_selector: SelectElement = document()
                    .query_selector("#difficulty")
                    .unwrap()
                    .unwrap()
                    .try_into()
                    .unwrap();

                self.difficulty = match difficulty_selector.value().unwrap().as_str() {
                    "easy" => 2,
                    "medium" => 4,
                    "hard" => 5,
                    _ => 1,
                };

                let difficulty_selector: SelectElement = document()
                .query_selector("#board_size")
                .unwrap()
                .unwrap()
                .try_into()
                .unwrap();

                match difficulty_selector.value().unwrap().as_str() {
                    "7x6" => {self.col = 7; self.row = 6;},
                    "7x7" => {self.col = 7; self.row = 7;},
                    "6x4" => {self.col = 6; self.row = 4;},
                _ => {self.col = 7; self.row = 6;},
                };



                self.game = Rc::new(RefCell::new(TootAndOttoState::new(self.row, self.col, self.difficulty, true, &self.player1, &"Computer".to_string())));

                let canvas: CanvasElement = document()
                    .query_selector("#background")
                    .unwrap()
                    .unwrap()
                    .try_into()
                    .unwrap();
                let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                let rect = canvas.get_bounding_client_rect();

                let game_clone = self.game.clone();
                let link = _ctx.link().clone();

                canvas.add_event_listener(enclose!((context) move |event: ClickEvent| {
                    let x_click = event.client_x() - rect.get_left() as i32;
                    let colss = game_clone.clone().borrow().cols;
                    for col in 0..colss {
                        let x_col = 75 * col as i32 + 100;
                            if (x_click - x_col) * (x_click - x_col) <= 25 * 25 {
                                link.send_message(Msg::clicked(Some(col as usize)));
                                return;
                            }   
                    }
                    link.send_message(Msg::clicked(None));
                }));

                background(self.game.clone());
            }
            Msg::EndGame => {self.start_or_end = false;}

            Msg::record() => {

                log::info!("you should be executed after player disc is drawn");
                let (row_to_move, col_to_move, TO_flag) = self.game.clone().borrow_mut().player_2_move(0,1);
                log::info!("TO falg is {} in canvas",TO_flag);
                computer_draw(self.game.clone(), col_to_move as usize, TO_flag);
                self.winner = self.game.clone().borrow().check_winner();
                if self.winner == 1 {
                    winner_draw(self.game.clone(), self.winner);

                    let p1 = self.player1.clone();
                    let p2 = self.player2.clone();
                    let draw = self.is_draw.clone();
                    let mut winner = self.player1.clone();
                    let w_id = self.winner.clone();
                    if w_id == -1 {
                        winner = self.player1.clone();
                    }
                    else if w_id == 1{
                        winner = self.player2.clone();
                    }
                    else if w_id == 2{
                        winner = "".to_string();
                    }
                    let mut diff = "Hard".to_string();
                    if self.difficulty == 2 {
                        diff = "Easy".to_string();
                    }
                    else if self.difficulty == 4 {
                        diff = "Medium".to_string();
                    }
                    else if self.difficulty == 5 {
                        diff = "Hard".to_string();
                    }
                    let now = js_sys::Date::new_0();
                    let date: String = now.to_iso_string().into();
    
                    spawn_local(async move{
                        let resp = req(p1, p2, draw, winner, diff, date).await;
                        log::info!("body = {:#?}", resp);
                    });
                }
                else if self.winner == 2{
                    self.is_draw = true;
                    winner_draw(self.game.clone(), self.winner);

                    let p1 = self.player1.clone();
                    let p2 = self.player2.clone();
                    let draw = self.is_draw.clone();
                    let mut winner = self.player1.clone();
                    let w_id = self.winner.clone();
                    if w_id == -1 {
                        winner = self.player1.clone();
                    }
                    else if w_id == 1{
                        winner = self.player2.clone();
                    }
                    else if w_id == 2{
                        winner = "".to_string();
                    }
                    let mut diff = "Hard".to_string();
                    if self.difficulty == 2 {
                        diff = "Easy".to_string();
                    }
                    else if self.difficulty == 4 {
                        diff = "Medium".to_string();
                    }
                    else if self.difficulty == 5 {
                        diff = "Hard".to_string();
                    }
                    let now = js_sys::Date::new_0();
                    let date: String = now.to_iso_string().into();
    
                    spawn_local(async move{
                        let resp = req(p1, p2, draw, winner, diff, date).await;
                        log::info!("body = {:#?}", resp);
                    });
                }

            }
            Msg::clicked(col) => {

                if self.winner == 0 && self.game.clone().borrow().to_move == -1{
                    let sel_box: SelectElement = document()
                        .query_selector("#TO")
                        .unwrap()
                        .unwrap()
                        .try_into()
                        .unwrap();
                
                    self.TO = match sel_box.value().unwrap().as_str() {
                        "T" => 1,
                        "O" => -1,
                        _ => 1,
                    };
                    log::info!("TO is {}", self.TO);
                    if col.is_some() {
                        let temp_col = col.unwrap() as usize;
                        let temp = self.game.clone().borrow_mut().player_1_move(temp_col, self.TO);
                        if temp.0 == -1 {
                            log::info!("column full");
                        }
                        else {
                            test_draw(self.game.clone(), temp_col, self.TO);
                            self.winner = self.game.clone().borrow().check_winner();
                            if self.winner == -1 {
                                winner_draw(self.game.clone(), self.winner);

                                let p1 = self.player1.clone();
                                let p2 = self.player2.clone();
                                let draw = self.is_draw.clone();
                                let mut winner = self.player1.clone();
                                let w_id = self.winner.clone();
                                if w_id == -1 {
                                    winner = self.player1.clone();
                                }
                                else if w_id == 1{
                                    winner = self.player2.clone();
                                }
                                else if w_id == 2{
                                    winner = "".to_string();
                                }
                                let mut diff = "Hard".to_string();
                                if self.difficulty == 2 {
                                    diff = "Easy".to_string();
                                }
                                else if self.difficulty == 4 {
                                    diff = "Medium".to_string();
                                }
                                else if self.difficulty == 5 {
                                    diff = "Hard".to_string();
                                }
                                let now = js_sys::Date::new_0();
                                let date: String = now.to_iso_string().into();
                
                                spawn_local(async move{
                                    let resp = req(p1, p2, draw, winner, diff, date).await;
                                    log::info!("body = {:#?}", resp);
                                });
                            }
                            else if self.winner == 1 {
                                winner_draw(self.game.clone(), self.winner);

                                let p1 = self.player1.clone();
                                let p2 = self.player2.clone();
                                let draw = self.is_draw.clone();
                                let mut winner = self.player1.clone();
                                let w_id = self.winner.clone();
                                if w_id == -1 {
                                    winner = self.player1.clone();
                                }
                                else if w_id == 1{
                                    winner = self.player2.clone();
                                }
                                else if w_id == 2{
                                    winner = "".to_string();
                                }
                                let mut diff = "Hard".to_string();
                                if self.difficulty == 2 {
                                    diff = "Easy".to_string();
                                }
                                else if self.difficulty == 4 {
                                    diff = "Medium".to_string();
                                }
                                else if self.difficulty == 5 {
                                    diff = "Hard".to_string();
                                }
                                let now = js_sys::Date::new_0();
                                let date: String = now.to_iso_string().into();
                
                                spawn_local(async move{
                                    let resp = req(p1, p2, draw, winner, diff, date).await;
                                    log::info!("body = {:#?}", resp);
                                });
                            }
                            else if self.winner == 0 {
                                // let link = _ctx.link().clone();
                                // link.send_message(Msg::record());
                                // log::info!("you should be executed after player disc is drawn");
                                // let (row_to_move, col_to_move, TO_flag) = self.game.clone().borrow_mut().player_2_move(0,1);
                                // log::info!("TO falg is {} in canvas",TO_flag);
                                // computer_draw(self.game.clone(), col_to_move as usize, TO_flag);
                                // self.winner = self.game.clone().borrow().check_winner();
                                // if self.winner == 1 {
                                //     winner_draw(self.game.clone(), self.winner);

                                //     let p1 = self.player1.clone();
                                //     let p2 = self.player2.clone();
                                //     let draw = self.is_draw.clone();
                                //     let mut winner = self.player1.clone();
                                //     let w_id = self.winner.clone();
                                //     if w_id == -1 {
                                //         winner = self.player1.clone();
                                //     }
                                //     else if w_id == 1{
                                //         winner = self.player2.clone();
                                //     }
                                //     else if w_id == 2{
                                //         winner = "".to_string();
                                //     }
                                //     let mut diff = "Hard".to_string();
                                //     if self.difficulty == 2 {
                                //         diff = "Easy".to_string();
                                //     }
                                //     else if self.difficulty == 4 {
                                //         diff = "Medium".to_string();
                                //     }
                                //     else if self.difficulty == 5 {
                                //         diff = "Hard".to_string();
                                //     }
                                //     let now = js_sys::Date::new_0();
                                //     let date: String = now.to_iso_string().into();
                    
                                //     spawn_local(async move{
                                //         let resp = req(p1, p2, draw, winner, diff, date).await;
                                //         log::info!("body = {:#?}", resp);
                                //     });
                                // }
                                // else if self.winner == 2{
                                //     self.is_draw = true;
                                //     winner_draw(self.game.clone(), self.winner);

                                //     let p1 = self.player1.clone();
                                //     let p2 = self.player2.clone();
                                //     let draw = self.is_draw.clone();
                                //     let mut winner = self.player1.clone();
                                //     let w_id = self.winner.clone();
                                //     if w_id == -1 {
                                //         winner = self.player1.clone();
                                //     }
                                //     else if w_id == 1{
                                //         winner = self.player2.clone();
                                //     }
                                //     else if w_id == 2{
                                //         winner = "".to_string();
                                //     }
                                //     let mut diff = "Hard".to_string();
                                //     if self.difficulty == 2 {
                                //         diff = "Easy".to_string();
                                //     }
                                //     else if self.difficulty == 4 {
                                //         diff = "Medium".to_string();
                                //     }
                                //     else if self.difficulty == 5 {
                                //         diff = "Hard".to_string();
                                //     }
                                //     let now = js_sys::Date::new_0();
                                //     let date: String = now.to_iso_string().into();
                    
                                //     spawn_local(async move{
                                //         let resp = req(p1, p2, draw, winner, diff, date).await;
                                //         log::info!("body = {:#?}", resp);
                                //     });
                                // }
                            }
                            else if self.winner == 2{
                                self.is_draw = true;
                                log::info!("draw {} in canvas1",self.is_draw);
                                winner_draw(self.game.clone(), self.winner);

                                let p1 = self.player1.clone();
                                let p2 = self.player2.clone();
                                let draw = self.is_draw.clone();
                                let mut winner = self.player1.clone();
                                let w_id = self.winner.clone();
                                if w_id == -1 {
                                    winner = self.player1.clone();
                                }
                                else if w_id == 1{
                                    winner = self.player2.clone();
                                }
                                else if w_id == 2{
                                    winner = "".to_string();
                                }
                                let mut diff = "Hard".to_string();
                                if self.difficulty == 2 {
                                    diff = "Easy".to_string();
                                }
                                else if self.difficulty == 4 {
                                    diff = "Medium".to_string();
                                }
                                else if self.difficulty == 5 {
                                    diff = "Hard".to_string();
                                }
                                let now = js_sys::Date::new_0();
                                let date: String = now.to_iso_string().into();
                
                                spawn_local(async move{
                                    let resp = req(p1, p2, draw, winner, diff, date).await;
                                    log::info!("body = {:#?}", resp);
                                });
                            }
                        }
                    }
                }
                else {

                    self.game = Rc::new(RefCell::new(TootAndOttoState::new(self.row, self.col, self.difficulty, true, &self.player1, &"Computer".to_string())));
                    let canvas: CanvasElement = document()
                    .query_selector("#background")
                    .unwrap()
                    .unwrap()
                    .try_into()
                    .unwrap();
                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
                    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                    background(self.game.clone());
                                        
                    self.winner = 0;
                    self.is_draw = false;
                }

                if self.winner == 0 && self.game.clone().borrow().to_move == 1{
                    let link = _ctx.link().clone();
                    link.send_message(Msg::record());
                }
            }
        }
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool{
        unimplemented!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {


        html! {
            <div style="margin-left:30%">
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>
            { if !self.start_or_end {
            html!{
            <div class="col-md-offset-4 col-md-8">
                <div class="col-md-offset-3 col-md-8">
        
                    <input id="textbox1" type="text" placeholder="Your Name" oninput = {&self.name_callback}/>
                    <select id="difficulty" style="margin: 5px">
                        <option selected=true disabled=false value="easy">{"Easy"}</option>
                        <option selected=false disabled=false value="medium">{"Medium"}</option>
                        <option selected=false disabled=false value="hard">{"Hard"}</option>
                    </select>

                    <select id="board_size" style="margin: 5px">
                        <option selected=true disabled=false value="7x6">{"7x6"}</option>
                        <option selected=false disabled=false value="7x7">{"7x7"}</option>
                        <option selected=false disabled=false value="6x4">{"6x4"}</option>
                    </select>
                    <button
                        id="startbutton" 
                        onclick={&self.start_callback}
                        disabled={false} 
                        title="Start Game">
                        { "Start Game" }
                    </button>
                </div>
                <p>{format!("New Game:  {} Vs Computer",self.player1)}</p>
            </div>
            }
            }
            else {
            html!{
            <div class="post" ng-repeat="game in games">
                <br/>
                <p>{format!("diff:  {} Vs Computer",self.difficulty)}</p>
                <h4>{format!("New Game:  {} Vs Computer",self.player1)}</h4>
                <small>{format!("(Winning Combination: {} - ", self.player1)} <b>{"TOOT"}</b> {"   and    Computer - "} <b>{"OTTO)"}</b></small>
                <br/>
                {"Select a Disc Type:  "}
                <select id="TO" style="margin: 5px" >
                <option selected=true disabled=false value="T">{"T"}</option>
                <option selected=false disabled=false value="O">{"O"}</option>
                </select>
            </div> 
            }}
            }
            <canvas id="background" height="760" width="640"></canvas>
            </div>
        }
    }
}
use serde::Serialize;
use wasm_bindgen::closure::Closure;
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub game_type: String,
    pub p1_name: String,
    pub p2_name: String,
    pub is_draw: bool,
    pub winner_name: String,
    pub difficulty: String,
    pub date_time: String,
}
async fn req(p1: String, p2: String, draw: bool, winner: String, difficulty: String, date_time: String){
    use reqwest::header::CONTENT_TYPE;
    use reqwest::header::ACCEPT;
    use reqwest::header::AUTHORIZATION;
    use reqwest::RequestBuilder;

    let game_type = "TootAndOtto".to_string();
    let p1_name = p1;
    let p2_name = p2;
    let is_draw = draw;
    let winner_name = winner;
    let difficulty = difficulty;
    let date_time = date_time;

    let data = JsValue::from_serde(&Branch {
        game_type,
        p1_name,
        p2_name,
        is_draw,
        winner_name,
        difficulty,
        date_time,
    })
    .unwrap();
    let request = web_sys::Request::new_with_str_and_init(
        "/api/gameresults",
        web_sys::RequestInit::new()
            .body(Some(js_sys::JSON::stringify(&data).unwrap().as_ref()))
            .method("POST"),
    ).unwrap();
    request.headers()
        .set("Content-Type", "application/json")
        .unwrap();
    let window = web_sys::window().unwrap();
    window.fetch_with_request(&request);

}