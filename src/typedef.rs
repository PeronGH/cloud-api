pub mod genshin {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct QueryWithCookie {
        pub cookie: String,
    }
}
