use yew::prelude::*;

#[derive(Debug)]
pub(crate) struct HomePageComponent;

impl Component for HomePageComponent {
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
            <>
                <h1>{"Cap Parking"}</h1>
                <p>{"Application de partage de places de parking entre voisins"}</p>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
