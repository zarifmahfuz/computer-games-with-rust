use yew::prelude::*;

pub struct Connect4Computer {
    game_started: bool,
    player_1_name: String,
}

pub enum Connect4ComputerMsg {
    Player1Name(String),
    StartGame,
}

impl Component for Connect4Computer {
    type Message = Connect4ComputerMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_started: false,
            player_1_name: String::from(""),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Connect4ComputerMsg::Player1Name(name) => {
                self.player_1_name = name;
            },
            Connect4ComputerMsg::StartGame => {
                self.game_started = true;
            },
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| Connect4ComputerMsg::Player1Name(e.data().unwrap()));
        let onclick = ctx.link().callback(|_| Connect4ComputerMsg::StartGame);
        html! {
            <div id="main" style="margin-left:30%">
                <div class="w3-container" id="services" style="margin-top:75px">
                    <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
                </div>

                <div class="col-md-offset-4 col-md-8">
                    <div class="col-md-offset-3 col-md-8">
                        <input id="textbox1" type="text" placeholder="Your Name" {oninput}/>
                        <input id="startbutton" class="button" type="submit" value="Start Game" {onclick}/>
                    </div>
                </div>

                // <div class="post" ng-repeat="game in games">
                //     <br/>
                //     <h4>{"New Game:  {{game.Player1Name}} Vs {{game.Player2Name}}"}</h4>
                //     <small>{"(Disc Colors: {{game.Player1Name}} - <b>Red</b>    and    {{game.Player2Name}} - <b>Yellow</b>)"}</small>
                //     <br/>
                // </div>

                <canvas id="gameboard" height="480" width="640"></canvas>
            </div>
        }
    }
}