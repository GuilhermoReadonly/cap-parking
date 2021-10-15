use crate::components::{
    header::HeaderComponent, page_home::HomePageComponent, page_login::LoginPageComponent,
    page_resident::ResidentComponent, page_residents::ResidentsComponent,
};
use yew::prelude::*;
use yew_router::{router::Router, Switch};

mod header;
mod page_home;
mod page_login;
mod page_resident;
mod page_residents;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/app/resident/{id}"]
    Resident(u128),
    #[to = "/app/residents"]
    Residents,
    #[to = "/app/login"]
    Login,
    #[to = "/app"]
    Home,
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
                            AppRoute::Index => html!{<HomePageComponent/>},
                            AppRoute::Home => html!{<HomePageComponent/>},
                            AppRoute::Residents => html!{<ResidentsComponent/>},
                            AppRoute::Resident(id) => html!{<ResidentComponent id=id/>},
                            AppRoute::Login => html!{<LoginPageComponent/>},
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
