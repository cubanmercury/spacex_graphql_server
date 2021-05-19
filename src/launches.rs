#[derive(Deserialize, Debug)]
struct Fairings {
    reused: Option<bool>,
    recovery_attempt: Option<bool>,
    recovered: Option<bool>,
    // ships: [String; 5]
}
#[derive(Deserialize, Debug)]
struct Patches {
    small: Option<String>,
    large: Option<String>
}
#[derive(Deserialize, Debug)]
struct Reddit {
    campaign: Option<String>,
    launch: Option<String>,
    media: Option<String>
}
#[derive(Deserialize, Debug)]
struct Flickr {
    // small: [String; 2],
    // original: [String; 2]
}
#[derive(Deserialize, Debug)]
struct Links {
    patch: Option<Patches>,
    reddit: Option<Reddit>,
    flickr: Option<Flickr>,
    presskit: Option<String>,
    webcast: Option<String>,
    youtube_id: Option<String>,
    article: Option<String>,
    wikipedia: Option<String>
}
#[derive(Deserialize, Debug)]
pub struct Launches {
    fairings: Option<Fairings>,
    links: Option<Links>,
}