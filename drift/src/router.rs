use gloo_storage::errors::StorageError;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

use crate::components::top_bar::{NavBar, Tab};
use crate::components::footer::Footer;
use crate::pages::register::RegisterComponent;
use crate::pages::login::LoginComponent;
use crate::forms::create_flag::{FlagData,CreateFlag};
use crate::pages::home::Home;
use crate::pages::challenge_template::{DisplayChallenge, DisplayAuthorChallenge, ChallengeTemplate};
use crate::components::file_upload::FileUploadPoint;
use crate::pages::notfound::NotFound;
use crate::settings::modify::ModifyComponent;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum MainRoute{
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
    #[at("/display_challenge/:id")]
    DisplayChallenge {id:String},
    #[at("/display_challenge/")]
    DisplayAllChallenges,
    #[at("/challenge/:id")]
    Challenge {id:String},
    #[at("/create_flag")]
    CreateFlag,
    #[at("/author_challenges")]
    AuthorChallenges,
    #[at("/upload_challenge/:id")]
    UploadChallenge {id:String},
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum SettingsRoute{
    #[at("/settings")]
    Settings,
    #[at("/settings/modify")]
    Modify,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}
pub fn switch_main(route: MainRoute) -> Html {
    
    let jwt: Result<String,StorageError>= LocalStorage::get("_AuthToken"); 
    let auth: Tab;

    match jwt {
        Ok(_) => {
                auth = Tab::Authorized;
            },
        Err(_) => {
            auth = Tab::Unauthorized;
        }
    }

    match route {
        MainRoute::Login => {
            html! {
                <LoginComponent />
            }
        },
        MainRoute::Register => {
           html! {
                <RegisterComponent />
           } 
        },
        MainRoute::CreateFlag => {
            let custom_form_submit = Callback::from(|data: FlagData| {
                log!("challenge is", &data.name.to_string());

                wasm_bindgen_futures::spawn_local( async move {
                    let url = format!("http://127.0.0.1:8000/api/v1/create_challenge");
                    let form = (
                        ("id", "".to_string()),
                        ("name",data.name),
                        // Just used to not get an no author error on zulu when its never used
                        ("author","placeholder"),
                        ("flag", data.flag),
                        ("points", data.points)
                    );
                    let client = reqwest::Client::new();
                    let jwt: String = LocalStorage::get("_AuthToken").unwrap();

                    client.post(&url)
                        .header("auth", jwt)
                        .form(&form)
                        .send()
                        .await
                        .unwrap(); //Getting an error here
                });
            });
            html! {
                <>
                    <NavBar tab={Tab::Authorized}/>
                    <CreateFlag />
                    <Footer />
                </>
            }
        },
       MainRoute::Home => {
            
            html! {
            <>
                <NavBar tab={auth}/>
                <Home />
                <Footer />
            </>
            }
        },
        MainRoute::DisplayChallenge { id } => {
            html! {
            <>
                <NavBar tab={auth}/>
                <DisplayChallenge flag={id}/>
                <Footer />
            </>
            }
        },
        MainRoute::DisplayAllChallenges => {
            html! {
            <>
                <NavBar tab={auth}/>
                <DisplayChallenge flag={""}/>
                <Footer />
            </>
            }
        },
        MainRoute::AuthorChallenges => {
            html! {
            <>
                <NavBar tab={auth}/>
                <DisplayAuthorChallenge />
                <Footer />
            </>
            }
        }
        MainRoute::Challenge { id } => {
            html! {
                <>
                    <NavBar tab={auth}/>
                    <ChallengeTemplate flag={id} />
                    <Footer />
                </>
            }
        }
        MainRoute::UploadChallenge { id } => {
            html! {
                <>
                    <NavBar tab={auth}/>
                    <FileUploadPoint flag={id} />
                    <Footer />
                </>
            }
        }
        MainRoute::SettingsRoot | MainRoute::Settings => html! { <Switch<SettingsRoute> render={switch_settings} /> },
        MainRoute::NotFound => {
            html! {
            <>
                <NavBar tab={auth}/>
                <NotFound />
                <Footer />
            </>
            }
        },
    }
}

pub fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Settings =>  html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>},
        SettingsRoute::Modify => {
            html!(<ModifyComponent />)
       },
        SettingsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>}
    }
}

