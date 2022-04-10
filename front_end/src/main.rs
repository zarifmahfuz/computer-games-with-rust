#![recursion_limit = "11256"]
use yew::prelude::*;
use yew_router::prelude::*;
use yew::html::Scope;

mod pages;
use pages::{
            welcome::WelcomeModel,
            c4computer::Connect4Computer,
            c4human::Connect4Human,
            howtoot::HowToTOOT,
            tootcomputer::TOOTComputer,
            toothuman::TOOTHuman,
            scoreboard::ScoreBoard,
            score::Score,
            howc4::HowToConnect4,
        };

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/c4computer")]
    Connect4Computer,
    #[at("/c4human")]
    Connect4Human,
    #[at("/howc4")]
    HowToConnect4,
    #[at("/howtoot")]
    HowToToot,
    #[at("/tootcomputer")]
    TOOTComputer,
    #[at("/toothuman")]
    TOOTHuman,
    #[at("/scoreboard")]
    ScoreBoard,
    #[at("/score")]
    Score,
    #[at("/")]
    WelcomeModel,
}

struct App;

impl Component for App {
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

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>

                <h3 class="w3-padding-64"><b>{"Play"}</b><b>{"Connect4 / TOOT-OTTO"}</b></h3>
            
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        html! {
            <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" 
                style="z-index:3;width:350px;font-weight:bold" id="mySidenav"><br/>
                <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" 
                style="width:100%">{"Close Menu"}</a>
                <div class="w3-container">
                    <h3 class="w3-padding-64"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
                </div>
                <Link<Route> to={Route::HowToConnect4}>{ "How To Play Connect4" }</Link<Route>>
                // <br/>

                <Link<Route> to={Route::Connect4Computer}>{ "Play Connect4 With Computer" }</Link<Route>>
                // <br/>
                
                <Link<Route> to={Route::Connect4Human}>{ "Play Connect4 With Another Human" }</Link<Route>>
                // <br/>
                <br/>

                <Link<Route> to={Route::HowToToot}>{ "How to Play TOOT-OTTO" }</Link<Route>>
                // <br/>

                <Link<Route> to={Route::TOOTComputer}>{ "Play Toot-Otto With Computer" }</Link<Route>>
                // <br/>

                <Link<Route> to={Route::TOOTHuman}>{ "Play Toot-Otto With Another Human" }</Link<Route>>
                // <br/>
                <br/>

                <Link<Route> to={Route::ScoreBoard}>{ "View Game History" }</Link<Route>>
                // <br/>

                <Link<Route> to={Route::Score}>{ "Score Board" }</Link<Route>>
                // <br/>
            </nav>
            
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::WelcomeModel => html! { <WelcomeModel/> },
        Route::Connect4Computer => html! { <Connect4Computer/> },
        Route::Connect4Human => html! { <Connect4Human/> },
        Route::HowToConnect4 => html! { <HowToConnect4/> },
        Route::HowToToot => html!{ <HowToTOOT/> },
        Route::TOOTComputer => html! { <TOOTComputer/> },
        Route::TOOTHuman => html! { <TOOTHuman/> },
        Route::ScoreBoard => html! { <ScoreBoard/> },
        Route::Score => html!{ <Score/> },
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}