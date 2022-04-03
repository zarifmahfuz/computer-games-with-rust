#![recursion_limit = "11256"]
use yew::prelude::*;
use yew_router::prelude::*;

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
            howc4::HowToConnect4};

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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>

                <h3 class="w3-padding-64"><b>{"Play"}</b><b>{"Connect4 / TOOT-OTTO"}</b></h3>

                <div>
                    <Link<Route> to={Route::WelcomeModel}>{ "Welcome" }</Link<Route>>
                    <br/>
                    <br/>
                    
                    <Link<Route> to={Route::HowToConnect4}>{ "How To Play Connect4" }</Link<Route>>
                    <br/>
                    
                    <Link<Route> to={Route::Connect4Computer}>{ "Play Connect4 With Computer" }</Link<Route>>
                    <br/>
                    
                    <Link<Route> to={Route::Connect4Human}>{ "Play Connect4 With Another Human" }</Link<Route>>
                    <br/>
                    <br/>

                    <Link<Route> to={Route::HowToToot}>{ "How to Play TOOT-OTTO" }</Link<Route>>
                    <br/>

                    <Link<Route> to={Route::TOOTComputer}>{ "Play Toot-Otto With Computer" }</Link<Route>>
                    <br/>

                    <Link<Route> to={Route::TOOTHuman}>{ "Play Toot-Otto With Another Human" }</Link<Route>>
                    <br/>
                    <br/>

                    <Link<Route> to={Route::ScoreBoard}>{ "View Game History" }</Link<Route>>
                    <br/>

                    <Link<Route> to={Route::Score}>{ "Score Board" }</Link<Route>>
                    <br/>
                </div>


                // <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav"><br>
                //     <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%">Close Menu</a>
                //     <div class="w3-container">
                //         <h3 class="w3-padding-64"><b>Play<br>Connect4 / TOOT-OTTO</b></h3>
                //     </div>
                //     <a href="#/HowToConnect4" class="w3-padding w3-hover-white">How to Play Connect4</a>
                //     <a href="#/Connect4Computer" class="w3-padding w3-hover-white">Play Connect4 With Computer</a> 
                //     <a href="#/Connect4Human" class="w3-padding w3-hover-white">Play Connect4 with Another Human</a> 
                //     <br></br>
                //     <a href="#/HowToToot" class="w3-padding w3-hover-white">How to Play TOOT-OTTO</a>
                //     <a href="#/TootOttoComputer" class="w3-padding w3-hover-white">Play Toot-Otto With Computer</a>
                //     <a href="#/TootOttoHuman" class="w3-padding w3-hover-white">Play Toot-Otto With Another Human</a>
                //     <br></br>
                //     <a href="#/ScoreBoard" class="w3-padding w3-hover-white">View Game History</a>
                //     <a href="#/Scores" class="w3-padding w3-hover-white">Score Board</a>
                // </nav>
            
            </BrowserRouter>
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
    yew::Renderer::<App>::new().render();
}