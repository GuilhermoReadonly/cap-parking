use std::error::Error;

use caparking_lib::{DecodedToken, ResidentSafe as ResidentLib};
use log::warn;
use yew::prelude::*;
use yew_router::{components::Link, history::History, prelude::RouterScopeExt};

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
    GetResidents,
    GetResidentsResponse(Result<Vec<ResidentLib>, Box<dyn Error>>),
}

#[derive(Debug, PartialEq, Properties)]
pub(super) struct PageProperties {
    pub token: Option<DecodedToken>,
}

#[derive(Debug)]
pub(super) struct ResidentsComponent {
    residents: Vec<Resident>,
}

impl Component for ResidentsComponent {
    type Message = Msg;
    type Properties = PageProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let residents = Vec::default();

        if ctx.props().token.is_some() {
            ctx.link().send_message(Msg::GetResidents);
        } else {
            warn!("Not authenticated, go to login page...");
            ctx.link()
                .history()
                .expect("history should be available")
                .push(AppRoute::Login);
        }

        Self { residents }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <table>
                    <caption>{"Résidents"}</caption>
                    <thead>
                        <tr>
                            <th>{"Id"}</th>
                            <th>{"Nom"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for self.residents.iter().map(|item|
                            {
                                html! {
                                    <tr>
                                        <td>{item.resident.id}</td>
                                        <td>
                                            <Link<AppRoute> to={AppRoute::Resident{id:item.resident.id}}>
                                                {&item.resident.name}
                                            </Link<AppRoute>>
                                        </td>
                                    </tr>
                                }
                            }
                        )}
                    </tbody>
                    <tfoot>
                        <tr>
                            <th colspan="2">{format!("Total: {}", self.residents.len())}</th>
                        </tr>
                    </tfoot>
                </table>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Message received: {:?}", msg);

        match msg {
            Msg::GetResidents => {
                let token = ctx.props().token.clone();

                ctx.link().send_future(async move {
                    match request::<(), _>("GET", "/api/residents", None, token).await {
                        Ok(data) => Msg::GetResidentsResponse(Ok(data)),
                        Err(err) => Msg::GetResidentsResponse(Err(Box::new(err))),
                    }
                });
                true
            }
            Msg::GetResidentsResponse(response) => {
                match response {
                    Ok(residents) => {
                        self.residents = residents.into_iter().map(|r| Resident::from(r)).collect();
                    }
                    Err(e) => {
                        log::error!("Something terrible happened...: {:?}", e);
                        self.residents = vec![];

                        ctx.link()
                            .history()
                            .expect("history should be available")
                            .push(AppRoute::Login);
                    }
                }
                true
            }
        }
    }
}
