use anyhow::Result;
use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Value};
use std::fmt::{format, Debug};
use std::fs::File;
use std::io::{BufRead, BufReader};
use xapian::{Database, WritableDatabase};

fn main() -> Result<()> {
    let qs = "overview:gangsters year:1972..1975";
    // let qs = "id:14236";
    let offset = 0;
    let page_size = 40;

    // automatically determining the database backend to use
    let _ = std::fs::create_dir_all("./data");
    let mut db = Database::new_with_path("./data/xapian-movie", 0).expect("Error opening database");

    // let doc_count = db.get_doccount().expect("Error getting doc count");
    // println!("doc count: {}", doc_count);

    let start_time = std::time::Instant::now();

    let mut qp = xapian::QueryParser::new().expect("Error creating query parser");
    // set en stemm
    let mut stem = xapian::Stem::new("en").expect("Error creating stemmer");
    qp.set_stemmer(stem).expect("set_stemmer failed");
    qp.add_prefix("title", "T");
    qp.add_prefix("overview", "O");

    // let mut nrp_year = xapian::RangeProcessor::new(2, "year:", xapian::RangeProcessorFlags::RP_PREFIX).expect("Error creating number range processor");
    // qp.add_rangeprocessor(&mut nrp_year);

    let mut nrp_year = xapian::NumberRangeProcessor::new(0, "year:", xapian::constants::RangeProcessorFlags::RP_PREFIX).expect("Error creating number range processor");
    qp.add_number_rangeprocessor(&mut nrp_year);

    qp.add_boolean_prefix("id", "Q");

    let qp_flags = xapian::constants::QueryParserFeatureFlag::FLAG_DEFAULT as i32 | xapian::constants::QueryParserFeatureFlag::FLAG_CJK_NGRAM as i32;
    println!(
        "qp_flags: {:?} | {:?} = {:?}",
        xapian::constants::QueryParserFeatureFlag::FLAG_DEFAULT,
        xapian::constants::QueryParserFeatureFlag::FLAG_CJK_NGRAM,
        qp_flags
    );
    let mut query = qp.parse_query(qs, qp_flags).expect("Error parsing query");

    let mut enquire = db.new_enquire().expect("Error creating enquire");
    enquire.set_query(&mut query).expect("set_query failed");

    let mut genres_spy = xapian::ValueCountMatchSpy::new(1).expect("Error creating value count match spy");
    enquire.add_matchspy_value_count(&mut genres_spy).expect("Error adding matchspy");

    let mut mset = enquire.get_mset(offset, page_size).expect("Error getting mset");
    let matches_estimated = mset.get_matches_estimated().expect("Error getting matches estimated");
    println!("matches_estimated: {}", matches_estimated);

    let mut stem = xapian::Stem::new("en").expect("Error creating stemmer");
    let snippet_flags = xapian::constants::SnippetFlags::SNIPPET_BACKGROUND_MODEL as i32 | xapian::constants::SnippetFlags::SNIPPET_EXHAUSTIVE as i32;
    let mut it = mset.begin().unwrap();
    loop {
        if it.eq(&mut mset.end().unwrap()).unwrap() {
            break;
        }
        // undefined reference to `Xapian::MSet::get_doc_by_index(unsigned int)'
        let mut doc = it.get_document().expect("Error getting document");
        let data = doc.get_data().unwrap();
        // println!("raw doc data: {}", &data);
        let movie: Movie = from_str(&data).expect("Error parsing json");
        // if (hi_start.empty() && hi_end.empty() && text.size() <= length) {
        // 	// Too easy!
        // 	return text;
        //     }
        println!(
            "snippet: {:?}",
            mset.snippet(movie.overview.as_str(), 100, &mut stem, snippet_flags, "<b>", "</b>", "...")
        );
        println!("movie: {:?}", movie);
        it.next();
    }

    println!("genres_spy: {}", genres_spy.get_total());

    let mut spy = genres_spy.values_begin().unwrap();
    loop {
        if spy.eq(&mut genres_spy.values_end().unwrap()) {
            break;
        }
        let value = spy.get_termfreq_value();
        let count = spy.get_termfreq_freq();
        println!("{}: {}", value, count);
        spy.next();
    }

    println!("qs={}", &qs);
    println!("doc count: {}, index doc took: {}ms", matches_estimated, start_time.elapsed().as_millis());
    println!("search test ok");
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    id: i64,
    title: String,
    overview: String,
    year: i32,
    genres: Vec<String>,
}
