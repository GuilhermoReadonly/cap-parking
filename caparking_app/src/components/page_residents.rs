use caparking_lib::ResidentSafe as ResidentLib;
use yew::prelude::*;
use yew_router::components::Link;

use crate::components::AppRoute;

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
    GetResidentsResponse(Result<Vec<ResidentLib>, anyhow::Error>),
}

#[derive(Debug)]
pub(super) struct ResidentsComponent {
    residents: Vec<Resident>,
}

impl Component for ResidentsComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let residents = Vec::default();

        ctx.link().send_message(Msg::GetResidents);

        Self { residents }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
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
                                            <Link<AppRoute> route={AppRoute::Resident{id:item.resident.id}}>
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Message received: {:?}", msg);

        match msg {
            Msg::GetResidents => {
                // // 1. build the request
                // let request = Request::get("/api/residents")
                //     .header("Authorization", "718718123456")
                //     .body(Nothing)
                //     .expect("Could not build request.");
                // // 2. construct a callback
                // let callback = self.link.callback(
                //     |response: Response<Json<Result<Vec<ResidentLib>, anyhow::Error>>>| {
                //         let Json(data) = response.into_body();
                //         Msg::GetResidentsResponse(data)
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
            Msg::GetResidentsResponse(response) => {
                match response {
                    Ok(residents) => {
                        self.residents = residents.into_iter().map(|r| Resident::from(r)).collect();
                    }
                    Err(e) => {
                        log::error!("Something terrible happened...: {:?}", e);
                        self.residents = vec![]
                    }
                }
                true
            }
        }
    }
}
