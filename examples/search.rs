use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use xapian_rusty::{Database, WritableDatabase};
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string, Value};
use chrono::{Datelike, DateTime, NaiveDateTime, Utc};
use anyhow::Result;

fn main()  -> Result<()> {
    let qs = "overview:gangsters year:1972..1975";
    // let qs = "id:14236";
    let offset = 0;
    let page_size = 40;

    // automatically determining the database backend to use
    let mut db = Database::new_with_path("./xapian-movie", 0)
        .expect("Error opening database");

    // let doc_count = db.get_doccount().expect("Error getting doc count");
    // println!("doc count: {}", doc_count);

    let start_time = std::time::Instant::now();

    let mut qp = xapian_rusty::QueryParser::new().expect("Error creating query parser");
    // set en stemm
    qp.set_stemmer(xapian_rusty::Stem::new("en").expect("Error creating stemmer")).expect("set_stemmer failed");
    qp.add_prefix("title", "T");
    qp.add_prefix("overview", "O");

    // let mut nrp_year = xapian_rusty::RangeProcessor::new(2, "year:", xapian_rusty::RangeProcessorFlags::RP_PREFIX).expect("Error creating number range processor");
    // qp.add_rangeprocessor(&mut nrp_year);

    let mut nrp_year = xapian_rusty::NumberRangeProcessor::new(0, "year:", xapian_rusty::RangeProcessorFlags::RP_PREFIX).expect("Error creating number range processor");
    qp.add_number_rangeprocessor(&mut nrp_year);

    qp.add_boolean_prefix("id", "Q");


    let qp_flags = xapian_rusty::QueryParserFeatureFlag::FLAG_DEFAULT as i32 | xapian_rusty::QueryParserFeatureFlag::FLAG_CJK_NGRAM as i32;
    println!("qp_flags: {:?} | {:?} = {:?}", xapian_rusty::QueryParserFeatureFlag::FLAG_DEFAULT, xapian_rusty::QueryParserFeatureFlag::FLAG_CJK_NGRAM, qp_flags);
    let mut query = qp.parse_query(qs, qp_flags)
        .expect("Error parsing query");

    let mut enquire = db.new_enquire().expect("Error creating enquire");
    enquire.set_query(&mut query).expect("set_query failed");

    let mut genres_spy = xapian_rusty::ValueCountMatchSpy::new(1).expect("Error creating value count match spy");
    enquire.add_matchspy_value_count(&mut genres_spy).expect("Error adding matchspy");

    let mut mset = enquire.get_mset(offset, page_size).expect("Error getting mset");
    let matches_estimated = mset.get_matches_estimated().expect("Error getting matches estimated");
    println!("matches_estimated: {}", matches_estimated);

    let mut it = mset.begin().unwrap();
    loop {
        if it.eq(&mut mset.end().unwrap()).unwrap() {
            break;
        }
        // undefined reference to `Xapian::MSet::get_doc_by_index(unsigned int)'
        let mut doc = it.get_document().expect("Error getting document");
        let data = doc.get_data();
        // println!("raw doc data: {}", &data);
        let movie: Movie = from_str(&data).expect("Error parsing json");
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
