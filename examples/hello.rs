use xapian::WritableDatabase;

fn main() {
    // https://xapian.org/docs/sourcedoc/html/namespaceXapian_1_1Chert.html#ad328887e1b0e513dff7f50f62a645a40
    let _ = std::fs::create_dir_all("./data");
    let mut db = WritableDatabase::new("./data/xapian-hello", xapian::DB_CREATE_OR_OPEN, xapian::CHERT)
        .expect("Error opening database");

    db.commit().expect("Error committing database");

    db.close().expect("Error closing database");
    println!("Hello test open and close WritableDatabase ok");
}
