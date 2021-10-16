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
    SendLogin,
    PostLoginResponse(Result<Token, anyhow::Error>),
    UpdateLogin(String),
    UpdatePassword(String),
}

#[derive(Debug)]
pub(crate) struct LoginPageComponent {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    login_form: LoginForm,
}

impl Component for LoginPageComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            login_form: LoginForm::default(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Login"}</h1>
                <label for="uname">{"Login"}</label>
                <input
                    type="text"
                    placeholder="login"
                    name="uname"
                    required=true
                    oninput=self.link.callback(|e: InputData| Msg::UpdateLogin(e.value))
                />
                <br/>
                <label for="psw">{"Mot de passe"}</label>
                <input
                    type="password"
                    placeholder="Mot de passe"
                    name="psw"
                    required=true
                    oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))
                    onkeypress=self.link.batch_callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Some(Msg::SendLogin) } else { None }
                    })
                />
                <br/>
                <button
                    type="submit"
                    onclick=self.link.callback(|_| Msg::SendLogin)
                >{"Login"}</button>
            </>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("Message received: {:?}", msg);

        match msg {
            Msg::SendLogin => {
                // 1. build the request
                let request = Request::post("/api/login")
                    .body(Json(&self.login_form))
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
            }
            Msg::UpdateLogin(val) => {
                self.login_form.login = val;
            }
            Msg::UpdatePassword(val) => {
                self.login_form.password = val;
            }
        };
        true
    }
}
