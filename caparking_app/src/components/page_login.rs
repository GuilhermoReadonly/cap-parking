use caparking_lib::{LoginForm, Token};
use log::{error, info};
use yew::{
    format::Json,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
};

#[derive(Debug)]
pub enum Msg {
    PostLogin(LoginForm),
    PostLoginResponse(Result<Token, anyhow::Error>),
}

#[derive(Debug)]
pub(crate) struct LoginPageComponent {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

impl Component for LoginPageComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let send_login = self.link.callback(|_| {
            info!("Click on login button !");
            Msg::PostLogin(LoginForm {
                login: "plop@plop".to_string(),
                password: "718718718".to_string(),
            })
        });

        html! {
            <>
                <h1>{"Login"}</h1>
                <label for="uname">{"Login"}</label>
                <input type="text" placeholder="login" name="uname" required=true />
                <br/>
                <label for="psw">{"Mot de passe"}</label>
                <input type="password" placeholder="Mot de passe" name="psw" required=true />
                <br/>
                <button type="submit" onclick={send_login}>{"Login"}</button>
            </>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("Message received: {:?}", msg);

        match msg {
            Msg::PostLogin(login_form) => {
                // 1. build the request
                let request = Request::post("/api/login")
                    .body(Json(&login_form))
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Token, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::PostLoginResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            Msg::PostLoginResponse(response) => {
                match response {
                    Ok(s) => {
                        info!("return: {:?}", s);
                    }
                    Err(e) => {
                        error!("Something terrible happened...: {:?}", e);
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }
}
