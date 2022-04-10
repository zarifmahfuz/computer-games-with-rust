use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Clone, PartialEq, Deserialize)]
struct GameResult {
    _id: i64,
    game_type: String,
    p1_name: String,
    p2_name: String,
    is_draw: bool,
    winner_name: String,
    difficulty: String,
    date_time: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Winner {
    winner_name: String,
    wins: i64,
}

#[derive(Clone, Properties, PartialEq)]
struct GameResultsProps {
    game_results: Vec<GameResult>,
}

#[function_component(GameResults)]
fn game_results(GameResultsProps { game_results }: &GameResultsProps) -> Html {
    game_results.iter().enumerate().map(|(sl, game_result)| html! {
        <tr>
        <td>{ format!("{}", sl + 1) }</td>
        <td>{ format!("{}", game_result.game_type) }</td>
        <td>{ format!("{}", game_result.winner_name) }</td>
        <td>{ format!("{}", game_result.p1_name) }</td>
        <td>{ format!("{}", game_result.date_time) }</td>
        </tr>
    }).collect()
}

#[derive(Clone, Properties, PartialEq)]
struct LeadersProps {
    leaders: Vec<Winner>,
}

#[function_component(Leaders)]
fn leaders(LeadersProps { leaders }: &LeadersProps) -> Html {
    leaders.iter().enumerate().map(|(sl, winner)| html! {
        <tr>
        <td>{ format!("{}", sl + 1) }</td>
        <td>{ format!("{}", winner.winner_name) }</td>
        <td>{ format!("{}", winner.wins) }</td>
        </tr>
    }).collect()
}

#[function_component(Leaderboard)]
fn leaderboard() -> Html {
    let computer_game_results = use_state(|| vec![]);
    {
        let computer_game_results = computer_game_results.clone();
        use_effect_with_deps(move |_| {
            let computer_game_results = computer_game_results.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_results: Vec<GameResult> = Request::get("/api/gameresults?winner_name=Computer")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    computer_game_results.set(fetched_results);
            });
            || ()
        }, ());
    }
    let leaders_all_difficulties = use_state(|| vec![]);
    {
        let leaders_all_difficulties = leaders_all_difficulties.clone();
        use_effect_with_deps(move |_| {
            let leaders_all_difficulties = leaders_all_difficulties.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_results: Vec<Winner> = Request::get("/api/leaderboard")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    leaders_all_difficulties.set(fetched_results);
            });
            || ()
        }, ());
    }
    let leaders_hard_difficulty = use_state(|| vec![]);
    {
        let leaders_hard_difficulty = leaders_hard_difficulty.clone();
        use_effect_with_deps(move |_| {
            let leaders_hard_difficulty = leaders_hard_difficulty.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_results: Vec<Winner> = Request::get("/api/leaderboard?difficulty=Hard")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    leaders_hard_difficulty.set(fetched_results);
            });
            || ()
        }, ());
    }
    let leaders_medium_difficulty = use_state(|| vec![]);
    {
        let leaders_medium_difficulty = leaders_medium_difficulty.clone();
        use_effect_with_deps(move |_| {
            let leaders_medium_difficulty = leaders_medium_difficulty.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_results: Vec<Winner> = Request::get("/api/leaderboard?difficulty=Medium")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    leaders_medium_difficulty.set(fetched_results);
            });
            || ()
        }, ());
    }
    let leaders_easy_difficulty = use_state(|| vec![]);
    {
        let leaders_easy_difficulty = leaders_easy_difficulty.clone();
        use_effect_with_deps(move |_| {
            let leaders_easy_difficulty = leaders_easy_difficulty.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_results: Vec<Winner> = Request::get("/api/leaderboard?difficulty=Easy")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    leaders_easy_difficulty.set(fetched_results);
            });
            || ()
        }, ());
    }
    html! {
        <div style="margin-left:30%">
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>

                <div><h4>{"Details of Games Won by Computer"}</h4></div>
                    <div id="game-stream">
                    <table>
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Winner"}</th>
                            <th>{"Played Against"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        <GameResults game_results={(*computer_game_results).clone()}/>
                    </table>
                </div>
                <br/>
                <div><h4>{"Details of Games Won by All Players"}</h4></div>
                <div>
                    <table>
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Winner or Draw"}</th>
                            <th>{"No. of Wins"}</th>
                        </tr>
                        <Leaders leaders={(*leaders_all_difficulties).clone()}/>
                    </table>
                </div>
                <br/>
                <div><h4>{"Details of Games Won by All Players Against Hard AI"}</h4></div>
                <div>
                    <table>
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Winner or Draw"}</th>
                            <th>{"No. of Wins"}</th>
                        </tr>
                        <Leaders leaders={(*leaders_hard_difficulty).clone()}/>
                    </table>
                </div>
                <br/>
                <div><h4>{"Details of Games Won by All Players Against Medium AI"}</h4></div>
                <div>
                    <table>
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Winner or Draw"}</th>
                            <th>{"No. of Wins"}</th>
                        </tr>
                        <Leaders leaders={(*leaders_medium_difficulty).clone()}/>
                    </table>
                </div>
                <br/>
                <div><h4>{"Details of Games Won by All Players Against Easy AI"}</h4></div>
                <div>
                    <table>
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Winner or Draw"}</th>
                            <th>{"No. of Wins"}</th>
                        </tr>
                        <Leaders leaders={(*leaders_easy_difficulty).clone()}/>
                    </table>
                </div>
            </div>
            </div>
    }
}

pub struct Score;

impl Component for Score {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        unimplemented!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Leaderboard/>
        }
    }
}