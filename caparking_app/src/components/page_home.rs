use yew::prelude::*;

#[derive(Debug)]
pub(crate) struct HomePageComponent {}

impl Component for HomePageComponent {
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
            <>
                <h1>{"Cap Parking"}</h1>
                <p>{"Application de partage de places de parking entre voisins"}</p>
            </>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
