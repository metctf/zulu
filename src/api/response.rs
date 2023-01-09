use rocket::response::{self, Response,Responder};
use rocket::http::{ContentType,Status,};

#[derive(Debug)]
pub struct ApiResponse{
    pub json: String,
    pub status: Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse{
    fn respond_to(self, req: &'r rocket::Request<'_>) -> response::Result<'static> {
        Response::build_from(self.json.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
