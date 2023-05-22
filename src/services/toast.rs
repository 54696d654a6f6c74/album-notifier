use notify_rust::Notification;

pub fn new_album(artist: &str, album: &str) {
    match Notification::new()
        .summary("New album detected!")
        .body(&("artist: ".to_owned() + artist + "\nAlbum: " + album))
        .show()
    {
        Ok(_) => (),
        Err(why) => panic!(
            "failed to create toast for new album by {} called {} because: {}",
            artist, album, why
        ),
    };
}
