use salvo::basic_auth::BasicAuthValidator;
use salvo::Depot;

pub struct Validator;
impl BasicAuthValidator for Validator {
    async fn validate(&self, username: &str, password: &str, _depot: &mut Depot) -> bool {
	username == "root" && password == "pwd"
    }
}

