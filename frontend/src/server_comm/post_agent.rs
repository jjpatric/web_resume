use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;
use shared::GameData;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    PostAgentSave(Vec<GameData>),
}

pub struct PostAgent {
    link: AgentLink<PostAgent>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for PostAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Vec<GameData>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::PostAgentSave(game_list) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, game_list.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}