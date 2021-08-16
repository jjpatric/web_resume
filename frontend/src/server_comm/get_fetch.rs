use yew::{format::{Json, Nothing}, prelude::*};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::agent::{Dispatched, Dispatcher};
use shared::GameData;
use crate::server_comm::get_agent;


#[derive(Debug)]
pub enum Msg {
    GetGames,
    ReceiveResponse(Result<Vec<GameData>, anyhow::Error>),
}

pub struct GameFetcher {
    _fetch_task: Option<FetchTask>,
    games: Option<Vec<GameData>>,
    link: ComponentLink<Self>,
    _error: Option<String>,
    my_get_agent: Dispatcher<get_agent::GetAgent>,
}

impl GameFetcher {
    fn view_games(&self) -> Html {
        match self.games {
            Some(_) => html!{},
            None => {
                let cb = self.link.callback(|_| Msg::GetGames);
                cb.emit("");
                html!{}
            }
        }
    }
}

impl Component for GameFetcher {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let this = Self {
            _fetch_task: None,
            games: None,
            link,
            _error: None,
            my_get_agent: get_agent::GetAgent::dispatcher(),
        };
        //log::warn!("create!");
        //this.view_games();
        this
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        //log::warn!("change!");
        self.update(Msg::GetGames);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        //log::warn!("update!");
        use Msg::*;

        match msg {
            GetGames => {
                // 1. build the request
                let request = Request::get("/games")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Vec<GameData>, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self._fetch_task = Some(task);
                // we dont want to redraw the page
                false
            },
            ReceiveResponse(response) => {
                match response {
                    Ok(games) => {
                        self.games = Some(games);
                        // Send games to all subscribers
                        self.my_get_agent
                        .send(get_agent::Request::GetAgentGames(self.games.as_ref().unwrap().to_vec()));
                    }
                    Err(_error) => {
                        self._error = Some(_error.to_string())
                    }
                }
                self._fetch_task = None;

                false
            },
        }
    }

    fn view(&self) -> Html {
        //log::warn!("view!");
        html! {
            {self.view_games()}
        }
    }
}