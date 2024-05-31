use rocket::request::FromParam;
use uuid::Uuid;

pub struct MyUuid(pub Uuid);

impl<'r> FromParam<'r> for MyUuid {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param).map(MyUuid).map_err(|_| "invalid UUID")
    }
}
