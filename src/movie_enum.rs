pub fn str_to_movie(name: String) -> Result<&'static str, &'static str> {
    Ok(match name.as_str() {
        "new-world" => "신세계",
        "the-war-of-flower" => "타짜",
        "nameless-gangster" => "범죄와의 전쟁",
        "wish" => "바람",
        _ => return Err("404 Not Found: Unknown movie title"),
    })
}
