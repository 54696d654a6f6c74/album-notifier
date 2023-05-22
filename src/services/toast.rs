use notify_rust::Notification;

pub fn new_album(band: &str, album: &str) {
    match Notification::new()
        .summary("New album detected!")
        .body(&("Band: ".to_owned() + band + "\nAlbum: " + album))
        .show()
    {
        Ok(_) => (),
        Err(why) => panic!(
            "failed to create toast for new album by {} called {} because: {}",
            band, album, why
        ),
    };
}
