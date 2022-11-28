/*!
Bookery is a client library to query a tantivy index customized for books.
 */

use reqwest::{blocking, StatusCode, Url};
use serde::Deserialize;

const BOOKIN_API_BASE_URL: &str = "http://localhost:3000/api/";

pub fn build_client() -> blocking::Client {
    blocking::Client::builder().build().unwrap()
}

fn build_full_url(term: &str) -> Url {
    let mut url = Url::parse(BOOKIN_API_BASE_URL).unwrap();
    url.query_pairs_mut().append_pair("q", term);
    println!("{}", url);
    url
}

pub fn get_hits(client: &blocking::Client, term: &str) -> Option<Hits> {
    let url = build_full_url(term);
    let res = client.get(url).send().unwrap();
    if res.status() != StatusCode::OK {
        eprintln!(
            "Get {} from Bookin API when querying {}",
            res.status(),
            term
        );
        return None;
    }
    serde_json::from_reader(res).unwrap()
}

#[derive(Debug, Deserialize)]
pub struct Hits {
    pub hits: Vec<Hit>,
}

#[derive(Debug, Deserialize)]
pub struct Hit {
    pub doc: Doc,
}

#[derive(Debug, Deserialize)]
pub struct Doc {
    pub chapter: Vec<String>,
    pub sentence: Vec<String>,
    pub title: Vec<String>,
}
