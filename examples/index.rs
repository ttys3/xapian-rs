use anyhow::Result;
use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Value};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use xapian::{Database, WritableDatabase};

fn main() -> Result<()> {
    // https://xapian.org/docs/sourcedoc/html/namespaceXapian_1_1Chert.html#ad328887e1b0e513dff7f50f62a645a40
    // https://xapian.org/docs/apidoc/html/classXapian_1_1WritableDatabase.html#acac2d0fa337933e0ed66c7dce2ce75d0
    // automatically determining the database backend to use
    let _ = std::fs::create_dir_all("./data");
    let mut db = WritableDatabase::new("./data/xapian-movie", xapian::constants::DB_CREATE_OR_OPEN, 0).expect("Error opening database");

    // let mut doc = xapian::Document::new().expect("Error creating document");

    let mut term_generator = xapian::TermGenerator::new().expect("Error creating term generator");
    // support CJK
    term_generator
        .set_flags(xapian::constants::TermGeneratorFlag::FLAG_CJK_NGRAM as i32, xapian::constants::TermGeneratorFlag::FLAG_DEFAULT as i32)
        .expect("Error setting flags");
    term_generator.set_stemmer(xapian::Stem::new("en").expect("Error creating stemmer"));

    // now we can index some data
    let file = File::open("./examples/movies.json")?;
    println!("read file ok");
    let reader = BufReader::new(file);

    let start_time = std::time::Instant::now();
    for line in reader.lines() {
        let mut line = line?;
        if !(line.starts_with("{")) {
            println!("skip invalid JSON: {}", line);
            continue;
        }
        if line.ends_with("},") {
            // strip the trailing comma
            line = (&line[..line.len() - 1]).parse().unwrap();
        }
        let mut movie: Movie = serde_json::from_str(&line)?;

        println!("{:?}", movie);

        let mut doc = xapian::Document::new().expect("Error creating document");
        doc.set_data(serde_json::to_string(&movie).unwrap().as_str());

        // add movie id term
        let idterm = format!("Q{}", movie.id);
        doc.add_boolean_term(&idterm);
        // add sortable_serialise int
        doc.add_int(0, movie.year);
        // add facets
        doc.add_string(1, movie.genres.join(",").as_str());
        doc.add_string(2, movie.year.to_string().as_str());

        term_generator.set_document(&mut doc);
        term_generator.index_text_with_prefix(&movie.title, "T");
        term_generator.index_text_with_prefix(&movie.overview, "O");
        // term_generator.index_text_with_prefix(&movie.year.to_string(),  "Y");

        db.replace_document(&idterm, &mut doc).expect("Error adding document");
    }

    println!(
        "doc count: {}, index doc took: {}ms",
        db.get_doccount().expect("Error getting doc count"),
        start_time.elapsed().as_millis()
    );
    // doc count: 31944, index doc took: 6091ms

    db.commit().expect("Error committing database");
    db.close().expect("Error closing database");
    println!("Hello test open and close WritableDatabase ok");
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    id: i64,
    title: String,
    overview: String,
    #[serde(rename(deserialize = "release_date"), deserialize_with = "deserialize_year")]
    year: i32,
    genres: Vec<String>,
}

fn deserialize_year<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: i64 = Deserialize::deserialize(deserializer)?;
    let dt = NaiveDateTime::from_timestamp_opt(value, 0);
    match dt {
        Some(dt) => {
            let year = DateTime::<Utc>::from_utc(dt, Utc).year();
            // println!("value={}, year: {}", value, year);
            Ok(year)
        }

        _ => Err(serde::de::Error::custom("Invalid year format")),
    }
}
