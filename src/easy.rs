use autocxx::prelude::*;

use cxx::UniquePtr;

// autocxx::include_cpp! {
//         #include "xapian.h"
//         #include "easy_wrapper.h"
//         safety!(unsafe_ffi)
//
//         extern_cpp_type!("Xapian", crate::ffi::Xapian)
//         extern_cpp_type!("WritableDatabase", crate::ffi::Xapian::WritableDatabase)
//
//         // generate!("writable_database_close")
// }

pub struct WritableDatabase {
    db: UniquePtr<crate::ffi_base::Xapian::WritableDatabase>,
}

impl WritableDatabase {
    pub fn new(path: &str, flags: i32, backend: i32) -> Self {
        cxx::let_cxx_string!(path = path);
        let db = crate::ffi_base::Xapian::WritableDatabase::new1(&path, c_int(flags), c_int(backend)).within_unique_ptr();
        Self { db }
    }

    pub fn commit(&mut self) {
        self.db.pin_mut().commit()
    }

    pub fn close(&mut self) {
        crate::ffi_base::writable_database_close(self.db.pin_mut());
    }
}

impl Default for WritableDatabase {
    fn default() -> Self {
        let db = crate::ffi_base::Xapian::WritableDatabase::new().within_unique_ptr();
        Self { db }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::easy::WritableDatabase;
    use crate::ffi_base::Xapian::{DB_CREATE_OR_OPEN, DB_BACKEND_HONEY};

    #[test]
    fn test_xapian_wrapper() {
        println!("xapian lib version: {:?}", crate::ffi_base::Xapian::version_string());
        // https://xapian.org/docs/sourcedoc/html/namespaceXapian_1_1Chert.html#ad328887e1b0e513dff7f50f62a645a40
        let _ = std::fs::create_dir_all("./data");
        // Honey backend doesn't support updating existing databases
        cxx::let_cxx_string!(path = "./data/xapian-hello");
        let mut db = WritableDatabase::new("./data/xapian-hello", DB_CREATE_OR_OPEN, DB_BACKEND_HONEY);

        println!("open WritableDatabase ok");
        db.commit();
        db.close();
    }
}