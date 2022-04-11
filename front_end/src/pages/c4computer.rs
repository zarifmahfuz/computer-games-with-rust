use my_2nd_yew_app::connect4::Connect4State;

use yew::prelude::*;
use wasm_bindgen::JsCast;
use std::convert::TryInto;

use serde::Deserialize;
use serde::Serialize;

use wasm_bindgen::JsValue;

pub struct Connect4Computer {
    game_started: bool,
    player_1_name: String,
    player_2_name: String,
    board_size: (i32, i32),
    difficulty: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostWinnerReq {
    game_type: String,
    p1_name: String,
    p2_name: String,
    is_draw: bool,
    winner_name: String,
    difficulty: String,
    date_time: String,
}

pub enum Connect4ComputerMsg {
    Player1Name(Option<String>),
    StartGame,
}

#[derive(PartialEq, Debug, Clone)]
struct Connect4Info {
    game_started: bool,
    player_1_name: String,
    player_2_name: String,
    board_size: (i32, i32),
    difficulty: i32,
}

#[derive(PartialEq, Debug, Clone)]
struct GameBoard {
    gameboard: Vec<Vec<i32>>,
    controller: i32,
    connect_obj: Connect4State,
}

#[derive(Properties, PartialEq)]
struct ViewGameInfoProps {
    game_info: Option<Connect4Info>
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
    let game_info_callback = game_info.clone();
    let board_col = game_info_callback.board_size.0.try_into().unwrap();
    let board_row = game_info_callback.board_size.1.try_into().unwrap();

    // used to track the game board status
    let game_board_state = use_state_eq::<GameBoard,_>(|| 
        GameBoard {
            gameboard: vec![ vec![0; board_row]; board_col],
            controller: 1, // indicates whose player's turn it is
            connect_obj: Connect4State::new(board_row, board_col, game_info_callback.difficulty, true, &game_info_callback.player_1_name, &game_info_callback.player_2_name)
        });

    let game_board_state_clone = game_board_state.clone();

    let handle_game_end_cond =  move |board: GameBoard| -> bool {
        let winner = board.connect_obj.check_winner();
        if winner != 0 {
            let mut win_text: String = "It's a draw".to_owned();
            let mut winner_name = game_info_callback.player_1_name.to_string();
            if winner == 1 {
                win_text = format!("{} wins",game_info_callback.player_2_name.to_string());
                winner_name = game_info_callback.player_2_name.to_string();
            }
            else if winner == -1 {
                win_text = format!("{} wins",game_info_callback.player_1_name.to_string());
            }

            // show a winning message
            win_text += " - Click on game board to reset";

            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = document.get_element_by_id("gameboard").unwrap();

            let canvas: web_sys::HtmlCanvasElement = canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

            let cntx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            cntx.set_font("14pt sans-serif");
            cntx.set_fill_style(&"#111".into());
            cntx.fill_text(&win_text, 130.0, 20.0);

            let game_info_clone = game_info_callback.clone();
            // post request here
            wasm_bindgen_futures::spawn_local(async move {


                let diff = match game_info_clone.difficulty {
                    1 => "Easy".to_string(),
                    2 => "Medium".to_string(),
                    5 => "Hard".to_string(),
                    _ => "Error".to_string(),
                };

                let now = js_sys::Date::new_0();
                let date: String = now.to_iso_string().into();

                let body = JsValue::from_serde(
                    &PostWinnerReq {
                        game_type: "Connect-4".to_string(),
                        p1_name: game_info_clone.player_1_name,
                        p2_name: game_info_clone.player_2_name,
                        is_draw: (winner == 2),
                        winner_name: winner_name,
                        difficulty: diff,
                        date_time: date,
                    }
                ).unwrap();

                let request = web_sys::Request::new_with_str_and_init(
                    "/api/gameresults",
                    web_sys::RequestInit::new()
                        .body(Some(js_sys::JSON::stringify(&body)
                        .unwrap()
                        .as_ref()))
                        .method("POST"),
                ).unwrap();

                request.headers()
                    .set("Content-Type", "application/json").unwrap();
                    
                let window = web_sys::window().unwrap();
                window.fetch_with_request(&request);
            });

            return true;
        }
        else {
            return false;
        }
    };

    // handle the clicking
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


        let mut turns = 0;

        let mut board;
        board = (*game_board_state).clone();

        let winner = board.connect_obj.check_winner();
        if winner != 0 {
            document.location().unwrap().reload();
            return;
        }

        let mut valid_move = false;

            let mut j = 0;
            while j < board_col {

                if ((x_coord - (75.0 * (j as f64) + 100.0))*(x_coord - (75.0 * (j as f64) + 100.0)) ) <=  (25.0 * 25.0) {
                    
                    let mut row = 0;
                    let mut i = 0;

                    while  i < board_row {
                        if board.gameboard[j][i] == 0 {
                            // it's empty, you can add circle here
                            board.gameboard[j][i] = board.controller;

                            // web_sys::console::log_1(&(format!("the board: {:?} ", board)).clone().into());

                            // keep this here to trigger the drawing
                            context.begin_path();
                            if board.controller == 1 {
                                context.set_fill_style(&"#ff4136".into()); 
                                board.connect_obj.player_1_move(j);
                            }
                            else {
                                context.set_fill_style(&"#ffff00".into()); 
                            }
                            row = (i+board_row - 2*i)-1;
                            context.arc(75.0 * (j as f64) + 100.0, 75.0 * (row as f64) + 50.0, 25.0, 0.0, std::f64::consts::PI * 2.0);
                            context.fill();

                            valid_move = true;
                            break; // you added the piece, you are done
                        }
                        // that row is full, go to the next one
                        i+=1
                    }

                    if valid_move {
                        let winner_found = handle_game_end_cond(board.clone());
                        if winner_found {
                            game_board_state.set(board);
                            return;
                        }
                    }
                    break;
                }
                j+=1;
            }

