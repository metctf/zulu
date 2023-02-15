use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Serialize,Deserialize};

use crate::MainRoute;
use crate::router::SettingsRoute;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(Tab::Unauthorized)]
    pub tab: Tab,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Tab {
    Authorized,
    Unauthorized,
}
#[function_component(NavBar)]
pub fn new_bar(props: &Props) -> Html{
    let navigator = use_navigator().unwrap();
    let logout = Callback::from( move |_| {
        LocalStorage::clear();
        navigator.push(&MainRoute::Home);
    });
 
    match &props.tab {
        Tab::Authorized => {
           html! {
                <div class={classes!{"topnav"}}>
                    <Link<MainRoute> to={MainRoute::Home}>{"Home"}</Link<MainRoute>>
                    <div class={classes!{"dropdown"}}>
                        <a style={"cursor: pointer;"}>{"Settings"}</a>
                        <div class={classes!("dropdown-content")}>
                            <Link<SettingsRoute> to={SettingsRoute::Modify}>{"Modify"}</Link<SettingsRoute>>
                            <a onclick={logout} style={"cursor:pointer;"}>{"Log Out"}</a>
                        </div>
                    </div>
                    <Link<MainRoute> classes={classes!("right")} to={MainRoute::SubmitFlag}>{"Submit Flag"}</Link<MainRoute>>
                    <Link<MainRoute> classes={classes!("right")} to={MainRoute::CreateFlag}>{"Create Flag"}</Link<MainRoute>>
                </div>
            }
        },
        Tab::Unauthorized => {
            html! {
                <div class={classes!{"topnav"}}>
                    <Link<MainRoute> to={MainRoute::Home}>{"Home"}</Link<MainRoute>>
                    <Link<MainRoute> classes={classes!("right")} to={MainRoute::Register}>{"Register"}</Link<MainRoute>>
                    <Link<MainRoute> classes={classes!("right")} to={MainRoute::Login}>{"Login"}</Link<MainRoute>>
                </div>
            }
        },
    }
}
