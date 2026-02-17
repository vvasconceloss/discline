#[derive(Clone, Debug)]
pub struct UserId(pub u64);

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
}
