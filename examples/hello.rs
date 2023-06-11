use xapian::base::Xapian::Database;
use xapian::enums::{DB_BACKEND_GLASS, DB_BACKEND_HONEY, DB_CREATE_OR_OPEN};
use xapian::WritableDatabase;

fn main() {
    // https://xapian.org/docs/sourcedoc/html/namespaceXapian_1_1Chert.html#ad328887e1b0e513dff7f50f62a645a40
    let _ = std::fs::create_dir_all("./data");
    // Honey backend doesn't support updating existing databases
    let mut db = WritableDatabase::new("./data/xapian-hello", DB_CREATE_OR_OPEN, DB_BACKEND_HONEY);

    println!("open WritableDatabase ok");
    db.commit();
    db.close();
}
