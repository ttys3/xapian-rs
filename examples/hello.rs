use xapian::WritableDatabase;

fn main() {
    println!("lib version: {}", xapian::version_string());
    let _ = std::fs::create_dir_all("./data");
    // Honey backend doesn't support updating existing databases
    let mut db = WritableDatabase::new("./data/xapian-hello", xapian::constants::DB_CREATE_OR_OPEN, xapian::constants::DB_BACKEND_HONEY);

    match db {
        Ok(mut db) => {
            println!("open WritableDatabase ok");
            match db.commit() {
                Ok(_) => {}
                Err(e) => {
                    println!("commit error: {}", e);
                }
            }
            db.close().unwrap();
        }
        Err(e) => {
            println!("open WritableDatabase error: {}", e);
        }
    }
}
