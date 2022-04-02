#![recursion_limit = "11256"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender,Properties, virtual_dom};
// use chrono::prelude::*;
// use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use yew_router::prelude::*;

pub struct ScoreBoard {
    link: ComponentLink<Self>,
    value: String,
}
// pub enum Msg {
//     GotInput(String),
//     Clicked,
// }
impl Component for ScoreBoard {
    type Message = ();
    type Properties = ();
fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
  ScoreBoard {
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
      <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
      <hr style="width:50px;border:5px solid red" class="w3-round"> </hr>
      
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
        <tr ng-repeat="game in games">
           <td>{"{{ $index + 1 }}"}</td>
           <td>{"{{game.gameType}}"}</td>
           <td>{"{{game.Player1Name}}"}</td>
           <td>{"{{game.Player2Name}}"}</td>
           <td>{"{{game.WinnerName}}"}</td>
           <td>{"{{game.GameDate | date:h:mma on MMM d, y}}"}</td>
        </tr>
      </table>
          
        </div>
      </div>
  

      </>
        }
    }
}

// fn main() {
//     yew::start_app::<WelcomeModel>();
// }