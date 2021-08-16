use yew::{prelude::*};

pub struct Welcome;

impl Component for Welcome {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Welcome to my rusty webpage 🦀"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
                <p>
                    {"Not much is going on here right now..."}
                </p>
                <p>{"Patience will bring greatness"}</p>
            </div>
        }
    }
}