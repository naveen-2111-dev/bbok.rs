pub mod lib {
    pub mod connect;
    pub mod middlewares;
}

pub mod models {
    pub mod enums;
    pub mod models;
}

pub mod schema {
    pub mod schema;
}

pub mod routes{
    pub mod signup;
}

pub use lib::*;
pub use models::*;
