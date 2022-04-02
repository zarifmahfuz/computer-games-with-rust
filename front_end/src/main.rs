#![recursion_limit = "11256"]
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterAnchor;

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

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Route {
    #[to = "/c4computer"]
    Connect4Computer,
    #[to = "/c4human"]
    Connect4Human,
    #[to = "/howc4"]
    HowToConnect4,
    #[to = "/howtoot"]
    HowToToot,
    #[to = "/tootcomputer"]
    TOOTComputer,
    #[to = "/toothuman"]
    TOOTHuman,
    #[to = "/scoreboard"]
    ScoreBoard,
    #[to = "/score"]
    Score,
    #[to = "/"]
    WelcomeModel,
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        html! {
            <>
            <h3 class="w3-padding-64"><b>{"Play"}</b><b>{"Connect4 / TOOT-OTTO"}</b></h3>
            <div>
                <Anchor route=Route::WelcomeModel>
                { "Welcome" }
                </Anchor>
                <br></br>
                <br></br>
                <Anchor route=Route::HowToConnect4>
                { "How To Play Connect4" }
                </Anchor>
                <br></br>
                <Anchor route=Route::Connect4Computer>
                { "Play Connect4 With Computer" }
                </Anchor>
                <br></br>
                <Anchor route=Route::Connect4Human>
                { "Play Connect4 With Another Human" }
                </Anchor>
                <br></br>
                <br></br>
                <Anchor route=Route::HowToToot>
                { "How to Play TOOT-OTTO" }
                </Anchor>
                <br></br>
                <Anchor route=Route::TOOTComputer>
                { "Play Toot-Otto With Computer" }
                </Anchor>
                <br></br>
                <Anchor route=Route::TOOTHuman>
                { "Play Toot-Otto With Another Human" }
                </Anchor>
                <br></br>
                <br></br>
                <Anchor route=Route::ScoreBoard>
                { "View Game History" }
                </Anchor>
                <br></br>
                <Anchor route=Route::Score>
                { "Score Board" }
                </Anchor>
                <br></br>
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
            <main>
                <Router<Route, ()>
                    render = Router::render(|switch: Route| {
                        match switch {
                            Route::WelcomeModel => html!{ <WelcomeModel/> },
                            Route::Connect4Computer => html!{ <Connect4Computer/> },
                            Route::Connect4Human => html!{ <Connect4Human/> },
                            Route::HowToConnect4 => html!{ <HowToConnect4/> },
                            Route::HowToToot => html!{ <HowToTOOT/> },
                            Route::TOOTComputer => html!{ <TOOTComputer/> },
                            Route::TOOTHuman => html!{ <TOOTHuman/> },
                            Route::ScoreBoard => html!{ <ScoreBoard/> },
                            Route::Score => html!{ <Score/> },
                        }
                    })
                />
            </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}