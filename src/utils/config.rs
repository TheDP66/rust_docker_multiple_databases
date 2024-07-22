fn get_env_var(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| panic!("{} must be set", var_name))
}

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub client_origin: String,
    pub database_url_codein: String,
    pub database_url_iekimtie: String,
    pub database_url_wkawan: String,
}

impl Config {
    pub fn init() -> Config {
        let port = get_env_var("PORT");
        let client_origin = get_env_var("CLIENT_ORIGIN");
        let database_url_codein = get_env_var("DATABASE_URL_CODEIN");
        let database_url_iekimtie = get_env_var("DATABASE_URL_IEKIMTIE");
        let database_url_wkawan = get_env_var("DATABASE_URL_WKAWAN");

        Config {
            port: port.parse::<u16>().unwrap(),
            client_origin,
            database_url_codein: database_url_codein,
            database_url_iekimtie: database_url_iekimtie,
            database_url_wkawan: database_url_wkawan,
        }
    }
}
