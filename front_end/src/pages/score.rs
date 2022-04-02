#![recursion_limit = "11256"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender,Properties, virtual_dom};
// use chrono::prelude::*;
// use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use yew_router::prelude::*;

pub struct Score {
    link: ComponentLink<Self>,
    value: String,
}
// pub enum Msg {
//     GotInput(String),
//     Clicked,
// }
impl Component for Score {
    type Message = ();
    type Properties = ();
fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
  Score {
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
      <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
      <hr style="width:50px;border:5px solid red" class="w3-round"></hr>
      <div><h4>{"Games Won by Computer"}</h4></div>
          <table>
              <tr>
                  <th>{"Total Games Played"}</th>
                  <th>{"Games Against Computer"}</th>
                  <th>{"Games Computer Won"}</th>
              </tr>
          </table>
      <br></br>
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
           </table>
      </div>
      <br></br>
      <div><h4>{"Details of Games Won by All Players"}</h4></div>
      <div id="game-stream">
          <table>
              <tr>
                  <th>{"Sl. No."}</th>
                  <th>{"Winner or Draw"}</th>
                  <th>{"No. of Wins"}</th>
              </tr>
              // {}
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