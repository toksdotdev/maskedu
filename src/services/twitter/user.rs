use chrono::DateTime;
use chrono::Utc;
use egg_mode::{
    tweet::Tweet,
    user::{TwitterUser as TwitterPayloadUser, UserEntities},
    Token,
};

pub struct TwitterUser {
    pub token: Option<Token>,
    pub contributors_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub description: Option<String>,
    pub entities: UserEntities,
    pub favourites_count: i32,
    pub follow_request_sent: Option<bool>,
    pub followers_count: i32,
    pub friends_count: i32,
    pub geo_enabled: bool,
    pub id: u64,
    pub is_translator: bool,
    pub lang: Option<String>,
    pub listed_count: i32,
    pub location: Option<String>,
    pub name: String,
    pub profile_background_color: String,
    pub profile_background_image_url: Option<String>,
    pub profile_background_image_url_https: Option<String>,
    pub profile_background_tile: Option<bool>,
    pub profile_banner_url: Option<String>,
    pub profile_image_url: String,
    pub profile_image_url_https: String,
    pub profile_link_color: String,
    pub profile_sidebar_border_color: String,
    pub profile_sidebar_fill_color: String,
    pub profile_text_color: String,
    pub profile_use_background_image: bool,
    pub protected: bool,
    pub screen_name: String,
    pub show_all_inline_media: Option<bool>,
    pub status: Option<Box<Tweet>>,
    pub statuses_count: i32,
    pub time_zone: Option<String>,
    pub url: Option<String>,
    pub utc_offset: Option<i32>,
    pub verified: bool,
    pub withheld_in_countries: Option<Vec<String>>,
    pub withheld_scope: Option<String>,
}

impl TwitterUser {
    pub fn with_token(token: Token, user: TwitterPayloadUser) -> Self {
        let mut user = TwitterUser::from(user);
        user.token = Some(token);
        user
    }
}

impl From<TwitterPayloadUser> for TwitterUser {
    fn from(user: TwitterPayloadUser) -> Self {
        Self {
            token: None,
            contributors_enabled: user.contributors_enabled,
            created_at: user.created_at,
            default_profile: user.default_profile,
            default_profile_image: user.default_profile_image,
            description: user.description,
            entities: user.entities,
            favourites_count: user.favourites_count,
            follow_request_sent: user.follow_request_sent,
            followers_count: user.followers_count,
            friends_count: user.friends_count,
            geo_enabled: user.geo_enabled,
            id: user.id,
            is_translator: user.is_translator,
            lang: user.lang,
            listed_count: user.listed_count,
            location: user.location,
            name: user.name,
            profile_background_color: user.profile_background_color,
            profile_background_image_url: user.profile_background_image_url,
            profile_background_image_url_https: user.profile_background_image_url_https,
            profile_background_tile: user.profile_background_tile,
            profile_banner_url: user.profile_banner_url,
            profile_image_url: user.profile_image_url,
            profile_image_url_https: user.profile_image_url_https,
            profile_link_color: user.profile_link_color,
            profile_sidebar_border_color: user.profile_sidebar_border_color,
            profile_sidebar_fill_color: user.profile_sidebar_fill_color,
            profile_text_color: user.profile_text_color,
            profile_use_background_image: user.profile_use_background_image,
            protected: user.protected,
            screen_name: user.screen_name,
            show_all_inline_media: user.show_all_inline_media,
            status: user.status,
            statuses_count: user.statuses_count,
            time_zone: user.time_zone,
            url: user.url,
            utc_offset: user.utc_offset,
            verified: user.verified,
            withheld_in_countries: user.withheld_in_countries,
            withheld_scope: user.withheld_scope,
        }
    }
}
