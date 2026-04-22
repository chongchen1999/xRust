pub fn greet(name: &str, lang: &str) -> String {
    match lang {
        "cn" => format!("你好, {}!", name),
        _    => format!("Hello, {}!", name),
    }
}