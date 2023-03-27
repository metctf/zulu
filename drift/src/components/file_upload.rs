use yew::prelude::*;
use crate::components::custom_button::CustomButton;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub flag: String,
}

#[function_component(FileUploadPoint)]
pub fn file_upload(props: &Props) -> Html {
    // Get input element
    // Get file from element
    // File reader read file
    // reader.onload(callback => send reader.result) 
    let id = props.flag.clone();
    let url = format!("http://127.0.0.1:8000/api/v1/upload_challenge/{}",id);
    html! {
        <div class={classes!("upload-div")}>
            <h1>{format!("Upload challenge for {}",&props.flag)}</h1>
            <form method="POST" enctype="multipart/form-data" action={url} class={classes!("upload-form")}>
                <label class={classes!("upload")}>
                    <i class="fa fa-cloud-upload"></i>
                    <p>{"Click to select the challenge file to upload"}</p>
                    <input type="file" accept="*" class={classes!("file_upload")} name={"Upload"} placeholder={"upload"} />
                </label>
                <CustomButton label="Submit" class="upload-button" />
            </form>
       </div>
    }
}
