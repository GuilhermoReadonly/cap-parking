use crate::components::{
    header::HeaderComponent, page_home::HomePageComponent, page_resident::ResidentComponent,
    page_residents::ResidentsComponent,
};
use yew::prelude::*;
use yew_router::{router::Router, Switch};

mod header;
mod page_home;
mod page_resident;
mod page_residents;

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
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="grid-container">
                <div class="header">
                    <HeaderComponent/>
                </div>
                <div class="content">
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Residents => html!{<ResidentsComponent/>},
                            AppRoute::Resident(id) => html!{<ResidentComponent id=id/>},
                            AppRoute::Index => html!{<HomePageComponent/>},
                        }
                    })
                />
                </div>
            </div>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
