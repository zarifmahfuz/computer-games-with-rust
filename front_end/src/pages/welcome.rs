// #![recursion_limit = "11256"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender,Properties, virtual_dom};
// use chrono::prelude::*;
// use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use yew_router::prelude::*;

pub struct WelcomeModel {
    link: ComponentLink<Self>,
    value: String,
}
// pub enum Msg {
//     GotInput(String),
//     Clicked,
// }
impl Component for WelcomeModel {
    type Message = ();
    type Properties = ();
fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    WelcomeModel {
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
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Welcome"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"></hr>
            <p>{"This application contains the following two board games, both in human Vs. human and human Vs. Computer versions."}
            </p>
        
            <ul>
        
                <li>{"Connect 4"}</li>
        
                <li>{"TOOT-OTTO"}</li>
        
        
            </ul>
            <p>{"Select the game of your choice from the side bar, and start playing. Enjoy!"}</p>
        </div>
        }
    }
}

// fn main() {
//     yew::start_app::<WelcomeModel>();
// }