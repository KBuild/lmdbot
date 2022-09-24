use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Deserialize, Debug)]
pub struct CommonDataField<T> {
    data: T,
}

#[derive(Serialize, Debug)]
pub struct RetweetRequest {
    tweet_id: String,
}

#[derive(Deserialize, Debug)]
pub struct RetweetResult {
    retweeted: bool,
}

lazy_static! {
    static ref TWEET_ID_REGEX: Regex = Regex::new(r"twitter\.com/.*/status(?:es)?/([^/\?]+)").unwrap();
}

enum EndpointType {
    Retweet,
}

fn make_endpoint(endpoint_type: EndpointType, user_id: &String) -> String {
    match endpoint_type {
        Retweet => format!("https://api.twitter.com/2/users/{}/retweets", user_id),
    }
}

pub struct Twitter {
    access_token: String,
    refresh_token: String,
    user_id: String,
}

impl Twitter {
    pub fn new(access_token: String, refresh_token: String, user_id: String) -> Self {
        Twitter {
            access_token,
            refresh_token,
            user_id,
        }
    }

    // TODO: Get Token with Use Refresh Token

    pub async fn retweet(&self, retweet_id: String) -> Result<CommonDataField<RetweetResult>, reqwest::Error> {
        let retweet_id: String = if retweet_id.contains("twitter.com") {
            TWEET_ID_REGEX.captures(retweet_id.as_str()).unwrap().get(1).map_or("", |m| m.as_str()).to_owned()
        } else { retweet_id };

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.access_token).parse().unwrap(),
        );
        let client = reqwest::Client::new()
            .post(make_endpoint(EndpointType::Retweet, &self.user_id))
            .json(&RetweetRequest { tweet_id: retweet_id })
            .headers(headers);
        client.send().await?.json().await
    }
}
