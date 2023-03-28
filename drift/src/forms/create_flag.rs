use std::{ops::Deref, borrow::BorrowMut};

use web_sys::HtmlInputElement;
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

use crate::components::custom_button::CustomButton;

#[derive(Default, Clone)]
pub struct FlagData {
    pub name: String,
    pub flag: String,
    pub points: u32,
}

#[function_component(CreateFlag)]
pub fn create_flag() -> Html {

    // Code for reading file element
    let preview = Callback::from(move |e: Event|{
        let file: HtmlInputElement = e.target_unchecked_into();
        let files = file.files();
        let mut list_of_files = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(web_sys::File::from);
            list_of_files.extend(files)
        }

        let reader = web_sys::FileReader::new().unwrap();
        reader.read_as_data_url(&*list_of_files[0]).expect("Cannot read file");

        let reader_option = Some(reader.clone());
        
        let onloadend = Closure::wrap(Box::new(move |_: Event| {
            let reader_option = reader_option.clone();
            let reader = reader_option.unwrap();
            let data_url = reader.result().unwrap();
            let data_url = data_url.as_string().unwrap();
            gloo::console::log!(&data_url);
        }) as Box<dyn FnMut(_)>);
        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));

        onloadend.forget();
    });

    let url = format!("http://127.0.0.1:8000/api/v1/create_challenge");
    html! {
        <>
        <div class={classes!("form-div")}>
            <h1>{"Create Challenge"}</h1>
            <form method="POST" enctype="multipart/form-data" action={url} >
                <input type="text" id="name" name="name" class="form-input" placeholder="challenge" />
                <br />
                <input type="text" id="author" name="author" class="form-input" placeholder="author" />
                <br />
                <input type="text" id="flag" name="flag" class="form-input" placeholder="flag" />
                <br />
                <select name="points" id="points" class="form-input">
                    <option selected=true>{"Challenge Score"}</option>
                    <option value=10>{"10"}</option>
                    <option value=20>{"20"}</option>
                    <option value=30>{"30"}</option>
                    <option value=40>{"40"}</option>
                    <option value=50>{"50"}</option>
                </select>
                <br />
                <div class={classes!("upload-point")}>
                <label class={classes!("upload")}>
                    <i class="fa fa-cloud-upload"></i>
                    <p>{"Click to select the challenge file to upload"}</p>
                    <input type="file" accept="*" class={classes!("file_upload")} name="upload" id="upload" placeholder="upload" onchange={preview}/>
                </label>
                </div>
                <CustomButton label="Submit" class="button" />
            </form>
        </div>
        </>
    }
}
