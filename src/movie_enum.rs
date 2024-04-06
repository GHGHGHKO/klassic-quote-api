pub enum Movies {
    NewWorld,
    TheWarOfFlower,
}

pub fn str_to_movie(name: String) -> Result<&'static str, &'static str> {
    let movie = match name.as_str() {
        "new-world" => Movies::NewWorld,
        "the-war-of-flower" => Movies::TheWarOfFlower,
        _ => return Err("404 Not Found: Unknown movie title"),
    };

    Ok(match movie {
        Movies::NewWorld => "신세계",
        Movies::TheWarOfFlower => "타짜"
    })
}
