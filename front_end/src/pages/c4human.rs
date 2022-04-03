use yew::prelude::*;

pub struct Connect4Human;

impl Component for Connect4Human {
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
            <div id="main" style="margin-left:30%">
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>

            <div class="col-md-offset-4 col-md-8">
                <form  ng-submit="Game()">
                    <div class="col-md-offset-3 col-md-8">
                        <input id="textbox1" type="text" placeholder="Player 1's Name" ng-model="newGame.Player1Name"/>
                        <input id="textbox2" type="text" placeholder="Player 2's Name" ng-model="newGame.Player2Name"/>
                        <input id="startbutton" class="button" type="submit" value="Start Game"/>
                    </div>
                </form>

                <div class="post" ng-repeat="game in games">
                    <br/>
                    <h4>{"New Game:  {{game.Player1Name}} Vs {{game.Player2Name}}"}</h4>
                    <small>{"(Disc Colors: {{game.Player1Name}} - <b>Red</b>    and    {{game.Player2Name}} - <b>Yellow</b>)"}</small>
                    <br/>
                </div>

                <canvas id="gameboard" height="480" width="640"></canvas>
            </div>
            </div>
        }
    }
}