use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use xapian_rusty::{Database, WritableDatabase};
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string, Value};
use chrono::{Datelike, DateTime, NaiveDateTime, Utc};
use anyhow::Result;

fn main()  -> Result<()> {
    let qs = "description:机器人 year:1980..1985 id:78";
    let offset = 0;
    let page_size = 5;

    // automatically determining the database backend to use
    let mut db = Database::new_with_path("./xapian-movie", 0)
        .expect("Error opening database");

    // let doc_count = db.get_doccount().expect("Error getting doc count");
    // println!("doc count: {}", doc_count);

    let mut qp = xapian_rusty::QueryParser::new().expect("Error creating query parser");
    // set en stemm
    qp.set_stemmer(xapian_rusty::Stem::new("en").expect("Error creating stemmer")).expect("set_stemmer failed");
    qp.add_prefix("title", "T");
    qp.add_prefix("overview", "O");

    qp.add_boolean_prefix("id", "Q");

    let genres = xapian_rusty::ValueCountMatchSpy::new(1);

    let mut query = qp.parse_query(qs, xapian_rusty::FLAG_CJK_NGRAM).expect("Error parsing query");

    let mut enquire = db.new_enquire().expect("Error creating enquire");
    enquire.set_query(&mut query).expect("set_query failed");

    let mut mset = enquire.get_mset(offset, page_size).expect("Error getting mset");
    let matches_estimated = mset.get_matches_estimated().expect("Error getting matches estimated");
    println!("matches_estimated: {}", matches_estimated);

    let mut it = mset.iterator().unwrap();
    loop {
        if !it.is_next().unwrap() {
            break;
        }
        // undefined reference to `Xapian::MSet::get_doc_by_index(unsigned int)'
        let data = it.get_document_data().expect("Error getting document");
        let movie: Movie = from_str(&data).expect("Error parsing json");
        println!("movie: {:?}", movie);
        it.next();
    }

    println!("search test ok");
    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    id: i64,
    title: String,
    overview: String,
    #[serde(rename = "release_date", deserialize_with = "deserialize_year")]
    year: i32,
    genres: Vec<String>,
}


fn deserialize_year<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let value: i64 = Deserialize::deserialize(deserializer)?;
    let dt = NaiveDateTime::from_timestamp_opt(value, 0);
    match dt     {
        Some(dt) => {
            let year = DateTime::<Utc>::from_utc(dt, Utc).year();
            Ok(year)
        },

        _ => {
            Err(serde::de::Error::custom("Invalid year format"))
        }
    }
}