use rocket::form::Form;
use rocket::State;
use super::super::structs::flag::Flag;
use super::super::connections::database::{Pool, create_flag};

#[post("/create_flag", data = "<flag>")]
pub async fn create_flag_api(pool: &State<Pool>, flag: Form<Flag>){
    let query = create_flag(&flag, pool).await;

    match query {
        Ok(_query) => {
            info!("Registered new flag: {}", &flag.flagid); 
        },
        Err(_) =>{
            error!("Couldn't create flag: {}", &flag.flagid);
        } 
    }
}
