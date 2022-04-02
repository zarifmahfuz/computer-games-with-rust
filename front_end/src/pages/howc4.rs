#![recursion_limit = "11256"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender,Properties, virtual_dom};
// use chrono::prelude::*;
// use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use yew_router::prelude::*;

pub struct HowToConnect4 {
    link: ComponentLink<Self>,
    value: String,
}
// pub enum Msg {
//     GotInput(String),
//     Clicked,
// }
impl Component for HowToConnect4 {
    type Message = ();
    type Properties = ();
fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    HowToConnect4 {
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
        <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play Connect 4"}</b></h5>
        <hr style="width:50px;border:5px solid red" class="w3-round"></hr>
        <p>{"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}
        </p>
        <br></br>
        <div><h5>{"To play Connect 4 follow the following steps:"}</h5></div>
        <ul>
    
            <li>{"A new game describes discs of which color belongs to which player"}</li>
    
            <li>{"Click on the desired column on the game board to place your disc"}</li>
    
            <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>
    
        </ul>
      <div> {"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a></div> 
      </div>
        }
    }
}

// fn main() {
//     yew::start_app::<WelcomeModel>();
// }