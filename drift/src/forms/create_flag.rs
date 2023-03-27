use yew::prelude::*;
use yew_router::prelude::*;
use std::ops::Deref;

use crate::components::text_input::TextInput;
use crate::components::custom_button::CustomButton;
use crate::components::number_input::NumberInput;
use crate::router::MainRoute;

#[derive(Default, Clone)]
pub struct FlagData {
    pub name: String,
    pub flag: String,
    pub points: u32,
}

#[function_component(CreateFlag)]
pub fn create_flag() -> Html {

    let url = format!("http://127.0.0.1:8000/api/v1/create_challenge");
    html! {
        <div class={classes!("form-div")}>
            <h1>{"Create a Flag"}</h1>
            <form method="POST" enctype="multipart/form-data" action={url} >
                <input type="text" id="name" name="name" class="form-input" placeholder="challenge" />
                <br />
                <input type="text" id="author" name="author" class="form-input" placeholder="author" />
                <br />
                <input type="text" id="flag" name="flag" class="form-input" placeholder="flag" />
                <br />
                <input type="number" id="points" name="points" class="form-input" placeholder="points" />
                <br />
                <label class={classes!("upload")}>
                    <i class="fa fa-cloud-upload"></i>
                    <p>{"Click to select the challenge file to upload"}</p>
                    <input type="file" accept="*" class={classes!("file_upload")} name="upload" id="upload" placeholder="upload" />
                </label>
                <br />
                <CustomButton label="Submit" class="button" />
            </form>
        </div>
    }
}
