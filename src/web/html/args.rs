#[derive(Debug)]
pub struct GlobalArgs {
    pub domain: String,
    pub runtime_id: u64,
    pub invite_link: String,
}

#[derive(Debug, Clone)]
pub struct PageArgs<'t, C: PageConfig> {
    pub config: C,
    pub title: &'t str,
    pub description: &'t str,
    pub login: Option<&'t UserDisp>,
}

pub trait PageConfig: serde::Serialize + Sized {
    fn page_type() -> &'static str;
}

impl PageConfig for () {
    fn page_type() -> &'static str {
        "unit"
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserDisp {
    pub id: u64,
    pub username: String,
    pub discrim: String,
    pub avatar: Option<String>,
}
