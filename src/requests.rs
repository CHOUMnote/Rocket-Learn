pub mod methods {
    // Static Paths
    #[get("/아이엠a에요")]
    pub fn static_paths() -> &'static str {
        "Hello aaaaa!!!"
    }

    // Dynamic Paths + async fn
    pub use tokio::time::{sleep, Duration};
    #[get("/delay/<seconds>")]
    pub async fn delay(seconds: Option<u64>) -> String {
        match seconds {
            Some(t) => {
                sleep(Duration::from_secs(t)).await;
                format!("Waited for {} seconds", t)
            }
            _ => format!("your param is not i8 format"),
        }
    }

    // Multiple Segments
    use std::path::PathBuf;
    #[get("/page/<path..>")]
    pub fn get_path(path: PathBuf) -> String {
        format!("You are requested : {:?} ", path)
    }

    // Ignored Segments => '_' 와일드 카드 이용
    #[get("/foo/<_>/bar")]
    pub fn foo_bar() -> &'static str {
        "Foo _____ bar!"
    }

    #[get("/<_..>")]
    pub fn everything() -> &'static str {
        "Hey, you're here."
    }
}
