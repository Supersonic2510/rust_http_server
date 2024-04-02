#[allow(dead_code)]
pub enum HTTPStatus {
    OK,
    BadRequest,
    Unauthorized,
    NotFound,
    Created,
    InternalServerError,
    // ... You can add more status codes here
}

impl HTTPStatus {
    pub fn code(&self) -> u16 {
        match *self {
            HTTPStatus::OK => 200,
            HTTPStatus::BadRequest => 400,
            HTTPStatus::Unauthorized => 401,
            HTTPStatus::NotFound => 404,
            HTTPStatus::Created => 201,
            HTTPStatus::InternalServerError => 500,
            // ... Remember to handle every case
        }
    }

    pub fn message(&self) -> &'static str {
        match *self {
            HTTPStatus::OK => "OK",
            HTTPStatus::BadRequest => "Bad Request",
            HTTPStatus::Unauthorized => "Unauthorized",
            HTTPStatus::NotFound => "Not Found",
            HTTPStatus::Created => "Created",
            HTTPStatus::InternalServerError => "Internal Server Error",
            // ... And here as well
        }
    }
}