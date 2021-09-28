use crate::components::residents::ResidentsComponent;
use yew::prelude::*;
use yew_router::{router::Router, Switch};

mod residents;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/residents/{id}"]
    Resident(u32),
    #[to = "/residents"]
    Residents,
    #[to = "/"]
    Index,
}

#[derive(Debug)]
pub(crate) struct MainComponent {}

impl Component for MainComponent {
    type Message = residents::Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
            render = Router::render(|switch: AppRoute| {
                match switch {
                    AppRoute::Residents => html!{<ResidentsComponent />},
                    AppRoute::Resident(id) => html!{id},
                    AppRoute::Index => html!{"Welcome home !!!"},
                }
            })
        />

        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
