use std::sync::{Arc, Mutex};

static CACHE: Mutex<Vec<Arc<str>>> = Mutex::new(Vec::new());

fn get_level(input: &str) -> Arc<str> {
    let mut cache = CACHE.lock().unwrap();
    if let Some(out) = cache.iter().find(|item| input.eq_ignore_ascii_case(item)) {
        out.clone()
    } else {
        let out: Arc<str> = Arc::from(input);
        cache.push(out.clone());
        out
    }
}

pub fn get_levels() -> Vec<Arc<str>> {
    CACHE.lock().unwrap().clone()
}
