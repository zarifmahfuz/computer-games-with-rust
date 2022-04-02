#![recursion_limit = "11256"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender,Properties, virtual_dom};
// use chrono::prelude::*;
// use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use yew_router::prelude::*;

pub struct Connect4Computer {
    link: ComponentLink<Self>,
    value: String,
}
// pub enum Msg {
//     GotInput(String),
//     Clicked,
// }
impl Component for Connect4Computer {
    type Message = ();
    type Properties = ();
fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Connect4Computer {
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
        <div id="main" ng-controller="mainController">

        <div class="w3-container" id="services" style="margin-top:75px">
          <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
          <hr style="width:50px;border:5px solid red" class="w3-round"> </hr>
        </div>
           <div class="col-md-offset-4 col-md-8">
            <form  ng-submit="Game()">
              <div class="col-md-offset-3 col-md-8">
      
                <input id="textbox1" type="text" placeholder="Your Name" ng-model="newGame.Player1Name"/>
                <input id="startbutton" class="button" type="submit" value="Start Game"/>
              </div>
            </form>
            <div class="post" ng-repeat="game in games">
              <br></br>
              <h4>{"New Game:  {{game.Player1Name}} Vs {{game.Player2Name}}"}</h4>
              <small>{"(Disc Colors: {{game.Player1Name}} - <b>Red</b>    and    {{game.Player2Name}} - <b>Yellow</b>)"}</small>
              <br></br>
            </div>  
              <canvas id="gameboard" height="480" width="640"></canvas>
      
          </div>
      </div>
        }
    }
}

// fn main() {
//     yew::start_app::<WelcomeModel>();
// }