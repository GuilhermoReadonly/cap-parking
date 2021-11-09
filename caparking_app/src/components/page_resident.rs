use std::error::Error;

use caparking_lib::ResidentSafe as ResidentLib;
use yew::prelude::*;

use crate::network::request;

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
    GetResidentResponse(Result<ResidentLib, Box<dyn Error>>),
}

#[derive(Debug)]
pub(super) struct ResidentComponent {
    resident: Option<Resident>,
}

#[derive(Debug, PartialEq, Properties)]
pub(super) struct PageProperties {
    pub id: u128,
    pub token: Option<String>,
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Message received: {:?}", msg);

        match msg {
            Msg::GetResident(id) => {
                let token = ctx.props().token.clone();
                ctx.link().send_future(async move {
                    match request::<(), ResidentLib>(
                        "GET",
                        &format!("/api/resident/{}", id),
                        None,
                        token,
                    )
                    .await
                    {
                        Ok(data) => Msg::GetResidentResponse(Ok(data)),
                        Err(err) => Msg::GetResidentResponse(Err(Box::new(err))),
                    }
                });
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
