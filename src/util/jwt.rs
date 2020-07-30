use lazy_static::lazy_static;
use parking_lot::RwLock;
use yew::services::storage::{Area, StorageService};
use dotenv_codegen::dotenv;

const TOKEN_KEY: &str = dotenv!("TOKEN_KEY");

lazy_static! {
    pub static ref TOKEN: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Session).expect("storage was disabled by the user");
        if let Ok(token) = storage.restore(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

pub fn set_token(token: Option<String>) {
    let mut storage = StorageService::new(Area::Session).expect("storage was disabled by the user");
    if let Some(t) = token.clone() {
        storage.store(TOKEN_KEY, Ok(t));
    } else {
        storage.remove(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}
