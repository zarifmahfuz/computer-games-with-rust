use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Clone, PartialEq, Deserialize)]
struct GameResult {
    _id: String,
    game_type: String,
    p1_name: String,
    p2_name: String,
    is_draw: bool,
    winner_name: String,
    difficulty: String,
    date_time: String,
}

#[derive(Clone, Properties, PartialEq)]
struct GameResultsProps {
    game_results: Vec<GameResult>,
}

#[function_component(GameResults)]
fn game_results(GameResultsProps { game_results }: &GameResultsProps) -> Html {
    game_results.iter().map(|game_result| html! {
        <tr>
        <td>{ format!("{}", game_result._id) }</td>
        <td>{ format!("{}", game_result.game_type) }</td>
        <td>{ format!("{}", game_result.p1_name) }</td>
        <td>{ format!("{}", game_result.p2_name) }</td>
        if !game_result.is_draw {
            <td>{ format!("{}", game_result.winner_name) }</td>
        } else {
            <td>{ "Draw" }</td>
        }
        <td>{ format!("{}", game_result.date_time) }</td>
        </tr>
    }).collect()
}

#[function_component(GameHistory)]
fn game_history() -> Html {
    let game_results = use_state(|| vec![]);
    {
        let game_results = game_results.clone();
        use_effect_with_deps(move |_| {
            let game_results = game_results.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_results: Vec<GameResult> = Request::get("/gameresults")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                game_results.set(fetched_results);
            });
            || ()
        }, ());
    }
    html! {
        <>
            <div class="w3-container" id="services" style="margin-top:75px; margin-left: 25%">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>

                <div id="game-stream">
                    <table>
                        <tr>
                        <th>{"Game-ID"}</th>
                        <th>{"Game Type"}</th>
                            <th>{"Player1"}</th>
                            <th>{"Player2"}</th>
                            <th>{"Winner"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        <GameResults game_results={(*game_results).clone()}/>
                    </table>
                </div>
            </div>
        </>
    }
}

pub struct ScoreBoard;

impl Component for ScoreBoard {
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
            <GameHistory/>
        }
    }
}