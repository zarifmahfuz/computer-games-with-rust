use requests::request;
use games::toot::TootAndOttoState;
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
// use web_sys::Document;
// use crate::pages::toot::*;
use std::collections::HashMap;

// use error_chain::error_chain;
use serde::Deserialize;
use serde_json::json;
use reqwest::Client;
// use tokio::task;

// use wasm_bindgen_futures::futures_0_3::spawn_local;
use wasm_bindgen_futures::spawn_local;

use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
// sue web_sys::Request;
// use std::env;
// use reqwest::Client;
// use yew::format::Nothing;
// use yew::format::Json;
// use yew::services::fetch::Request;

// use yew::{
//     format::{Json, Nothing},
//     prelude::*,
//     services::fetch::{FetchService, FetchTask, Request, Response},
// };


// error_chain! {
//     foreign_links {
//         EnvVar(env::VarError);
//         HttpRequest(reqwest::Error);
//     }
// }
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
// for player draw
fn test_draw(game: Rc<RefCell<TootAndOttoState>>, col: usize, TO: i32) {

    let canvas: CanvasElement = document()
        .query_selector("#background")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    // log::info!("im here {}", col);
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
    }
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

}

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
    // row = 6-row;
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
    }
    context.restore();
    let mut text = "T";
    if TO == 1 {
        text = "T";
    }
    else if TO == -1{
        text = "O";
    }
    // context.fill_text(text, ((col as f64)- 8.5) as f64, ((row as f64) + 8.0) as f64, None);
    context.set_font("bold 25px serif");
    context.fill_text(text, (75 * col + 100 -9) as f64, (75 * row + 50 + 8) as f64, None);

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

            Msg::setDifficulty(val) => {self.difficulty = 1;}
            Msg::setTO(val) => {self.TO = 1;}
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
                    "easy" => 1,
                    "medium" => 2,
                    "hard" => 3,
                    _ => 1,
                };



                self.game = Rc::new(RefCell::new(TootAndOttoState::new(6, 7, self.difficulty, true, &self.player1, &"Computer".to_string())));

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
                    let colss = game_clone.clone().borrow().cols;
                    for col in 0..colss {
                        let x_col = 75 * col as i32 + 100;
                            // log::info!("col is {}", col);
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

            Msg::record() => {
                spawn_local(async move{
                    let resp = req().await;
                    log::info!("body = {:#?}", resp);
                });

                // use reqwasm::http::Request;
                // use reqwasm::http::RequestMode::NoCors;
                // spawn_local(async move{
                //     let fetched_videos: Vec<Branch>= Request::get("http://localhost:5000/gameresults")
                //     .mode(NoCors)
                //     .send()
                //     .await
                //     .unwrap()
                //     .json()
                //     // .then(data => {log::info!("body = {:#?}", fetched_videos);});
                //     .await
                //     .unwrap();
                //     log::info!("body = {:#?}", fetched_videos);
                // });

                // let videos = use_state(|| vec![]);
                // {
                //     let videos = videos.clone();
                //     use_effect_with_deps(move |_| {
                //         let videos = videos.clone();
                //         wasm_bindgen_futures::spawn_local(async move {
                //             let fetched_videos: Vec<Branch> = Request::get("http://127.0.0.1:5000/gameresults")
                //                 .send()
                //                 .await
                //                 .unwrap()
                //                 .json()
                //                 .await
                //                 .unwrap();
                //                 videos.set(fetched_videos);
                //         });
                //             || ()
                //         }, ());
                // }

                // use futures::prelude::*;

                // futures::stream::iter(0..1)
                //     .for_each(|c| async move {
                //         let resp = req().await;
                //         log::info!("body = {:#?}", resp);
                //     });

                // let callback = _ctx.link().callback(async move{
                //     let resp = req().await;
                //     log::info!("body = {:#?}", resp);
                // });

                // async move {
                //     let resp = req().await;
                //     log::info!("body = {:#?}", resp);
                // };

            }
            Msg::clicked(col) => {
                let link = _ctx.link().clone();
                link.send_message(Msg::record());
                // let result = post_game();

                if self.winner == 0{
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
                    if col.is_some() {
                        let temp_col = col.unwrap() as usize;
                        self.new_select_col = temp_col;
                        test_draw(self.game.clone(), temp_col, self.TO);
                        self.game.clone().borrow_mut().player_1_move(temp_col, self.TO);
                        self.winner = self.game.clone().borrow().check_winner();
                        if self.winner == -1 {
                            // log::info!("player win");
                            winner_draw(self.game.clone(), self.winner);
                        }
                        else if self.winner == 0 {
                            let (row_to_move, col_to_move, TO_flag) = self.game.clone().borrow_mut().player_2_move(0,1);
                            log::info!("TO falg is {} in canvas",TO_flag);
                            computer_draw(self.game.clone(), col_to_move as usize, TO_flag);
                            self.winner = self.game.clone().borrow().check_winner();
                            if self.winner == 1 {
                                // log::info!("computer win");
                                winner_draw(self.game.clone(), self.winner);
                            }
                        }
                    }
                }
                else {
                    // post_game();
                    // let gist_body = json!({
                    //     "game_type": "Connect-4",
                    //     "p1_name": "John",
                    //     "p2_name": "Computer",
                    //     "is_draw": false,
                    //     "winner_name": "Computer",
                    //     "difficulty": "Hard",
                    //     "date_time": "2022-04-06T05:00:00.00Z"
                    // });
                    // let request_url = "http://127.0.0.1:5000/gameresults";

                    // let request = new Request('/myEndpoint', {
                    //     method: 'POST',
                    //     body: JSON.stringify(obj)
                    //    });
                    // // let response = reqwest::Client::new().post(request_url)
                    // // .json(&gist_body)
                    // // .send().await;
                
                    // let post_request = Request::post("http://127.0.0.1:5000/gameresults")
                    // .header("Content-Type", "application/json")
                    // .body(Json(&gist_body))
                    // .expect("Could not build that request.");
                
                    // post_game();
                    // let gist_body = json!({
                    //     "game_type": "Connect-4",
                    //     "p1_name": "Zarif",
                    //     "p2_name": "Computer",
                    //     "is_draw": false,
                    //     "winner_name": "Computer",
                    //     "difficulty": "Hard",
                    //     "date_time": "2022-04-06T05:00:00.00Z"
                    // });
                    // let request_url = "http://127.0.0.1:5000/gameresults";

                    // let response = Client::new()
                    // .post(request_url)
                    // .json(&gist_body)
                    // .send().await;

                    // log::info!("Created {:?}", gist);
                    // println!("Created {:?}", gist);
                    
                    self.winner = 0;
                    self.game = Rc::new(RefCell::new(TootAndOttoState::new(6, 7, self.difficulty, true, &self.player1, &"Computer".to_string())));
                    let canvas: CanvasElement = document()
                    .query_selector("#background")
                    .unwrap()
                    .unwrap()
                    .try_into()
                    .unwrap();
                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
                    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                    background(self.game.clone());
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
use serde::Serialize;
use wasm_bindgen::closure::Closure;
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub _id: String,
    pub game_type: String,
    pub p1_name: String,
    pub p2_name: String,
    pub is_draw: bool,
    pub winner_name: String,
    pub difficulty: String,
    pub date_time: String,
}
async fn req(){
    use reqwest::header::CONTENT_TYPE;
    use reqwest::header::ACCEPT;
    use reqwest::header::AUTHORIZATION;
    use reqwest::RequestBuilder;
    // use reqwest::Client;
    let gist_body = json!({
        "game_type": "Connect-4",
        "p1_name": "John",
        "p2_name": "Computer",
        "is_draw": false,
        "winner_name": "Computer",
        "difficulty": "Hard",
        "date_time": "2022-04-06T05:00:00.00Z"
    });
    let _id = "99".to_string();
    let game_type = "TOOT".to_string();
    let p1_name = "John".to_string();
    let p2_name = "Computer".to_string();
    let is_draw = false;
    let winner_name = "Computer".to_string();
    let difficulty = "Hard".to_string();
    let date_time = "2022-04-06T05:00:00.00Z".to_string();

    let data = JsValue::from_serde(&Branch {
        _id,
        game_type,
        p1_name,
        p2_name,
        is_draw,
        winner_name,
        difficulty,
        date_time,
    })
    .unwrap();
    let request_url = "http://127.0.0.1:5000/gameresults";
    // let client = reqwest::Client::new();
    // let resp = client
    // .post(request_url)
    // .header(CONTENT_TYPE, "application/json")
    // .json(&gist_body)
    // .send()
    // .await
    // .unwrap();

    // let resp = client.
    // get("http://localhost:5000/gameresults").
    // fetch_mode_no_cors().
    // // json(&gist_body).
    // // header(ACCEPT, "application/json").
    // // header(CONTENT_TYPE, "application/json").
    // send().await.unwrap().text().await;
    // // let resp = reqwest::get("http://127.0.0.1:5000/gameresults")
    // // .await;
    // log::info!("body = {:#?}", resp);
    // // link.send_self(Msg::record());
    // return resp.unwrap();


    // use web_sys::{Request, RequestInit, RequestMode, Response};
    // use wasm_bindgen_futures::JsFuture;
    // use wasm_bindgen::JsCast;

    let url = "http://127.0.0.1:5000/gameresults";

    let request = web_sys::Request::new_with_str_and_init(
        "/gameresults",
        web_sys::RequestInit::new()
            .body(Some(js_sys::JSON::stringify(&data).unwrap().as_ref()))
            .method("POST"),
            // .mode(RequestMode::NoCors),
    ).unwrap();
    request.headers()
        .set("Content-Type", "application/json")
        .unwrap();
    let window = web_sys::window().unwrap();
    // let resp_value = window.fetch_with_request(&request.unwrap());

    // let cb = Closure::wrap(Box::new(|_| {
    //     let window = web_sys::window().unwrap();
    //     fetch_posts(&window);
    // }) as Box<dyn FnMut(_)>);
    window.fetch_with_request(&request);
    // cb.forget();

    // let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // log::info!("body = {:#?}", &resp_value);
    // // log::info!("body = {:#?}", &resp_value.unwrap().as_string());

    // // assert!(resp_value.is_instance_of::<Response>());
    // let resp: Response = resp_value.dyn_into().unwrap();

    // log::info!("body = {:#?}", &resp);
    // let json = JsFuture::from(resp.json()?).await?;
    // log::info!("body = {:#?}", &json);

    // let branch_info: Branch = json.into_serde().unwrap();
    // log::info!("body = {:#?}", &branch_info);

    // let branch_info = json.unwrap().into_serde().unwrap();

    // let resp = Request::get("http://localhost:5000/gameresults")
    // .send()
    // .await
    // .unwrap();

    // log::info!("body = {:#?}", resp_value);

    // Ok(()))
}

fn post_game() -> Result<(), Box<dyn std::error::Error>>{
    use reqwest::header::CONTENT_TYPE;
    use reqwest::header::ACCEPT;
    use reqwest::header::AUTHORIZATION;
    // use reqwest::Client;
    let gist_body = json!({
        "game_type": "Connect-4",
        "p1_name": "John",
        "p2_name": "Computer",
        "is_draw": false,
        "winner_name": "Computer",
        "difficulty": "Hard",
        "date_time": "2022-04-06T05:00:00.00Z"
    });
    let request_url = "http://127.0.0.1:5000/gameresults";

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        // go check out her latest album. It's 🔥
        query = "Little Simz"
    );

    // let client = reqwest::blocking::Client::new();
    // let response = client
    // .post(request_url)
    // .header(CONTENT_TYPE, "application/json")
    // .json(&gist_body);
    // .send()
    // .await
    // .unwrap();

    // let response = reqwest::get("http://127.0.0.1:5000/gameresults?winner_name=Computer")
    // .await?
    // .json::<HashMap<String, String>>()
    // .await?;
    // use reqwasm::http::Request;




    // let resp = reqwest::blocking::get("http://127.0.0.1:5000/gameresults")?;

    // let client = reqwest::blocking::Client::new();
    // let response = client
    // .get(url)
    // .header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
    // .header(CONTENT_TYPE, "application/json")
    // .header(ACCEPT, "application/json")
    // .send()
    // .await
    // .unwrap();


    // match response.status() {
    //     reqwest::StatusCode::OK => {
    //         // on success, parse our JSON to an APIResponse
    //         match response.json::<i32>().await {
    //             Ok(parsed) => println!("Success! {:?}", parsed),
    //             Err(_) => println!("Hm, the response didn't match the shape we expected."),
    //         };
    //     }
    //     reqwest::StatusCode::UNAUTHORIZED => {
    //         println!("Need to grab a new token");
    //     }
    //     other => {
    //         panic!("Uh oh! Something unexpected happened: {:?}", other);
    //     }
    // };
    // log::info!("body = {:#?}", resp);
    Ok(())
}