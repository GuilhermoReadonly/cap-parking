use caparking_lib::{LoginForm, LoginResponse};
use log::{error, info};
use std::{error::Error, fmt::Debug};

use web_sys::{HtmlInputElement as InputElement, KeyboardEvent};
use yew::{events::Event, html, Component, Context, Html, TargetCast};

use crate::network::request;

#[derive(Debug)]
pub enum Msg {
    SendLogin,
    PostLoginResponse(Result<LoginResponse, Box<dyn Error>>),
    PostLoginFetching,
    UpdateLogin(String),
    UpdatePassword(String),
}

#[derive(Debug)]
pub(crate) struct LoginPageComponent {
    login_form: LoginForm,
}

impl Component for LoginPageComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            login_form: LoginForm::default(),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{"Login"}</h1>
                <label for="uname">{"Login"}</label>
                <input
                    type="text"
                    placeholder="login"
                    name="uname"
                    required=true
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: InputElement = e.target_unchecked_into();
                        let value = input.value();
                        Msg::UpdateLogin(value)
                    })}
                />
                <br/>
                <label for="psw">{"Mot de passe"}</label>
                <input
                    type="password"
                    placeholder="Mot de passe"
                    name="psw"
                    required=true
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: InputElement = e.target_unchecked_into();
                        let value = input.value();
                        Msg::UpdatePassword(value)
                    })}
                    onkeypress={ctx.link().batch_callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" {
                            Some(Msg::SendLogin)
                        } else {
                            None
                        }
                    })}
                />
                <br/>
                <button
                    type="submit"
                    onclick={ctx.link().callback(|_| Msg::SendLogin)}
                >{"Login"}</button>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Message received: {:?}", msg);

        match msg {
            Msg::SendLogin => {
                let login_form = self.login_form.clone();
                ctx.link().send_future(async {
                    match request("POST", "/api/login", Some(login_form)).await {
                        Ok(login_response) => Msg::PostLoginResponse(Ok(login_response)),
                        Err(err) => Msg::PostLoginResponse(Err(Box::new(err))),
                    }
                });
                ctx.link().send_message(Msg::PostLoginFetching);
            }
            Msg::PostLoginResponse(response) => match response {
                Ok(s) => {
                    info!("return: {:?}", s);
                }
                Err(e) => {
                    error!("Something terrible happened...: {:?}", e);
                }
            },
            Msg::UpdateLogin(val) => {
                self.login_form.login = val;
            }
            Msg::UpdatePassword(val) => {
                self.login_form.password = val;
            }
            _ => (),
        };
        true
    }
}
