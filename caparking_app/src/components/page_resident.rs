use std::error::Error;

use caparking_lib::{DecodedToken, ResidentSafe as ResidentLib};
use log::{debug, warn};
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};

use crate::{components::AppRoute, network::request};

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
    Edit,
    CancelEdit,
    PutResident(ResidentLib),
    PutResidentResponse(Result<ResidentLib, Box<dyn Error>>),
    UpdateName(String),
    UpdateLogin(String),
}

#[derive(Debug)]
pub(super) struct ResidentComponent {
    resident: Option<Resident>,
    edit: bool,
}

#[derive(Debug, PartialEq, Properties)]
pub(super) struct PageProperties {
    pub id: u128,
    pub token: Option<DecodedToken>,
}

impl ResidentComponent {
    fn get_resident(ctx: &Context<Self>) {
        if ctx.props().token.is_some() {
            ctx.link().send_message(Msg::GetResident(ctx.props().id));
        } else {
            warn!("Not authenticated, go to login page...");
            ctx.link()
                .history()
                .expect("history should be available")
                .push(AppRoute::Login);
        }
    }
}

impl Component for ResidentComponent {
    type Message = Msg;
    type Properties = PageProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self::get_resident(ctx);
        Self {
            resident: None,
            edit: false,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        debug!("Properties as changed {:?}", ctx.props());
        Self::get_resident(ctx);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let allowed_to_edit = ctx
            .props()
            .token
            .clone()
            .map_or_else(|| false, |t| t.claims.sub.id.0 == ctx.props().id);
        match (&self.resident, &self.edit) {
            (Some(r), false) => html! {
                <>
                <h2>{&r.resident.name}</h2>
                <p>{format!("Login: {}", &r.resident.login)}</p>
                <p>{format!("Parking: {:?}", &r.resident.parking_spots)}</p>
                {
                    if allowed_to_edit {
                        html! {<button onclick={ctx.link().callback(|_| Msg::Edit)}> {"Modifier"} </button>}
                    } else {
                        html!{}
                    }
                }
                </>
            },
            (Some(r), true) => {
                let resident = r.resident.clone();
                html! {
                    <>
                    <h2>{&r.resident.name}</h2>
                    <p>{"Name: "} <input
                        type="text"
                        value={r.resident.name.clone()}
                        required=true
                        onchange={ctx.link().callback(move |e: Event| {
                            let input: InputElement = e.target_unchecked_into();
                            let value = input.value();
                            Msg::UpdateName(value)
                        })}
                    /></p>
                    <p>{"Login: "} <input
                        type="text"
                        value={r.resident.login.clone()}
                        required=true
                        onchange={ctx.link().callback(move |e: Event| {
                            let input: InputElement = e.target_unchecked_into();
                            let value = input.value();
                            Msg::UpdateLogin(value)
                        })}
                    /></p>
                    <p>{format!("Parking: {:?}", &r.resident.parking_spots)}</p>
                    <button onclick={ctx.link().callback(|_| Msg::CancelEdit)}>{"Annuler"} </button>
                    <button onclick={ctx.link().callback(move|_| {
                        Msg::PutResident(resident.clone())
                    })}>{"Sauvegarder"}</button>
                    </>
                }
            }
            _ => html! {

                <p>{"Something, somewhere, went terribly wrong..."}</p>
            },
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
                        self.resident = None;

                        ctx.link()
                            .history()
                            .expect("history should be available")
                            .push(AppRoute::Login);
                    }
                }
                true
            }
            Msg::Edit => {
                self.edit = true;
                true
            }
            Msg::CancelEdit => {
                self.edit = false;
                true
            }
            Msg::PutResident(resident) => {
                let token = ctx.props().token.clone();
                ctx.link().send_future(async move {
                    match request::<ResidentLib, ResidentLib>(
                        "PUT",
                        &"/api/resident".to_string(),
                        Some(resident),
                        token,
                    )
                    .await
                    {
                        Ok(data) => Msg::PutResidentResponse(Ok(data)),
                        Err(err) => Msg::PutResidentResponse(Err(Box::new(err))),
                    }
                });
                true
            }
            Msg::PutResidentResponse(response) => {
                match response {
                    Ok(resident) => {
                        self.resident = Some(Resident::from(resident));
                    }
                    Err(e) => {
                        log::error!("Something terrible happened...: {:?}", e);
                        self.resident = None;

                        ctx.link()
                            .history()
                            .expect("history should be available")
                            .push(AppRoute::Login);
                    }
                }
                true
            }
            Msg::UpdateName(val) => {
                if let Some(r) = &mut self.resident {
                    r.resident.name = val;
                }
                true
            }
            Msg::UpdateLogin(val) => {
                if let Some(r) = &mut self.resident {
                    r.resident.login = val;
                }
                true
            }
        }
    }
}
