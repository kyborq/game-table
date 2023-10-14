use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_game_token() -> String {
    let rng = thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    random_string
}
