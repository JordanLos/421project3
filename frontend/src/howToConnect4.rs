use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct HowToConnect4 {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props { }

pub enum Msg {}


impl Component for HowToConnect4 {
    type Message = Msg;
    type Properties = Props;

    fn create (props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HowToConnect4 { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        html !{
            <>
                <form ng-submit="Game()">
                  <div class="w3-container" id="services" style="margin-top:75px">
                    <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play Connect 4"}</b></h5>
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
                    <p>{"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}
                    </p>
                    <br></br>
                    <div><h5>{"To play Connect 4 follow the following steps:"}</h5></div>
                    <ul>

                        <li>{"A new game describes discs of which color belongs to which player"}</li>

                        <li>{"Click on the desired column on the game board to place your disc"}</li>

                        <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>

                    </ul>
                  <br/> {"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a>
                  </div>
                </form>
            </>

        }
    }
}