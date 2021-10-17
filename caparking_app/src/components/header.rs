use yew::prelude::*;
use yew_router::components::Link;

use crate::components::AppRoute;

#[derive(Debug)]
pub struct HeaderComponent {}

impl Component for HeaderComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="topnav">
                <Link<AppRoute> route={AppRoute::Home}>
                    {"Accueil"}
                </Link<AppRoute>>
                <Link<AppRoute> route={AppRoute::Residents}>
                    {"Résidents"}
                </Link<AppRoute>>
                <Link<AppRoute> route={AppRoute::Login} classes={classes!("login-elt")}>
                    {"Login"}
                </Link<AppRoute>>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
