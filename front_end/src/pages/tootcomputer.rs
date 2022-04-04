use yew::prelude::*;
// use yew::{html, Html, ChangeData};
// use yew::components::Select;
// use yew::html::InputData;
// use yew::events::ClickEvent;

pub struct TOOTComputer {
    player: String,
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
}

impl Component for TOOTComputer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TOOTComputer {
            player: "".to_string(),
            difficulty: "".to_string(),
            TO: "T".to_string(),
            name_callback: _ctx.link().callback(|e: InputEvent| Msg::setPlayerName(e)),
            diff_callback: _ctx.link().callback(|e: InputEvent| Msg::setDifficulty(e)),
            TO_callback: _ctx.link().callback(|e: InputEvent| Msg::setTO(e)),
            start_callback: _ctx.link().callback(|e| Msg::StartGame),
            end_callback: _ctx.link().callback(|e| Msg::EndGame),
            start_or_end: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::setPlayerName(val) => {self.player = val.data().unwrap().to_string();}
            Msg::setDifficulty(val) => {self.difficulty = val.data().unwrap().to_string();}
            Msg::setTO(val) => {self.TO = val.data().unwrap().to_string();}
            Msg::StartGame => {self.start_or_end = true;}
            Msg::EndGame => {self.start_or_end = false;}
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
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
            <div class="col-md-offset-4 col-md-8">
                <form  ng-submit="Game()">
                <div class="col-md-offset-3 col-md-8">
        
                    <input id="textbox1" type="text" placeholder="Your Name" oninput = {&self.name_callback}/>
                    <select id="difficulty_dropdown" style="margin: 5px" oninput={&self.diff_callback}>
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
                </div>
                </form>
            </div>
            <div class="post" ng-repeat="game in games">
                <br/>
                <h4>{format!("New Game:  {} Vs Computer",self.player)}</h4>
                <small>{format!("(Winning Combination: {} - ", self.player)} <b>{"TOOT"}</b> {"   and    Computer - "} <b>{"OTTO)"}</b></small>
                <br/>
                {"Select a Disc Type:  "}
                <select id="disc_dropdown" style="margin: 5px" oninput={&self.TO_callback}>
                <option selected=true disabled=false value="T">{"T"}</option>
                <option selected=false disabled=false value="O">{"O"}</option>
                </select>
            </div> 
            <canvas id="gameboard" height="480" width="640"></canvas>
            </div>
        }
    }
}