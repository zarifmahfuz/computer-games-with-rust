use yew::prelude::*;
// use yew::{html, Html, ChangeData};
// use yew::components::Select;
// use yew::html::InputData;
// use yew::events::ClickEvent;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, ResizeEvent};
use stdweb::web::html_element::{CanvasElement, SelectElement};
use stdweb::web::{document, window, CanvasRenderingContext2d, FillRule};
use web_sys::Document;
use crate::pages::toot::*;
use std::collections::HashMap;

// https://stackoverflow.com/questions/57547849/rust-adding-event-listeners-to-a-webassembly-game
// This reference teach you how to implement this macro rulle and how to add listener to a canvas.
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
    difficulty: String,
    TO: String,
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
    game: Rc<RefCell<Game>>,
}

// draw the background for you
fn background(game: Rc<RefCell<Game>>) {
    let canvas: CanvasElement = document()
        .query_selector("#background")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    log::info!("im here");
    context.save();
    context.set_fill_style_color("#00bfff");
    context.begin_path();
    let rows = game.clone().borrow().num_row;
    let cols = game.clone().borrow().num_col;
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
// for player draw
fn test_draw(game: Rc<RefCell<Game>>, col: usize) {

    let canvas: CanvasElement = document()
        .query_selector("#background")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    log::info!("im here {}", col);
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
    let rows = game.clone().borrow().num_row;
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
    }
    context.restore();
}
// for computer draw
fn computer_draw(game: Rc<RefCell<Game>>, col: usize) {
    let canvas: CanvasElement = document().query_selector("#background")
    .unwrap().unwrap().try_into().unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    // println!("I'mghere");
    log::info!("im here");
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
    // row = 6-row;
    let rows = game.clone().borrow().num_row;
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
    }
    context.restore();
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
}

impl Component for TOOTComputer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        let game = Rc::new(RefCell::new(Game {
            p1: "".to_string(),
            p2: "".to_string(),
            play_with_computer: false,
            winner: "".to_string(),
            total_move: 0,
            max_ai_depth: 4,
            h_map: HashMap::new(),
            num_row:6,
            num_col:7,
        }));

        TOOTComputer {
            player1: "".to_string(),
            player2: "Computer".to_string(),
            difficulty: "".to_string(),
            TO: "".to_string(),
            name_callback: _ctx.link().callback(|e: InputEvent| Msg::setPlayerName(e)),
            diff_callback: _ctx.link().callback(|e: InputEvent| Msg::setDifficulty(e)),
            TO_callback: _ctx.link().callback(|e: InputEvent| Msg::setTO(e)),
            start_callback: _ctx.link().callback(|_| Msg::StartGame),
            end_callback: _ctx.link().callback(|e| Msg::EndGame),
            start_or_end: false,
            new_select_col: 0,
            game: game.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
        
            Msg::setPlayerName(val) => {
                let mut owned = self.player1.to_owned();
                let another_owned =  val.data().unwrap().to_owned();
                owned.push_str(&another_owned);
                self.player1 = owned;
            }

            Msg::setDifficulty(val) => {self.difficulty = val.data().unwrap();}
            Msg::setTO(val) => {self.TO = val.data().unwrap();}
            Msg::StartGame => {
                // js! {
                //     console.log("Something");
                // }
                self.start_or_end = true;


                let difficulty_selector: SelectElement = document()
                    .query_selector("#difficulty")
                    .unwrap()
                    .unwrap()
                    .try_into()
                    .unwrap();

                self.difficulty = match difficulty_selector.value().unwrap().as_str() {
                    "easy" => "easy".to_string(),
                    "medium" => "medium".to_string(),
                    "hard" => "hard".to_string(),
                    _ => "easy".to_string(),
                };



                self.game = Rc::new(RefCell::new(Game {
                    p1: self.player1.clone(),
                    p2: self.player2.clone(),
                    play_with_computer: false,
                    winner: "".to_string(),
                    total_move: 0,
                    max_ai_depth: 4,
                    h_map: HashMap::new(),
                    num_row:6,
                    num_col:7,
                }));

                //////
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
                    let num_cols = game_clone.clone().borrow().num_col;
                    for col in 0..num_cols {
                        let x_col = 75 * col as i32 + 100;
                            log::info!("col is {}", col);
                            if (x_click - x_col) * (x_click - x_col) <= 25 * 25 {
                                link.send_message(Msg::clicked(Some(col as usize)));
                                return;
                            }   
                    }
                    link.send_message(Msg::clicked(None));
                }));

                background(self.game.clone());
                // while(true){};
            }
            Msg::EndGame => {self.start_or_end = false;}
            Msg::clicked(col) => {
                let sel_box: SelectElement = document()
                    .query_selector("#TO")
                    .unwrap()
                    .unwrap()
                    .try_into()
                    .unwrap();
            
                self.TO = match sel_box.value().unwrap().as_str() {
                    "T" => "T".to_string(),
                    "O" => "O".to_string(),
                    _ => "".to_string(),
                };
                if col.is_some() {
                    let temp_col = col.unwrap() as usize;
                    self.new_select_col = temp_col;
                    test_draw(self.game.clone(), temp_col);
                    computer_draw(self.game.clone(), 5);
                }
                // test_draw(self.game.clone(), 0);
            }
        }
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool{
        unimplemented!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        // let onchange = _ctx.link().batch_callback(|e| {
        //     if let ChangeData::Select(select) = e {
        //         // do something with web_sys::HtmlSelectElement
        //         self.TO = select.value;
        //     } else {
        //         None
        //     }
        // });

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
                    <button
                        id="startbutton" 
                        onclick={&self.start_callback}
                        disabled={false} 
                        title="Start Game">
                        { "Start Game" }
                    </button>
                    // <button id = "startbutton" onclick={Callback::from(|_| ())}>
                    //      ^^^^^^^ event listener name
                    //     { "start game!" }
                    // </button>
                    // <button style="margin: 5px" onclick={Callback::from(|_| (Msg::StartGame))}>{ "Start Game" }</button>
                    // <button style="margin: 5px" onclick={_ctx.link().callback(|_| Msg::StartGame)}>{ "Start Game" }</button>
                    </div>
                // <h4>{format!("New Game:  {} Vs Computer",self.player1)}</h4>
                <p>{format!("New Game:  {} Vs Computer",self.player1)}</p>
                <p>{format!("diff:  {} Vs Computer",self.difficulty)}</p>
            </div>
            }
            }
            else {
            html!{
            <div class="post" ng-repeat="game in games">
                <br/>
                <p>{format!("diff:  {} Vs Computer",self.difficulty)}</p>
                <p>{format!("TO:  {}",self.TO)}</p>
                <p>{format!("new col:  {}",self.new_select_col)}</p>
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