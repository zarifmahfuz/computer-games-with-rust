use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;

use web_sys::{Request, RequestInit, RequestMode, Response};

pub struct TOOTComputer {
    game_started: bool,
    player_1_name: String,
    player_2_name: String,
    difficulty: i32,
    TO: i32,
    // used to update name
    // name_callback: Callback<InputEvent>,
    // // used to update diff
    // diff_callback: Callback<InputEvent>,
    // // used to change T or O
    // TO_callback: Callback<InputEvent>,
    // // used to start the game
    // start_callback: Callback<MouseEvent>,
    // // used to end the game
    // end_callback: Callback<MouseEvent>,
    // true => start
    // false => end
    start_or_end: bool,
    new_select_col: usize,
    // game: Rc<RefCell<Game>>,
    winner: i32,
}

pub enum TOOTComputerMsg {
    Player1Name(Option<String>),
    StartGame,
}

#[derive(PartialEq, Debug, Clone)]
struct TOOTInfo {
    game_started: bool,
    player_name: String,
}

#[derive(PartialEq, Debug, Clone)]
struct GameBoard {
    gameboard: Vec<Vec<i32>>,
}

#[derive(Properties, PartialEq)]
struct ViewGameInfoProps {
    game_info: Option<TOOTInfo>
}


#[function_component(ViewGameInfo)]
fn view_game(props: &ViewGameInfoProps) -> Html {

    let game_info = match &props.game_info {
        Some(p) => p,
        None => return html!{},
    };

    // the game hasn't started yet
    if !game_info.game_started {
        web_sys::console::log_1(&"NO_STARTED".clone().into());
        return html! {};
    }

    // handle the clicking
    let game_board_state = use_state_eq::<GameBoard,_>(|| 
        GameBoard {
            gameboard: vec![ vec![0; 6]; 7] // chaneg the 6 for the correct board size
        });
    let game_board_state_clone = game_board_state.clone();

    let redraw = Callback::from(move |mouse_event: MouseEvent| {
        web_sys::console::log_1(&mouse_event.clone().into());
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("gameboard").unwrap();

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let rect = canvas.get_bounding_client_rect();
        let x_coord = (mouse_event.client_x() as f64) - rect.left();
        let y_coord = (mouse_event.client_y() as f64) - rect.top();

        let mut j = 0;
        while j < 7 {

            if ((x_coord - (75.0 * (j as f64) + 100.0))*(x_coord - (75.0 * (j as f64) + 100.0)) ) <=  (25.0 * 25.0) {
                // console.log("clicked region " + j);
                
                web_sys::console::log_1(&"click valid".clone().into());

                let mut valid_move = false;
                let mut row = 0;
                let mut i = 0;
                let mut board;
                board = (*game_board_state).clone();

                while  i < 6 {
                    if board.gameboard[j][i] != 1 {
                        // it's empty, you can add circle here
                        board.gameboard[j][i] = 1;

                        // addCircle();
                        web_sys::console::log_1(&(format!("the board: {:?} ", board)).clone().into());
                        game_board_state.set(board);

                        // keep this here to trigger the drawing
                        context.begin_path();
                        context.set_fill_style(&"#ff4136".into()); 
                        row = (i+6 - 2*i)-1;
                        context.arc(75.0 * (j as f64) + 100.0, 75.0 * (row as f64) + 50.0, 25.0, 0.0, std::f64::consts::PI * 2.0);
                        context.fill();

                        valid_move = true;
                        break; // you added the piece, you are done
                    }
                    // that row is full, go to the next one
                    i+=1
                }

                if valid_move {
                    // computer makes a move
                    web_sys::console::log_1(&("computer move!").clone().into());
                }

                // this.paused = false;

                // check if entering a thing is valid


                // valid = this.action(j); 
                // if (valid === 1) { // give user retry if action is invalid
                //     this.rejectClick = true;
                // }
                break; //because there will be no 2 points that are clicked at a time
            }
            j+=1;
        }

    });

    use_effect(move || {
        // Make a call to DOM API after component is rendered
        web_sys::console::log_1(&"update".clone().into());
        let document = web_sys::window().unwrap().document().unwrap();//.document().unwrap();
        let canvas = document.get_element_by_id("gameboard").unwrap();

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // ensure board is only redrawn when the game is new
        let move_tracker = (*game_board_state_clone).gameboard.iter().map(|vec| vec.iter().sum::<i32>()).sum::<i32>();
        if move_tracker == 0 {

            // draw the board
            context.begin_path();
            let mut y = 0;
            let mut x = 0;
            context.set_fill_style(&"#00bfff".into()); 
            while y <6 {
                x = 0;
                while x < 7 {
                    context.arc(75.0 * (x as f64) + 100.0, 75.0 * (y as f64) + 50.0, 25.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                    context.rect(75.0 * (x as f64) + 150.0, 75.0 * (y as f64), -100.0, 100.0);
                    x+=1;
                }
                y+=1;
            }
            context.fill();
        }


        // Perform the cleanup
        || web_sys::console::log_1(&"cleanup".clone().into())
    });


    html!{
        <div>

            <div class="post" ng-repeat="game in games">
                <br/>
                    <h4>{format!("New Game: {} Vs Computer", game_info.player_name)}</h4>
                    <small> {format!("Disc Colors: {} - ", game_info.player_name)} </small>
                    <small> <strong> {"Red"} </strong> </small>
                    <small>{" and Computer - "}</small>
                    <small> <b>{"Yellow"}</b></small>
                <br/>

                {"Select a Disc Type:  "}
                    <select id="TO" style="margin: 5px" >
                    <option selected=true disabled=false value="T">{"T"}</option>
                    <option selected=false disabled=false value="O">{"O"}</option>
                </select>
            </div>


            <canvas id="gameboard" height="480" width="640" onclick = {redraw} ></canvas>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct GameProps {
    #[prop_or_default]
    pub children: Children,
}


impl Component for TOOTComputer {
    type Message = TOOTComputerMsg;
    type Properties = GameProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_started: false,
            player_1_name: String::from(""),
            player_2_name: "Computer".to_string(),
            difficulty: 1,
            TO: 1,
            // name_callback: _ctx.link().callback(|e: InputEvent| Msg::setPlayerName(e)),
            // diff_callback: _ctx.link().callback(|e: InputEvent| Msg::setDifficulty(e)),
            // TO_callback: _ctx.link().callback(|e: InputEvent| Msg::setTO(e)),
            // start_callback: _ctx.link().callback(|_| Msg::StartGame),
            // end_callback: _ctx.link().callback(|e| Msg::EndGame),
            start_or_end: false,
            new_select_col: 0,
            // game: game.clone(),
            winner: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TOOTComputerMsg::Player1Name(name) => {
                // web_sys::console::log_1(&name.clone().into());
                match name {
                    Some(n) => self.player_1_name += &n,
                    None => { self.player_1_name.pop(); }
                }
            },
            TOOTComputerMsg::StartGame => {
                self.game_started = true;



            },
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let game_state_outer = game_state.clone();
        // let game_state = use_latest::<Option<GameInfo>,_>(|| None);

        let oninput = ctx.link().callback(|e: InputEvent| TOOTComputerMsg::Player1Name(e.data()));
        let onclick = ctx.link().callback(|_| TOOTComputerMsg::StartGame);
        // let oninput = ctx.link().callback(|e: InputEvent| TOOTComputerMsg::Player1Name(e.data()));
        
        let game_info = TOOTInfo {
            game_started: self.game_started.clone(),
            player_name: self.player_1_name.clone(),
        };

        html! {
            // <div style="margin-left:30%">
            // <div class="w3-container" id="services" style="margin-top:75px">
            //     <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
            //     <hr style="width:50px;border:5px solid red" class="w3-round"/>
            // </div>
            // { if !self.start_or_end {
            // html!{
            // <div class="col-md-offset-4 col-md-8">
            //     <div class="col-md-offset-3 col-md-8">
        
            //         // <input id="textbox1" type="text" placeholder="Your Name" oninput = {&self.name_callback}/>
            //         <input id="textbox1" type="text" placeholder="Your Name" {oninput}/>
            //         <select id="difficulty" style="margin: 5px">
            //             <option selected=true disabled=false value="easy">{"Easy"}</option>
            //             <option selected=false disabled=false value="medium">{"Medium"}</option>
            //             <option selected=false disabled=false value="hard">{"Hard"}</option>
            //         </select>
            //         <input id="startbutton" class="button" type="submit" value="Start Game" {onclick}/>

            //         // <button
            //         //     id="startbutton" 
            //         //     onclick={&self.start_callback}
            //         //     disabled={false} 
            //         //     title="Start Game">
            //         //     { "Start Game" }
            //         // </button>

            //         // <button id = "startbutton" onclick={Callback::from(|_| ())}>
            //         //      ^^^^^^^ event listener name
            //         //     { "start game!" }
            //         // </button>
            //         // <button style="margin: 5px" onclick={Callback::from(|_| (Msg::StartGame))}>{ "Start Game" }</button>
            //         // <button style="margin: 5px" onclick={_ctx.link().callback(|_| Msg::StartGame)}>{ "Start Game" }</button>
            //         </div>
            //     // <h4>{format!("New Game:  {} Vs Computer",self.player1)}</h4>
            //     <p>{format!("New Game:  {} Vs Computer",self.player_1_name)}</p>
            //     <p>{format!("diff:  {} Vs Computer",self.difficulty)}</p>
            // </div>
            // }
            // }
            // else {
            // html!{
            // <div class="post" ng-repeat="game in games">
            //     <br/>
            //     <p>{format!("diff:  {} Vs Computer",self.difficulty)}</p>
            //     <p>{format!("TO:  {}",self.TO)}</p>
            //     <p>{format!("new col:  {}",self.new_select_col)}</p>
            //     <h4>{format!("New Game:  {} Vs Computer",self.player_1_name)}</h4>
            //     <small>{format!("(Winning Combination: {} - ", self.player_1_name)} <b>{"TOOT"}</b> {"   and    Computer - "} <b>{"OTTO)"}</b></small>
            //     <br/>
            //     {"Select a Disc Type:  "}
            //     <select id="TO" style="margin: 5px" >
            //     <option selected=true disabled=false value="T">{"T"}</option>
            //     <option selected=false disabled=false value="O">{"O"}</option>
            //     </select>
            // </div> 
            // }}
            // }
            // <canvas id="background" height="760" width="640"></canvas>
            // </div>
            <div id="main" style="margin-left:30%">
                <div class="w3-container" id="services" style="margin-top:75px">
                    <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
                </div>

                <div class="col-md-offset-4 col-md-8">
                    <div class="col-md-offset-3 col-md-8">
                        <input id="textbox1" type="text" placeholder="Your Name" {oninput}/>
                        <select id="difficulty" style="margin: 5px">
                            <option selected=true disabled=false value="easy">{"Easy"}</option>
                            <option selected=false disabled=false value="medium">{"Medium"}</option>
                            <option selected=false disabled=false value="hard">{"Hard"}</option>
                        </select>
                        <input id="startbutton" class="button" type="submit" value="Start Game" {onclick}/>
                    </div>
                </div>

                <div>
                    <ViewGameInfo game_info = {Some(game_info)} />
                </div>

            </div>
        }
    }
}
#[derive(Clone, PartialEq, Deserialize)]
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

#[function_component(GameHistory)]
fn game_history() -> Html {
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