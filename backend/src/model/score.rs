pub struct Score {
    name: String,
    score: i32,
    created: String,
    updated: String,
}

// #[derive(Deserialize)]
pub struct SaveScore {
    name: String,
    score: i32,
}
