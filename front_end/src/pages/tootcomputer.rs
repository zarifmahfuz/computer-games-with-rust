#![recursion_limit = "11256"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender,Properties, virtual_dom};
// use chrono::prelude::*;
// use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use yew_router::prelude::*;

pub struct TOOTComputer {
    link: ComponentLink<Self>,
    value: String,
}
// pub enum Msg {
//     GotInput(String),
//     Clicked,
// }
impl Component for TOOTComputer {
    type Message = ();
    type Properties = ();
fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    TOOTComputer {
        link,
        value: "".into(),
    }
}
fn update(&mut self, msg: Self::Message) -> ShouldRender {
    // match msg {
    //     Msg::GotInput(new_value) => {
    //         self.value = new_value;
    //     }
    //     Msg::Clicked => {
    //         self.value = "Changed Value".to_string();
    //     }
    // }
    true
}

fn change(&mut self, msg: Self::Message) -> ShouldRender {
    false
}



fn view(&self) -> Html {
    html! {
      <>
      <div class="w3-container" id="services" style="margin-top:75px">
        <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
        <hr style="width:50px;border:5px solid red" class="w3-round"></hr>
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
          <br></br>
          <h4>{"New Game:  {{game.Player1Name}} Vs {{game.Player2Name}}"}</h4>
          <small>{"(Winning Combination: {{game.Player1Name}} - <b>TOOT</b>    and    {{game.Player2Name}} - <b>OTTO</b>)"}</small>
          <br></br>
             
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

      </>
        }
    }
}

// fn main() {
//     yew::start_app::<WelcomeModel>();
// }