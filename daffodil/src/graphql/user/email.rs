use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
#[graphql(name = "UserSignUpByEmailRequest")]
pub struct SignUp {
    pub real_name: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub timezone: String,
    pub lang: String,
}

#[derive(GraphQLInputObject)]
#[graphql(name = "UserSignInByEmailRequest")]
pub struct SignIn {
    pub user: String,
    pub password: String,
}
