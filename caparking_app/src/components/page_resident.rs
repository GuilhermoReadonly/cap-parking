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

#[derive(Debug)]
pub enum Msg {
    GetResident(u32),
    GetResidentResponse(Result<ResidentLib, anyhow::Error>),
}

#[derive(Debug)]
pub(super) struct ResidentComponent {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: PageProperties,
    resident: Option<Resident>,
    fetch_task: Option<FetchTask>,
}

#[derive(Debug, Clone, Properties)]
pub(super) struct PageProperties {
    pub id: u32,
}

impl Component for ResidentComponent {
    type Message = Msg;
    type Properties = PageProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetResident(props.id));

        Self {
            link,
            props: PageProperties{id: 0},
            resident: None,
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
            <>
                {format!("ID: {} \nResident: {:?}", self.props.id, self.resident)}
            </>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("Message received: {:?}", msg);

        match msg {
            Msg::GetResident(id) => {
                // 1. build the request
                let request = Request::get(format!("/api/resident/{}", id))
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback = self.link.callback(
                    |response: Response<Json<Result<ResidentLib, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::GetResidentResponse(data)
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
            Msg::GetResidentResponse(response) => {
                match response {
                    Ok(resident) => {
                        self.resident = Some(Resident::from(resident));
                    }
                    Err(e) => {
                        log::error!("Something terrible happened...: {:?}", e);
                        self.resident = None
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }
}
