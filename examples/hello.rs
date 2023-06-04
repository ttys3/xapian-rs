use xapian::WritableDatabase;

fn main() {
    // https://xapian.org/docs/sourcedoc/html/namespaceXapian_1_1Chert.html#ad328887e1b0e513dff7f50f62a645a40
    let _ = std::fs::create_dir_all("./data");
    // Honey backend doesn't support updating existing databases
    let mut db = WritableDatabase::new("./data/xapian-hello", xapian::DB_CREATE_OR_OPEN, xapian::DB_BACKEND_HONEY);

    match db {
        Ok(mut db) => {
            println!("open WritableDatabase ok");
            match db.commit() {
                Ok(_) => {}
                Err(e) => {
                    println!("commit error: {}", e);
                }
            }
            db.close().expect("Error closing database");
        }
        Err(_) => {
            println!("open WritableDatabase error");
        }
    }
}
