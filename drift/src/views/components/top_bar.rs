use yew::prelude::*;
use yew_router::prelude::Link;
use serde::{Serialize,Deserialize};

use crate::MainRoute;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(Tab::Unauthorized)]
    pub tab: Tab,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Tab {
    Authorized,
    Unauthorized,
    Settings
}
#[function_component(NavBar)]
pub fn new_bar(props: &Props) -> Html{
    match &props.tab {
        Tab::Authorized => {
            html! {
                <div class={classes!{"topnav"}}>
                    <Link<MainRoute> to={MainRoute::Home}>{"Home"}</Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::CreateFlag}>{"Create Flag"}</Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::SubmitFlag}>{"Submit Flag"}</Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::SettingsRoot}>{"Settings"}</Link<MainRoute>>
                </div>
            }
        },
        Tab::Unauthorized => {
            html! {
                <div class={classes!{"topnav"}}>
                    <Link<MainRoute> to={MainRoute::Home}>{"Home"}</Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::Register}>{"Register"}</Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::Login}>{"Login"}</Link<MainRoute>>
                </div>
            }
        },
        Tab::Settings => {
            html! {
                <div class={classes!{"topnav"}}>
                    <Link<MainRoute> to={MainRoute::Home}>{"Home"}</Link<MainRoute>>
                </div>
            }
        }
    }
}
