use yew::prelude::*;

use crate::components::residents::ResidentsComponent;

mod residents;

#[derive(Debug)]
pub(crate) struct MainComponent {}

impl Component for MainComponent {
    type Message = residents::Msg;
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
                <ResidentsComponent />
            </>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
