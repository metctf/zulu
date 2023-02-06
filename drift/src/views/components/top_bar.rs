use yew::prelude::*;
use yew_router::prelude::Link;
use crate::MainRoute;

pub struct TopBarComponent;

impl Component for TopBarComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
                <div class={classes!{"topnav"}}>
                    <Link<MainRoute> to={MainRoute::Home}>{"Home"}</Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::Register}>{"Register"}</Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::Login}>{"Login"}</Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::CreateFlag}>{"Create Flag"}</Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::SubmitFlag}>{"Submit Flag"}</Link<MainRoute>>
                </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
}
