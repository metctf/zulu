use rocket::form::Form;
use rocket::State;
use crate::structs::challenge::Challenge;
use super::super::structs::json::JsonResponse;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use rocket::Data;
use rocket::http::ContentType;
use crate::connections::database::{Pool, create_challenge, remove_challenge, modify_challenge, return_challenge, return_one_challenge, submit_challenge, return_author_challenge};
use super::super::connections::filesystem::create_challenge_file;
use crate::auth::jwt::JwtToken;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};
use crate::structs::challenge::SubmitChallenge;

#[post("/create_challenge", data = "<data>")]
pub async fn create_challenge_api(pool: &State<Pool>, content_type: &ContentType, data: Data<'_>) -> status::Custom<Json<JsonResponse>> {
    let mut options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
        MultipartFormDataField::text("name"),
        MultipartFormDataField::text("author"),
        MultipartFormDataField::text("flag"),
        MultipartFormDataField::text("points"),
        MultipartFormDataField::file("upload"),
        ]
    );

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();

    let name = multipart_form_data.texts.remove("name");
    let author = multipart_form_data.texts.remove("author");
    let flag = multipart_form_data.texts.remove("flag");
    let points = multipart_form_data.texts.remove("points");
    let upload = multipart_form_data.files.remove("upload");

    
    // god bless you if you read any of this below, I swear I could write this better but I don't
    // have time :(
    if let Some(mut text_fields) = name {
        let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.

        let name_text = text_field.text;

        if let Some(mut text_fields) = author {
            let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.

            let author_text = text_field.text;

            if let Some(mut text_fields) = flag {
                let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.

                let flag_text = text_field.text;

                if let Some(mut text_fields) = points {
                    let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.

                    let points_text = text_field.text;

                    if let Some(file_fields) = upload {
                        let file_field = &file_fields[0]; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

                        let upload_content_type = &file_field.content_type;
                        let upload_file_name = &file_field.file_name;
                        let upload_path = &file_field.path;

                        let challenge = Challenge {
                            id: String::from(""),
                            name: name_text.clone(),
                            author: author_text,
                            flag: flag_text,
                            points: points_text.parse::<u32>().unwrap()
                        };

                        let query = create_challenge(challenge, pool).await;

                        match query {
                            Ok(query) => {
                                let file_query = create_challenge_file(&name_text, upload_path).await;


                                match file_query {
                                    Ok(_) => {
                                        info!("Registered new challenge: {}", query);
                                        let resp = JsonResponse {
                                            id: String::from(query)
                                        };
                                        return status::Custom(Status::Ok, Json(resp))
                                    },
                                    Err(_) =>{
                                        error!("Couldn't create challenge");
                                        return status::Custom(Status::InternalServerError, Json(JsonResponse { id: String::from("") }))
                                    } 
                                }
                            },
                            Err(_) =>{
                                error!("Couldn't create challenge");
                                return status::Custom(Status::InternalServerError, Json(JsonResponse { id: String::from("") }))
                            } 
                        }
                    }
                    else {
                        return status::Custom(Status::InternalServerError, Json(JsonResponse { id: String::from("") }))
                    }
                }
                else {
                    return status::Custom(Status::InternalServerError, Json(JsonResponse { id: String::from("") }))
                }
            }
            else {
                return status::Custom(Status::InternalServerError, Json(JsonResponse { id: String::from("") }))
            }
        }
        else {
            return status::Custom(Status::InternalServerError, Json(JsonResponse { id: String::from("") }))
        }
    }
    else {
        return status::Custom(Status::InternalServerError, Json(JsonResponse { id: String::from("") }))
    }
}

#[delete("/delete_challenge/<id>")]
pub async fn delete_challenge_api(pool: &State<Pool>, token: JwtToken, id: String) -> status::Custom<Json<JsonResponse>> {
    let access = JwtToken::decode(token.body);

    match access {
        Ok(_) => {
            let query = remove_challenge(id, pool).await;
            match query{
                Ok(query) => {
                    let resp = JsonResponse {
                        id: String::from(query)
                    };
                    status::Custom(Status::Ok, Json(resp))
                },
                Err(_) => {
                    status::Custom(Status::NotFound, Json(JsonResponse { id: String::from("") }))
                }
            }
        },
        Err(_) => {
            status::Custom(Status::Forbidden, Json(JsonResponse { id: String::from("") }))
        }
    }
}


