use caparking_lib::Resident as ResidentLib;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
};

#[derive(Debug, Default, Clone, Properties)]
struct Resident {
    resident: ResidentLib,
}

impl Resident {
    pub fn _new(name: String, parking_spots: Vec<u32>) -> Self {
        Self {
            resident: ResidentLib {
                id: rand::random(),
                name,
                parking_spots,
            },
        }
    }
}

impl From<ResidentLib> for Resident {
    fn from(item: ResidentLib) -> Self {
        Resident { resident: item }
    }
}

struct ResidentComponent {
    props: Resident,
}

impl Component for ResidentComponent {
    type Message = ();
    type Properties = Resident;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
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

#[derive(Debug)]
pub enum Msg {
    GetResidents,
    GetResidentsResponse(Result<Vec<ResidentLib>, anyhow::Error>),
}

#[derive(Debug)]
struct MainComponent {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    residents: Vec<Resident>,
    fetch_task: Option<FetchTask>,
}

impl Component for MainComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let residents = Vec::default();

        link.send_message(Msg::GetResidents);

        Self {
            link,
            residents,
            fetch_task: None,
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
            {for self.residents.iter().map(|item|
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetResidents => {
                // 1. build the request
                let request = Request::get("/api/residents")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<ResidentLib>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::GetResidentsResponse(data)
                    },
                );
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            Msg::GetResidentsResponse(response) => {
                match response {
                    Ok(residents) => {
                        self.residents = residents.into_iter().map(|r| Resident::from(r)).collect();
                    }
                    _ => self.residents = vec![],
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
        }
    }
}

fn main() {
    yew::start_app::<MainComponent>();
}
