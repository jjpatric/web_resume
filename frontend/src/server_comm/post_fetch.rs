use yew::{prelude::*};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::agent::{Bridged};
use shared::GameData;
use crate::server_comm::post_agent;


#[derive(Debug)]
pub enum Msg {
    AddGame(Vec<GameData>),
    RedrawPage,
}

pub struct GamePoster {
    _fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    _error: Option<String>,
    _producer: Box<dyn Bridge<post_agent::PostAgent>>,
}

impl Component for GamePoster {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let this = Self {
            _fetch_task: None,
            _producer: post_agent::PostAgent::bridge(link.callback(Msg::AddGame)),
            link, // _producer borrows link so it goes first
            _error: None,
        };
        this
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            AddGame(game_vec) => {
                // 1. build the request
                if game_vec.len() != 1 {
                    return false;
                }
                let game = &game_vec[0];
                let json_data = Ok(serde_json::to_string(&game).unwrap());
                let request = Request::post("/games")
                    .header("Content-Type", "application/json")
                    .body(json_data)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|_response: Response<Result<String, anyhow::Error>>| {
                            Msg::RedrawPage
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self._fetch_task = Some(task);

                false
            },
            RedrawPage => {
                self._fetch_task = None;
                // we want to redraw so that we update game_history
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {}
    }
}