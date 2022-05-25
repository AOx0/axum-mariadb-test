use ddb::*;

// The struct is used to transform a POST into Rust types and also to insert it to the table
#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = Users)]
pub struct UsersI {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Queryable, Debug)]
pub struct UsersLogin {
    pub email: String,
    pub password: String,
}
