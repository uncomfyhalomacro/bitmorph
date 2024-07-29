#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Base {},

    #[route("/user/:is_logged_in")]
    LoginPage { is_logged_in: bool },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn AppTitle(is_logged_in: bool) -> Element {
    if is_logged_in {
        rsx! {
                h1 { "BitMorph" }
        }
    } else {
        rsx! {
                h1 { "BitMorph - Login" }
        }
    }
}

#[component]
/// TODO: Make this use a Vec<Element> parameter in the future
fn NavBar(is_logged_in: bool) -> Element {
    rsx! {
        AppTitle { is_logged_in }
    }
}

#[component]
fn LoginPage(is_logged_in: bool) -> Element {
    let mut user = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    rsx! {
        NavBar { is_logged_in }
        div {
            display: "flex", flex_direction: "column",
            label { "Username/Email address" input { margin: "0.5rem", r#type: "text", value: "{user}", oninput: move |event| user.set(event.value()) } }
            label { "Password" input { margin: "0.5rem", r#type: "password", value: "{password}", oninput: move |event| password.set(event.value()) } }
            div {
                display: "flex", flex_direction: "row",
                    button { label { "Login" } }
                    button { label { "Signup" } }
            }
            Link { to: Route::Base {}, "Go back to home" }
        }
    }
}

#[component]
fn Base() -> Element {
    let is_logged_in = false;

    rsx! {
        NavBar { is_logged_in }
        Home { is_logged_in }
    }
}

#[component]
fn UserPage() -> Element {
    rsx! {
        div {
            h1 { "Welcome" }
        }
    }
}

#[component]
fn Home(is_logged_in: bool) -> Element {
    if is_logged_in {
        rsx! {
            UserPage { }
        }
    } else {
        rsx! {
            Link {
                to: Route::LoginPage { is_logged_in },
                "Login as user",
            }
        }
    }
}
