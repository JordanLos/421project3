use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct Scores {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props { }

pub enum Msg {}


impl Component for Scores {
    type Message = Msg;
    type Properties = Props;

    fn create (props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Scores { props }
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
                <div class="w3-container" id="services" style="margin-top:75px">
                    <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
                    <div><h4>{"Games Won by Computer"}</h4></div>
                    <table>
                            <tr>
                                <th>{"Total Games Played"}</th>
                                <th>{"Games Against Computer"}</th>
                                <th>{"Games Computer Won"}</th>
                            </tr>
                            <tr>
                                 <td>{"{{games.length}}"}</td>
                                 <td>{"{{(games|filter:{Player2Name:'Computer'}).length}}"}</td>
                                 <td>{"{{(games|filter:{WinnerName:'Computer'}).length}}"}</td>
                            </tr>
                    </table>

                    <br/>

                    <div>{"<h4>Details of Games Won by Computer<h4>"}</div>
                    <div id="game-stream">
                        <table>
                            <tr>
                                <th>{"Sl. No."}</th>
                                <th>{"Game Type"}</th>
                                <th>{"Winner"}</th>
                                <th>{"Played Against"}</th>
                                <th>{"When Played"}</th>
                            </tr>
                            <tr ng-repeat="game in games | filter:{WinnerName:'Computer'}">
                                 <td>{"{{ $index + 1 }}"}</td>
                                 <td>{"{{game.gameType}}"}</td>
                                 <td>{"{{game.WinnerName}}"}</td>
                                 <td>{"{{game.Player1Name}}"}</td>
//                                 <td>{"{{game.GameDate | date:"h:mma 'on' MMM d, y"}}"}</td>
                            </tr>
                        </table>

                        <br/>

                        <div><h4>{"Details of Games Won by All Players"}</h4></div>
                        <div id="game-stream">
                            <table>
                                <tr>
                                    <th>{"Sl. No."}</th>
                                    <th>{"Winner or Draw"}</th>
                                    <th>{"No. of Wins"}</th>
                                </tr>
//                                <tr ng-repeat="game in games | groupBy:"WinnerName" | toArray:true | orderBy: $value.length | reverse">
                                <tr>
                                     <td>{"{{ $index + 1 }}"}</td>
                                     <td>{"{{game.$key}}"}</td>
                                     <td>{"{{game.length}}"}</td>
                                </tr>
                            </table>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}