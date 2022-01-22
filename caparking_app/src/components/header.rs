use caparking_lib::DecodedToken;
use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::AppRoute;

#[derive(Debug)]
pub(super) struct HeaderComponent;

#[derive(Debug, PartialEq, Properties)]
pub(super) struct PageProperties {
    pub token: Option<DecodedToken>,
}

impl Component for HeaderComponent {
    type Message = ();
    type Properties = PageProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let token = &ctx.props().token;
        info!("Token in header: {token:?}");
        html! {
            <div class="topnav">
                <Link<AppRoute> to={AppRoute::Home}>{"Accueil"}</Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Residents}>{"Résidents"}</Link<AppRoute>>
                {
                    match ctx.props().token.clone() {
                        Some(t) => html!{<Link<AppRoute> to={AppRoute::Resident{ id: t.claims.sub.id }} classes={classes!("login-elt")}>{t.claims.sub.name}</Link<AppRoute>>},
                        None => html!{<Link<AppRoute> to={AppRoute::Login} classes={classes!("login-elt")}>{"Login"}</Link<AppRoute>>},
                    }
                }
                
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
