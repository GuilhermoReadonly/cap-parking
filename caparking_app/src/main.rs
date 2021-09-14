use caparking_lib::Resident as ResidentLib;
use yew::prelude::*;

#[derive(Debug, Default, Clone, Properties)]
struct Resident {
    resident: ResidentLib,
}

impl Resident {
    fn new(name: String, parking_spots: Vec<u32>) -> Self {
        Self {
            resident: ResidentLib {
                id: rand::random(),
                name,
                parking_spots,
            },
        }
    }
}

struct ResidentComponent {
    _link: (),
    props: Resident,
}

impl Component for ResidentComponent {
    type Message = ();
    type Properties = Resident;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { _link: (), props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <tr>
                <th>{&self.props.resident.id}</th>
                <th>{&self.props.resident.name}</th>
            </tr>
        }
    }
}

#[derive(Debug, Default, Clone)]
struct MainComponent {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: (),
    props: Vec<Resident>,
}

impl Component for MainComponent {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut props = Vec::default();
        props.push(Resident::new("Plop1".to_string(), vec![]));
        props.push(Resident::new("Plop2".to_string(), vec![]));
        props.push(Resident::new("Plop3".to_string(), vec![]));
        Self {
            link: (),
            props: props,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
        <table>
            <tr>
                <th>{"Id"}</th>
                <th>{"Name"}</th>
            </tr>
            {for self.props.iter().map(|item|
                {
                    html! {
                        <>
                            <ResidentComponent with item.clone() />
                        </>
                    }
                }
                )}
        </table> }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

fn main() {
    yew::start_app::<MainComponent>();
}
