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
pub enum GlobalMsg {
    NewToken(String),
}

#[derive(Debug)]
pub(crate) struct MainComponent{
    token: Option<String>
}

impl Component for MainComponent {
    type Message = GlobalMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{token: None}
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let token = self.token.clone();

        let cb = ctx.link().callback(|token: String| {
            log::info!("Callback activated: {:?}", token);
            GlobalMsg::NewToken(token)
        });

        html! {
            <div class="grid-container">
                <div class="header">
                    <HeaderComponent/>
                </div>
                <div class="content">
                <Router<AppRoute>
                    render={Router::render(move |routes: &AppRoute| {
                        match routes {
                            AppRoute::Index => html!{<HomePageComponent/>},
                            AppRoute::Home => html!{<HomePageComponent/>},
                            AppRoute::Residents => html!{<ResidentsComponent token={token.clone()} />},
                            AppRoute::Resident{id} => html!{<ResidentComponent id={id.clone()} token={token.clone()}/>},
                            AppRoute::Login => html!{<LoginPageComponent update_token_callback={cb.clone()}/>},
                        }
                    })}
                />
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Message received: {:?}", msg);

        match msg {
            GlobalMsg::NewToken(t) => {
                self.token = Some(t);
                true
            }

        }
    }
}