#[get("/get_challenge/<flag>")]
pub async fn display_flag(pool: &State<Pool>, flag: String) -> status::Custom<Json<Vec<Challenge>>> {
    /*
     * Return info as json
     */
    let query = return_challenge(pool, flag).await;

    match query {
        Ok(query) => {
            status::Custom(Status::Ok, Json(query))
        }
        Err(query) => {
            error!("{:?}",query);
            let challenge = Challenge {
                id: String::from(""),
                name: String::from(""),
                author: String::from(""),
                flag: String::from(""),
                points: 0,
            };
            let mut vec: Vec<Challenge> = vec![];
            vec.push(challenge);

            status::Custom(Status::NotFound, Json(vec))
        }

    }
}

#[get("/get_challenge",rank=2)]
pub async fn display_all_challenges(pool: &State<Pool>) -> status::Custom<Json<Vec<Challenge>>> {
    /*
     * Return info as json
     */
    let query = return_challenge(pool, format!("")).await;

    match query {
        Ok(query) => {
            status::Custom(Status::Ok, Json(query))
        }
        Err(query) => {
            error!("{:?}",query);
            let challenge = Challenge {
                id: String::from(""),
                name: String::from(""),
                author: String::from(""),
                flag: String::from(""),
                points: 0,
            };
            let mut vec: Vec<Challenge> = vec![];
            vec.push(challenge);

            status::Custom(Status::NotFound, Json(vec))
        }

    }
}
#[get("/get_single_challenge/<flag>")]
pub async fn single_flag(pool: &State<Pool>, flag: String) -> status::Custom<Json<Challenge>> {
    let query = return_one_challenge(pool, flag).await;

    match query {
        Ok(query) => status::Custom(Status::Ok, Json(query)),
        Err(_) => status::Custom(Status::NotFound, Json(Challenge::default()))
    }
}

#[post("/modify_challenge", data = "<challenge>")]
pub async fn modify_challenge_api(pool: &State<Pool>, token: JwtToken, challenge: Form<Challenge>) -> status::Custom<Json<JsonResponse>> {
    let access = JwtToken::decode(token.body);

    match access {
        Ok(_) => {
            let query = modify_challenge(&challenge, pool).await;
            match query {
                Ok(_query) => {
                    let resp = JsonResponse {
                        id: String::from(_query)
                    };
                    status::Custom(Status::Ok, Json(resp))
                }
                Err(_) => status::Custom(Status::NotFound, Json(JsonResponse { id: String::from("") }))
            }
        },
        Err(_) => status::Custom(Status::Forbidden, Json(JsonResponse { id: String::from("") }))

    }

}

#[post("/submit_challenge", data = "<challenge>")]
pub async fn submit_challenge_api(pool: &State<Pool>, challenge: Form<SubmitChallenge>) -> status::Custom<Json<bool>> {
    let query = submit_challenge(challenge, pool).await;
    match query {
        Ok(_) => status::Custom(Status::Ok, Json(true)),
        Err(_) => status::Custom(Status::NotFound, Json(false))
    }
}

#[get("/get_author_challenge")]
pub async fn author_challenge(pool: &State<Pool>, jwt: JwtToken) -> status::Custom<Json<Vec<Challenge>>> {
    log::info!("{}",&jwt.id);
    let query = return_author_challenge(pool, jwt.id).await;

    match query {
        Ok(query) => {
            status::Custom(Status::Ok, Json(query))
        }
        Err(query) => {
            error!("{:?}",query);
            let challenge = Challenge {
                id: String::from(""),
                name: String::from(""),
                author: String::from(""),
                flag: String::from(""),
                points: 0,
            };
            let mut vec: Vec<Challenge> = vec![];
            vec.push(challenge);

            status::Custom(Status::NotFound, Json(vec))
        }
    } 
}
