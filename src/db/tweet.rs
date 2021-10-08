use serde::Deserialize;
use chrono::{Datelike,Utc};


#[derive(Deserialize, Debug)]
pub struct Tweet{
    #[serde(default = "default_str_content")]
    pub username:  String,
	#[serde(default = "default_str_content")]
    pub content: String,
    #[serde(default = "default_votes_count")]
    pub upvotes: i32,
    #[serde(default = "default_votes_count")]
    pub downvotes: i32,
    #[serde(default = "default_fecha")]
    pub fecha: String,
    #[serde(default = "default_str_content")]
    pub avatar: String, 
    #[serde(default = "default_pk")]
    pub pk: i32, 
    pub hashtags: Option<Vec<String>>
}




#[derive(Debug, PartialEq, Eq)]
pub struct Row {
    pub added_id: i32
}

fn default_pk() -> i32 {
    0
}

fn default_str_content() -> String {
    "".to_owned()
}

fn default_votes_count() -> i32 {
    0
}

fn default_fecha() -> String {
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    let mut date: String = "".to_owned();
    date.push_str(&*now.month().to_string());
    date.push_str("/");
    date.push_str(&*now.day().to_string());
    date.push_str("/");
    date.push_str(&*year.to_string());

    date.to_owned()
}







