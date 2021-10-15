use yew::prelude::*;
use yew_router::components::RouterAnchor;

use crate::components::AppRoute;

#[derive(Debug)]
pub struct HeaderComponent {}

impl Component for HeaderComponent {
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
            <div class="topnav">
                <RouterAnchor<AppRoute> route=AppRoute::Home>
                    {"Accueil"}
                </RouterAnchor<AppRoute>>
                <RouterAnchor<AppRoute> route=AppRoute::Residents>
                    {"Résidents"}
                </RouterAnchor<AppRoute>>
                <RouterAnchor<AppRoute> route=AppRoute::Login classes="login-elt">
                    {"Login"}
                </RouterAnchor<AppRoute>>
            </div>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
