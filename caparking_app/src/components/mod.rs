use crate::components::{
    header::HeaderComponent, page_home::HomePageComponent, page_login::LoginPageComponent,
    page_resident::ResidentComponent, page_residents::ResidentsComponent,
};
use caparking_lib::{Claims, DecodedToken};
use jsonwebtoken::dangerous_insecure_decode;
use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

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
    NewToken(DecodedToken),
}

#[derive(Debug)]
pub(crate) struct MainComponent {
    token: Option<DecodedToken>,
}

impl Component for MainComponent {
    type Message = GlobalMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { token: None }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let token = self.token.clone();

        let cb = ctx.link().callback(|token: String| {
            log::info!("Callback activated: {:?}", token);

            // can't use dangerous_insecure_decode_with_validation() because it seems to not compile to wasm
            let claims = dangerous_insecure_decode::<Claims>(
                &token,
                //&Validation::default(),
            )
            .expect("Token decoding failed")
            .claims;
            let decoded_token = DecodedToken {
                raw_token: token,
                claims,
            };
            info!("Jwt decoded: {decoded_token:?}");
            GlobalMsg::NewToken(decoded_token)
        });

        html! {
            <div class="grid-container">
                <BrowserRouter>
                    <div class="header">
                        <HeaderComponent token={token.clone()}/>
                    </div>
                    <div class="content">
                        <Switch<AppRoute> render={Switch::render(move |routes: &AppRoute| {
                            info!("Route: {:?}", routes);
                            match routes.clone() {
                                AppRoute::Index => html!{<HomePageComponent/>},
                                AppRoute::Home => html!{<HomePageComponent/>},
                                AppRoute::Residents => html!{<ResidentsComponent token={token.clone()} />},
                                AppRoute::Resident{id} => html!{<ResidentComponent id={id.clone()} token={token.clone()}/>},
                                AppRoute::Login => html!{<LoginPageComponent update_token_callback={cb.clone()}/>},
                            }
                        })} />
                    </div>
                </BrowserRouter>
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