        if valid_move {
            web_sys::console::log_1(&"computer makes a move".clone().into());
            let col_num = board.connect_obj.player_2_move(0).1;
            let col:usize = col_num.try_into().unwrap();
            let mut y = 0;
            while  y < board_row {
                if board.gameboard[col][y] == 0 {
                    // it's empty, you can add circle here
                    board.gameboard[col][y] = board.controller;

                    // keep this here to trigger the drawing
                    context.begin_path();
                    context.set_fill_style(&"#ffff00".into()); 

                    let row = (y+board_row - 2*y)-1;
                    context.arc(75.0 * (col as f64) + 100.0, 75.0 * (row as f64) + 50.0, 25.0, 0.0, std::f64::consts::PI * 2.0);
                    context.fill();
                    break; // you added the piece, you are done
                }
                // that row is full, go to the next one
                y+=1
            }
            handle_game_end_cond(board.clone());
        }
        game_board_state.set(board);

    });

    // draw the board
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
            while y <board_row {
                x = 0;
                while x < board_col {
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
                    <h4>{format!("New Game: {} Vs {}", game_info.player_1_name,game_info.player_2_name )}</h4>
                    <small> {format!("Disc Colors: {} - ", game_info.player_1_name)} </small>
                    <small> <strong> {"Red"} </strong> </small>
                    <small>{format!(" and {} - ", game_info.player_2_name)}</small>
                    <small> <b>{"Yellow"}</b></small>
                <br/>
            </div>

            <canvas id="gameboard" height={format!("{}", board_row*80)} width={format!("{}", board_col*90)} onclick = {redraw} ></canvas>
        </div>
    }
}


#[derive(Properties, PartialEq)]
pub struct GameProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Connect4Computer {
    type Message = Connect4ComputerMsg;
    type Properties = GameProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_started: false,
            player_1_name: String::from(""),
            player_2_name: String::from("Computer"),
            board_size: (6,7),
            difficulty: 1,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Connect4ComputerMsg::Player1Name(name) => {
                // web_sys::console::log_1(&name.clone().into());
                match name {
                    Some(n) => self.player_1_name += &n,
                    None => { self.player_1_name.pop(); }
                }
            },
            Connect4ComputerMsg::StartGame => {
                self.game_started = true;
                let document = web_sys::window().unwrap().document().unwrap();

                // get the selected board size
                let selector = document.query_selector("#size_selector")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlSelectElement>()
                    .unwrap();
                let val = selector.value();

                // store the selected board size
                match val.as_str() {
                    "7x6" => self.board_size = (7,6),
                    "6x4" => self.board_size = (6,4),
                    "8x8" => self.board_size = (8,8),
                    _ => println!("something else!"),
                }

                // get the selected difficulty
                let diff_selector = document.query_selector("#diff_selector")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlSelectElement>()
                    .unwrap();
                self.difficulty = diff_selector.value().parse::<i32>().unwrap();

            },
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let oninput1 = ctx.link().callback(|e: InputEvent| Connect4ComputerMsg::Player1Name(e.data()));
        let onclick = ctx.link().callback(|_| Connect4ComputerMsg::StartGame);

        let game_info = Connect4Info {
            game_started: self.game_started.clone(),
            player_1_name: self.player_1_name.clone(),
            player_2_name: self.player_2_name.clone(),
            board_size: self.board_size.clone(),
            difficulty: self.difficulty.clone(),
        };

        html! {
            <div id="main" style="margin-left:30%">
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>

            <div class="col-md-offset-4 col-md-8">
                <input id="textbox1" type="text" placeholder="Player 1's Name" oninput = {oninput1}/>
                <input id="startbutton" class="button" type="submit" {onclick} /> //value="Start Game"
            </div>

            <div>
                <label for="size_selector"> {"Choose a board size:"} </label>
                <select id="size_selector" style="margin: 5px" >
                    <option selected=true disabled=false value={"7x6"}>{"7x6"}</option>
                    <option selected=false disabled=false value="6x4">{"6x4"}</option>
                    <option selected=false disabled=false value="8x8">{"8x8"}</option>
                </select>

                <label for="diff_selector"> {"Choose a diffuclty:"} </label>
                <select id="diff_selector" style="margin: 5px" >
                    <option selected=true disabled=false value=1> {"Easy"}</option>
                    <option selected=false disabled=false value=2> {"Medium"}</option>
                    <option selected=false disabled=false value=5> {"Hard"}</option>
                </select>
            </div>

                <div>
                    <ViewGameInfo game_info = {Some(game_info)} />
                </div>
            </div>
        }
    }
}