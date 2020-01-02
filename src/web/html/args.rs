#[derive(Debug)]
pub struct Args<'t, T> {
    pub global: &'t GlobalArgs,
    pub page: PageArgs<'t>,
    pub local: T,
}

#[derive(Debug)]
pub struct GlobalArgs {
    pub domain: String,
    pub runtime_id: u64,
    pub invite_link: String,
}

#[derive(Debug, Clone, Copy)]
pub struct PageArgs<'t> {
    pub title: &'t str,
    pub description: &'t str,
    pub login: Option<&'t UserDisp>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserDisp {
    pub id: u64,
    pub username: String,
    pub discrim: String,
    pub avatar: Option<String>,
}
