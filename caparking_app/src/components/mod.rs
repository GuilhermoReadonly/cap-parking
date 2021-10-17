use crate::components::{
    header::HeaderComponent, page_home::HomePageComponent, page_login::LoginPageComponent,
    page_resident::ResidentComponent, page_residents::ResidentsComponent,
};
use yew::prelude::*;
use yew_router::{router::Router, Routable};

mod header;
mod page_home;
mod page_login;
mod page_resident;
mod page_residents;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/app/resident/:id")]
    Resident { id: u128 },
    #[at("/app/residents")]
    Residents,
    #[at("/app/login")]
    Login,
    #[at("/app")]
    Home,
    #[at("/")]
    Index,
}

#[derive(Debug)]
pub(crate) struct MainComponent;

impl Component for MainComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="grid-container">
                <div class="header">
                    <HeaderComponent/>
                </div>
                <div class="content">
                <Router<AppRoute>
                    render={Router::render(|routes: &AppRoute| {
                        match routes {
                            AppRoute::Index => html!{<HomePageComponent/>},
                            AppRoute::Home => html!{<HomePageComponent/>},
                            AppRoute::Residents => html!{<ResidentsComponent/>},
                            AppRoute::Resident{id} => html!{<ResidentComponent id={id.clone()}/>},
                            AppRoute::Login => html!{<LoginPageComponent/>},
                        }
                    })}
                />
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
