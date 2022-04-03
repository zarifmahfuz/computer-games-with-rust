use yew::prelude::*;

pub struct TOOTComputer;

impl Component for TOOTComputer {
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
            <div style="margin-left:30%">
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>
            <div class="col-md-offset-4 col-md-8">
                <form  ng-submit="Game()">
                <div class="col-md-offset-3 col-md-8">
        
                    <input id="textbox1" type="text" placeholder="Your Name" ng-model="newGame.Player1Name"/>
                    <input id="startbutton" class="button" type="submit" value="Start Game"/>
                </div>
                </form>
            </div>
            <div class="post" ng-repeat="game in games">
                <br/>
                <h4>{"New Game:  {{game.Player1Name}} Vs {{game.Player2Name}}"}</h4>
                <small>{"(Winning Combination: {{game.Player1Name}} - <b>TOOT</b>    and    {{game.Player2Name}} - <b>OTTO</b>)"}</small>
                <br/>
                    
                <form>
                    <h4>{"Select a Disc Type   :"}</h4>
                    // removed a checked here
                    // <input type="radio" name="choice" value="T" checked ng-model="newGame.Label"> T
                    <input type="radio" name="choice" value="T" ng-model="newGame.Label"/>
                    // removed T and O
                    <input type="radio" name="choice" value="O" ng-model="newGame.Label"/>
                    
                </form>
            </div> 
            <canvas id="gameboard" height="480" width="640"></canvas>
            </div>
        }
    }
}