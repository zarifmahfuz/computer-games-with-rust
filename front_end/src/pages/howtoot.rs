#![recursion_limit = "11256"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender,Properties, virtual_dom};
// use chrono::prelude::*;
// use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use yew_router::prelude::*;

pub struct HowToTOOT {
    link: ComponentLink<Self>,
    value: String,
}
// pub enum Msg {
//     GotInput(String),
//     Clicked,
// }
impl Component for HowToTOOT {
    type Message = ();
    type Properties = ();
fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    HowToTOOT {
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
        <div class="w3-container" id="services" style="margin-top:75px">
        <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play TOOT-OTTO"}</b></h5>
        <hr style="width:50px;border:5px solid red" class="w3-round"></hr>
        <p>{"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}
        </p>
        <br></br>
        <div><h5>{"To play TOOT-OTTO follow the following steps:"}</h5></div>
        <ul>
    
            <li>{"A new game describes which player is TOOT and which is OTTO"}</li>
    
            <li>{"Select the disc type T or O that you want to place"}</li>
    
            <li>{"Click on the desired column on the game board to place your disc"}</li>
    
            <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally"}</li>
    
        </ul>
    
      <div> {"For More information on TOOT-OTTO click "} <a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a>
      </div>
      </div>
        }
    }
}

// fn main() {
//     yew::start_app::<WelcomeModel>();
// }