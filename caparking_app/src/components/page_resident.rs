use caparking_lib::ResidentSafe as ResidentLib;
use yew::prelude::*;

#[derive(Debug, Default, PartialEq, Properties)]
struct Resident {
    resident: ResidentLib,
}

impl From<ResidentLib> for Resident {
    fn from(item: ResidentLib) -> Self {
        Resident { resident: item }
    }
}

#[derive(Debug)]
pub enum Msg {
    GetResident(u128),
    GetResidentResponse(Result<ResidentLib, anyhow::Error>),
}

#[derive(Debug)]
pub(super) struct ResidentComponent {
    resident: Option<Resident>,
}

#[derive(Debug, PartialEq, Properties)]
pub(super) struct PageProperties {
    pub id: u128,
}

impl Component for ResidentComponent {
    type Message = Msg;
    type Properties = PageProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetResident(ctx.props().id));

        Self { resident: None }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            match &self.resident {
                Some(r) => html! {
                    <>
                    <h2>{&r.resident.name}</h2>
                    <p>{format!("Id: {}", &r.resident.id)}</p>
                    <p>{format!("Login: {}", &r.resident.login)}</p>
                    <p>{format!("Parking: {:?}", &r.resident.parking_spots)}</p>
                    </>
                },
                _ => html! {
                    <p>{"Something, somewhere, went terribly wrong..."}</p>
                }
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Message received: {:?}", msg);

        match msg {
            Msg::GetResident(id) => {
                // 1. build the request
                // let request = Request::get(format!("/api/resident/{}", id))
                //     .body(Nothing)
                //     .expect("Could not build request.");
                // // 2. construct a callback
                // let callback = self.link.callback(
                //     |response: Response<Json<Result<ResidentLib, anyhow::Error>>>| {
                //         let Json(data) = response.into_body();
                //         Msg::GetResidentResponse(data)
                //     },
                // );
                // // 3. pass the request and callback to the fetch service
                // let task = FetchService::fetch(request, callback).expect("failed to start request");
                // // 4. store the task so it isn't canceled immediately
                // self.fetch_task = Some(task);
                // // we want to redraw so that the page displays a 'fetching...' message to the user
                // // so return 'true'
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
                true
            }
        }
    }
}
