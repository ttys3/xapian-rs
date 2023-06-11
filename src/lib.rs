pub mod constants;

use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::io;

use cxx::UniquePtr;

#[cxx::bridge]
pub(crate) mod ffi {

    #[namespace = "Xapian"]
    extern "C++" {
        pub(crate) type Database;
        pub(crate) type Stem;
        pub(crate) type WritableDatabase;
        pub(crate) type TermGenerator;
        pub(crate) type Document;
        pub(crate) type MSet;
        pub(crate) type MSetIterator;
        pub(crate) type TermIterator;
        pub(crate) type Enquire;
        pub(crate) type QueryParser;
        pub(crate) type Query;
        pub(crate) type MultiValueKeyMaker;
        pub(crate) type RangeProcessor;
        pub(crate) type NumberRangeProcessor;
        pub(crate) type MatchSpy;
        pub(crate) type ValueCountMatchSpy;
    }

    unsafe extern "C++" {
        include!("xapian/xapian-bind.h");

        pub(crate) fn version_string() -> &'static str;
        pub(crate) fn new_database() -> Result<UniquePtr<Database>>;
        pub(crate) fn new_database_with_path(path: &str, db_type: i32) -> Result<UniquePtr<Database>>;
        pub(crate) fn database_reopen(db: Pin<&mut Database>) -> Result<()>;
        pub(crate) fn database_close(db: Pin<&mut Database>) -> Result<()>;
        pub(crate) fn new_enquire(db: Pin<&mut Database>) -> Result<UniquePtr<Enquire>>;
        pub(crate) fn add_database(db: Pin<&mut Database>, add_db: Pin<&mut Database>) -> Result<()>;

        pub(crate) fn new_stem(lang: &str) -> Result<UniquePtr<Stem>>;

        pub(crate) fn new_writable_database_with_path(path: &str, action: i32, db_type: i32) -> Result<UniquePtr<WritableDatabase>>;
        pub(crate) fn commit(db: Pin<&mut WritableDatabase>) -> Result<()>;
        pub(crate) fn close(db: Pin<&mut WritableDatabase>) -> Result<()>;

        pub(crate) fn replace_document(db: Pin<&mut WritableDatabase>, unique_term: &str, doc: Pin<&mut Document>) -> Result<u32>;

        pub(crate) fn delete_document(db: Pin<&mut WritableDatabase>, unique_term: &str) -> Result<()>;
        pub(crate) fn get_doccount(db: Pin<&mut WritableDatabase>) -> Result<usize>;

        pub(crate) fn new_termgenerator() -> Result<UniquePtr<TermGenerator>>;
        pub(crate) fn set_stemmer(tg: Pin<&mut TermGenerator>, stem: Pin<&mut Stem>) -> Result<()>;
        pub(crate) fn set_flags(tg: Pin<&mut TermGenerator>, toggle: i32, mask: i32) -> Result<()>;
        pub(crate) fn set_document(tg: Pin<&mut TermGenerator>, doc: Pin<&mut Document>) -> Result<()>;
        pub(crate) fn index_text_with_prefix(tg: Pin<&mut TermGenerator>, data: &str, prefix: &str) -> Result<()>;
        pub(crate) fn index_text(tg: Pin<&mut TermGenerator>, data: &str) -> Result<()>;
        pub(crate) fn index_int(tg: Pin<&mut TermGenerator>, data: i32, prefix: &str) -> Result<()>;
        pub(crate) fn index_long(tg: Pin<&mut TermGenerator>, data: i64, prefix: &str) -> Result<()>;
        pub(crate) fn index_float(tg: Pin<&mut TermGenerator>, data: f32, prefix: &str) -> Result<()>;
        pub(crate) fn index_double(tg: Pin<&mut TermGenerator>, data: f64, prefix: &str) -> Result<()>;

        pub(crate) fn new_document() -> Result<UniquePtr<Document>>;
        pub(crate) fn add_string(doc: Pin<&mut Document>, slot: u32, data: &str) -> Result<()>;
        pub(crate) fn add_int(doc: Pin<&mut Document>, slot: u32, data: i32) -> Result<()>;
        pub(crate) fn add_long(doc: Pin<&mut Document>, slot: u32, data: i64) -> Result<()>;
        pub(crate) fn add_double(doc: Pin<&mut Document>, slot: u32, data: f64) -> Result<()>;
        pub(crate) fn set_data(doc: Pin<&mut Document>, data: &str) -> Result<()>;
        pub(crate) fn get_doc_data(doc: Pin<&mut Document>) -> Result<String>;
        pub(crate) fn add_boolean_term(doc: Pin<&mut Document>, data: &str) -> Result<()>;

        pub(crate) fn get_matches_estimated(set: Pin<&mut MSet>) -> Result<i32>;
        pub(crate) fn mset_size(set: Pin<&mut MSet>) -> Result<i32>;
        pub(crate) fn mset_snippet(
            set: Pin<&mut MSet>,
            text: &str,
            length: i32,
            stem: Pin<&mut Stem>,
            flags: i32,
            hi_start: &str,
            hi_end: &str,
            omit: &str,
        ) -> String;
        pub(crate) fn mset_iterator_get_document(iter: Pin<&mut MSetIterator>) -> Result<UniquePtr<Document>>;
        pub(crate) fn mset_iterator_eq(iter: Pin<&mut MSetIterator>, other: Pin<&mut MSetIterator>) -> Result<bool>;
        pub(crate) fn mset_iterator_next(iter: Pin<&mut MSetIterator>) -> Result<()>;

        pub(crate) fn mset_begin(set: Pin<&mut MSet>) -> Result<UniquePtr<MSetIterator>>;
        pub(crate) fn mset_end(set: Pin<&mut MSet>) -> Result<UniquePtr<MSetIterator>>;
        pub(crate) fn mset_back(set: Pin<&mut MSet>) -> Result<UniquePtr<MSetIterator>>;
        // pub(crate) fn get_doc_by_index(set: Pin<&mut MSet>, index: i32) -> Result<UniquePtr<Document>>;

        pub(crate) fn get_mset(en: Pin<&mut Enquire>, from: i32, size: i32) -> Result<UniquePtr<MSet>>;
        pub(crate) fn set_query(en: Pin<&mut Enquire>, query: Pin<&mut Query>) -> Result<()>;
        pub(crate) fn set_sort_by_key(en: Pin<&mut Enquire>, sorter: Pin<&mut MultiValueKeyMaker>, reverse: bool) -> Result<()>;
        pub(crate) fn add_matchspy_value_count(en: Pin<&mut Enquire>, vcms: Pin<&mut ValueCountMatchSpy>) -> Result<()>;

        pub(crate) fn new_query_parser() -> Result<UniquePtr<QueryParser>>;
        pub(crate) fn set_max_wildcard_expansion(qp: Pin<&mut QueryParser>, limit: i32) -> Result<()>;
        pub(crate) fn set_stemmer_to_qp(qp: Pin<&mut QueryParser>, stem: Pin<&mut Stem>) -> Result<()>;
        pub(crate) fn set_database(qp: Pin<&mut QueryParser>, add_db: Pin<&mut Database>) -> Result<()>;
        pub(crate) fn add_prefix(qp: Pin<&mut QueryParser>, field: &str, prefix: &str) -> Result<()>;
        pub(crate) fn add_boolean_prefix(qp: Pin<&mut QueryParser>, field: &str, prefix: &str) -> Result<()>;
        pub(crate) fn add_rangeprocessor(qp: Pin<&mut QueryParser>, range_proc: Pin<&mut RangeProcessor>) -> Result<()>;
        pub(crate) fn add_number_rangeprocessor(qp: Pin<&mut QueryParser>, range_proc: Pin<&mut NumberRangeProcessor>) -> Result<()>;
        pub(crate) fn parse_query(qp: Pin<&mut QueryParser>, query_string: &str, flags: i32) -> Result<UniquePtr<Query>>;
        pub(crate) fn parse_query_with_prefix(qp: Pin<&mut QueryParser>, query_string: &str, flags: i32, prefix: &str) -> Result<UniquePtr<Query>>;

        pub(crate) fn new_query() -> Result<UniquePtr<Query>>;
        pub(crate) fn new_query_range(op: i32, slot: u32, begin: f64, end: f64) -> Result<UniquePtr<Query>>;
        pub(crate) fn add_right_query(this_q: Pin<&mut Query>, op: i32, q: Pin<&mut Query>) -> Result<UniquePtr<Query>>;
        pub(crate) fn new_query_double_with_prefix(prefix: &str, d: f64) -> Result<UniquePtr<Query>>;
        pub(crate) fn query_is_empty(this_q: Pin<&mut Query>) -> bool;
        pub(crate) fn get_description(this_q: Pin<&mut Query>) -> String;

        pub(crate) fn new_multi_value_key_maker() -> Result<UniquePtr<MultiValueKeyMaker>>;
        pub(crate) fn add_value_to_multi_value_key_maker(this_m: Pin<&mut MultiValueKeyMaker>, slot: u32, asc_desc: bool) -> Result<()>;

        pub(crate) fn new_value_count_match_spy(slot: u32) -> Result<UniquePtr<ValueCountMatchSpy>>;
        pub(crate) fn new_range_processor(slot: u32, prefix: &str, flags: i32) -> Result<UniquePtr<RangeProcessor>>;
        pub(crate) fn new_number_range_processor(slot: u32, prefix: &str, flags: i32) -> Result<UniquePtr<NumberRangeProcessor>>;

        pub(crate) fn value_count_matchspy_values_begin(vcms: Pin<&mut ValueCountMatchSpy>) -> Result<UniquePtr<TermIterator>>;
        pub(crate) fn value_count_matchspy_values_end(vcms: Pin<&mut ValueCountMatchSpy>) -> Result<UniquePtr<TermIterator>>;
        pub(crate) fn value_count_matchspy_get_total(vcms: Pin<&mut ValueCountMatchSpy>) -> i32;

        pub(crate) fn term_iterator_get_termfreq_value(titer: Pin<&mut TermIterator>) -> String;
        pub(crate) fn term_iterator_get_termfreq_freq(titer: Pin<&mut TermIterator>) -> i32;
        pub(crate) fn term_iterator_eq(titer: Pin<&mut TermIterator>, other: Pin<&mut TermIterator>) -> bool;
        pub(crate) fn term_iterator_next(titer: Pin<&mut TermIterator>);
    }
}

pub fn version_string() ->&'static str {
    ffi::version_string()
}

#[warn(unused_unsafe)]
pub struct MultiValueKeyMaker {
    pub cxxp: UniquePtr<ffi::MultiValueKeyMaker>,
}

impl MultiValueKeyMaker {
    pub fn new() -> Result<Self, cxx::Exception> {
        Ok(Self {
            cxxp: ffi::new_multi_value_key_maker()?,
        })
    }

    pub fn add_value(&mut self, slot: u32, asc_desc: bool) -> Result<(), cxx::Exception> {
        ffi::add_value_to_multi_value_key_maker(self.cxxp.pin_mut(), slot, asc_desc)?;
        Ok(())
    }
}

pub struct Query {
    pub cxxp: UniquePtr<ffi::Query>,
}

impl Query {
    pub fn new() -> Result<Self, cxx::Exception> {
        Ok(Self { cxxp: ffi::new_query()? })
    }

    pub fn new_range(op: constants::XapianOp, slot: u32, begin: f64, end: f64) -> Result<Self, cxx::Exception> {
        Ok(Self {
            cxxp: ffi::new_query_range(op as i32, slot, begin, end)?,
        })
    }

    pub fn add_right(&mut self, op: constants::XapianOp, q: &mut Query) -> Result<Self, cxx::Exception> {
        Ok(Self {
            cxxp: ffi::add_right_query(self.cxxp.pin_mut(), op as i32, q.cxxp.pin_mut())?,
        })
    }

    pub fn new_double_with_prefix(prefix: &str, d: f64) -> Result<Self, cxx::Exception> {
        Ok(Self {
            cxxp: ffi::new_query_double_with_prefix(prefix, d)?,
        })
    }

    pub fn is_empty(&mut self) -> bool {
        self.cxxp.is_null()
    }

    pub fn is_empty_content_query(&mut self) -> bool {
        if !self.cxxp.is_null() {
            #[allow(unused_unsafe)]
            unsafe {
                let mut err = 0;
                let res = ffi::query_is_empty(self.cxxp.pin_mut());
                if err == 0 {
                    return res;
                } else {
                    return true;
                }
            }
        }
        true
    }

    pub fn get_description(&mut self) -> String {
        if !self.cxxp.is_null() {
            let res = ffi::get_description(self.cxxp.pin_mut());
            return res.to_string();
        }
        String::default()
    }
}

pub struct QueryParser {
    pub cxxp: UniquePtr<ffi::QueryParser>,
}

impl QueryParser {
    pub fn new() -> Result<Self, cxx::Exception> {
        unsafe {
            Ok(Self {
                cxxp: ffi::new_query_parser()?,
            })
        }
    }

    pub fn set_max_wildcard_expansion(&mut self, limit: i32) -> Result<(), cxx::Exception> {
        unsafe {
            ffi::set_max_wildcard_expansion(self.cxxp.pin_mut(), limit)?;
            Ok(())
        }
    }

    pub fn set_stemmer(&mut self, mut stem: Stem) -> Result<(), cxx::Exception> {
        ffi::set_stemmer_to_qp(self.cxxp.pin_mut(), stem.cxxp.pin_mut())?;
        Ok(())
    }

    pub fn set_database(&mut self, database: &mut Database) -> Result<(), cxx::Exception> {
        ffi::set_database(self.cxxp.pin_mut(), database.cxxp.pin_mut())?;
        Ok(())
    }

    pub fn add_prefix(&mut self, field: &str, prefix: &str) -> Result<(), cxx::Exception> {
        ffi::add_prefix(self.cxxp.pin_mut(), field, prefix)?;
        Ok(())
    }

    pub fn add_boolean_prefix(&mut self, field: &str, prefix: &str) -> Result<(), cxx::Exception> {
        ffi::add_boolean_prefix(self.cxxp.pin_mut(), field, prefix)?;
        Ok(())
    }

    pub fn add_rangeprocessor(&mut self, range_proc: &mut RangeProcessor) -> Result<(), cxx::Exception> {
        ffi::add_rangeprocessor(self.cxxp.pin_mut(), range_proc.cxxp.pin_mut())?;
        Ok(())
    }

    pub fn add_number_rangeprocessor(&mut self, range_proc: &mut NumberRangeProcessor) -> Result<(), cxx::Exception> {
        ffi::add_number_rangeprocessor(self.cxxp.pin_mut(), range_proc.cxxp.pin_mut())?;
        Ok(())
    }

    pub fn parse_query(&mut self, query: &str, flags: i32) -> Result<Query, cxx::Exception> {
        unsafe {
            Ok(Query {
                cxxp: ffi::parse_query(self.cxxp.pin_mut(), query, flags)?,
            })
        }
    }

    pub fn parse_query_with_prefix(&mut self, query: &str, flags: i32, prefix: &str) -> Result<Query, cxx::Exception> {
        Ok(Query {
            cxxp: ffi::parse_query_with_prefix(self.cxxp.pin_mut(), query, flags, prefix)?,
        })
    }
}

pub struct MSetIterator {
    // pub mset: &'a mut MSet,
    // pub index: i32,
    pub cxxp: UniquePtr<ffi::MSetIterator>,
}

impl MSetIterator {
    // pub fn is_next(&mut self) -> Result<bool, cxx::Exception> {
    //     #[allow(unused_unsafe)]
    //     unsafe {
    //         let mut err = 0;
    //         let res = ffi::mset_size(self.mset.cxxp.pin_mut()) > self.index;
    //
    //         if err == 0 {
    //             Ok(res)
    //         } else {
    //             Err(XError::Xapian(err))
    //         }
    //     }
    // }

    // pub fn next(&mut self) -> Result<(), cxx::Exception> {
    //     #[allow(unused_unsafe)]
    //     unsafe {
    //         let mut err = 0;
    //         if ffi::mset_size(self.mset.cxxp.pin_mut()) > self.index {
    //             self.index += 1;
    //         }
    //
    //         if err == 0 {
    //             Ok(())
    //         } else {
    //             Err(XError::Xapian(err))
    //         }
    //     }
    // }

    pub fn get_document(&mut self) -> Result<Document, cxx::Exception> {
        Ok(Document {
            cxxp: ffi::mset_iterator_get_document(self.cxxp.pin_mut())?,
        })
    }

    pub fn eq(&mut self, other: &mut MSetIterator) -> Result<bool, cxx::Exception> {
        Ok(ffi::mset_iterator_eq(self.cxxp.pin_mut(), other.cxxp.pin_mut())?)
    }

    pub fn next(&mut self) -> Result<(), cxx::Exception> {
        ffi::mset_iterator_next(self.cxxp.pin_mut())?;

        Ok(())
    }
}

pub struct MSet {
    pub cxxp: UniquePtr<ffi::MSet>,
}

impl MSet {
    // pub fn iterator(&mut self) -> Result<MSetIterator, i8> {
    //     Ok(MSetIterator {
    //         mset: self,
    //         index: 0,
    //         cxxp: self.cxxp.clone(),
    //     })
    // }

    // https://xapian.org/docs/sourcedoc/html/classXapian_1_1MSet.html#ad00d5e7f564fe0e5031cb5f89b829ffe
    pub fn begin(&mut self) -> Result<MSetIterator, cxx::Exception> {
        Ok(MSetIterator {
            cxxp: ffi::mset_begin(self.cxxp.pin_mut())?,
        })
    }

    pub fn end(&mut self) -> Result<MSetIterator, cxx::Exception> {
        Ok(MSetIterator {
            cxxp: ffi::mset_end(self.cxxp.pin_mut())?,
        })
    }

    pub fn back(&mut self) -> Result<MSetIterator, cxx::Exception> {
        Ok(MSetIterator {
            cxxp: ffi::mset_back(self.cxxp.pin_mut())?,
        })
    }

    pub fn get_matches_estimated(&mut self) -> Result<i32, cxx::Exception> {
        Ok(ffi::get_matches_estimated(self.cxxp.pin_mut())?)
    }

    pub fn size(&mut self) -> Result<i32, cxx::Exception> {
        Ok(ffi::mset_size(self.cxxp.pin_mut())?)
    }

    pub fn snippet(&mut self, text: &str, length: i32, stem: &mut Stem, flags: i32, hi_start: &str, hi_end: &str, omit: &str) -> String {
        let res = ffi::mset_snippet(self.cxxp.pin_mut(), text, length, stem.cxxp.pin_mut(), flags, hi_start, hi_end, omit);
        return res.to_string();
    }
}

pub struct Enquire {
    pub cxxp: UniquePtr<ffi::Enquire>,
    sorter: Option<MultiValueKeyMaker>,
}

impl Enquire {
    pub fn get_mset(&mut self, from: i32, size: i32) -> Result<MSet, cxx::Exception> {
        Ok(MSet {
            cxxp: ffi::get_mset(self.cxxp.pin_mut(), from, size)?,
        })
    }

    pub fn set_query(&mut self, query: &mut Query) -> Result<(), cxx::Exception> {
        ffi::set_query(self.cxxp.pin_mut(), query.cxxp.pin_mut())?;

        Ok(())
    }

    pub fn set_sort_by_key(&mut self, mut sorter: MultiValueKeyMaker, reverse: bool) -> Result<(), cxx::Exception> {
        #[allow(unused_unsafe)]
        let mut err = 0;
        ffi::set_sort_by_key(self.cxxp.pin_mut(), sorter.cxxp.pin_mut(), reverse)?;
        self.sorter = Some(sorter);
        Ok(())
    }

    pub fn add_matchspy() {
        unimplemented!()
    }

    pub fn add_matchspy_value_count(&mut self, vcms: &mut ValueCountMatchSpy) -> Result<(), cxx::Exception> {
        #[allow(unused_unsafe)]
        let mut err = 0;
        ffi::add_matchspy_value_count(self.cxxp.pin_mut(), vcms.cxxp.pin_mut())?;

        Ok(())
    }
}

pub struct Database {
    pub cxxp: UniquePtr<ffi::Database>,
}

#[allow(unused_unsafe)]
impl Database {
    pub fn new() -> Result<Self, cxx::Exception> {
        Ok(Self { cxxp: ffi::new_database()? })
    }

    pub fn new_with_path(path: &str, db_type: i32) -> Result<Self, cxx::Exception> {
        Ok(Self {
            cxxp: ffi::new_database_with_path(path, db_type)?,
        })
    }

    pub fn new_enquire(&mut self) -> Result<Enquire, cxx::Exception> {
        let obj = ffi::new_enquire(self.cxxp.pin_mut())?;

        Ok(Enquire { cxxp: obj, sorter: None })
    }

    pub fn add_database(&mut self, database: &mut Database) -> Result<(), cxx::Exception> {
        ffi::add_database(self.cxxp.pin_mut(), database.cxxp.pin_mut())?;
        Ok(())
    }

    pub fn reopen(&mut self) -> Result<(), cxx::Exception> {
        Ok(ffi::database_reopen(self.cxxp.pin_mut())?)
    }

    pub fn close(&mut self) -> Result<(), cxx::Exception> {
        Ok(ffi::database_close(self.cxxp.pin_mut())?)
    }
}

pub struct WritableDatabase {
    cxxp: UniquePtr<ffi::WritableDatabase>,
}

#[allow(unused_unsafe)]
impl WritableDatabase {
    pub fn new(path: &str, action: i32, db_type: i32) -> Result<Self, cxx::Exception> {
        match ffi::new_writable_database_with_path(path, action, db_type) {
            Ok(cxxp) => Ok(WritableDatabase { cxxp }),
            Err(e) => Err(e),
        }
    }

    pub fn delete_document(&mut self, unique_term: &str) -> Result<(), cxx::Exception> {
        ffi::delete_document(self.cxxp.pin_mut(), unique_term)?;
        Ok(())
    }

    pub fn replace_document(&mut self, unique_term: &str, doc: &mut Document) -> Result<u32, cxx::Exception> {
        let docid = ffi::replace_document(self.cxxp.pin_mut(), unique_term, doc.cxxp.pin_mut())?;
        Ok(docid)
    }

    pub fn commit(&mut self) -> Result<(), cxx::Exception> {
        ffi::commit(self.cxxp.pin_mut())
    }

    pub fn close(&mut self) -> Result<(), cxx::Exception> {
        ffi::close(self.cxxp.pin_mut())?;
        Ok(())
    }

    pub fn get_doccount(&mut self) -> Result<usize, cxx::Exception> {
        let res = ffi::get_doccount(self.cxxp.pin_mut())?;
        Ok(res)
    }
}

pub struct Document {
    cxxp: UniquePtr<ffi::Document>,
}

#[allow(unused_unsafe)]
impl Document {
    pub fn new() -> Result<Self, cxx::Exception> {
        Ok(Self { cxxp: ffi::new_document()? })
    }

    pub fn add_string(&mut self, slot: u32, data: &str) -> Result<(), cxx::Exception> {
        ffi::add_string(self.cxxp.pin_mut(), slot, data)?;
        Ok(())
    }

    pub fn add_int(&mut self, slot: u32, data: i32) -> Result<(), cxx::Exception> {
        ffi::add_int(self.cxxp.pin_mut(), slot, data)?;
        Ok(())
    }

    pub fn add_long(&mut self, slot: u32, data: i64) -> Result<(), cxx::Exception> {
        ffi::add_long(self.cxxp.pin_mut(), slot, data)?;
        Ok(())
    }

    pub fn add_double(&mut self, slot: u32, data: f64) -> Result<(), cxx::Exception> {
        ffi::add_double(self.cxxp.pin_mut(), slot, data)?;
        Ok(())
    }

    pub fn set_data(&mut self, data: &str) -> Result<(), cxx::Exception> {
        ffi::set_data(self.cxxp.pin_mut(), data)?;
        Ok(())
    }

    pub fn get_data(&mut self) -> Result<String, cxx::Exception> {
        let res = ffi::get_doc_data(self.cxxp.pin_mut())?;
        Ok(res.to_string())
    }

    pub fn add_boolean_term(&mut self, data: &str) -> Result<(), cxx::Exception> {
        ffi::add_boolean_term(self.cxxp.pin_mut(), data)?;
        Ok(())
    }
}

pub struct Stem {
    cxxp: UniquePtr<ffi::Stem>,
}

#[allow(unused_unsafe)]
impl Stem {
    pub fn new(lang: &str) -> Result<Self, cxx::Exception> {
        let obj = ffi::new_stem(lang)?;
        Ok(Self { cxxp: obj })
    }
}

pub struct TermGenerator {
    cxxp: UniquePtr<ffi::TermGenerator>,
}

impl TermGenerator {
    pub fn new() -> Result<Self, cxx::Exception> {
        Ok(Self {
            cxxp: ffi::new_termgenerator()?,
        })
    }
}

#[allow(unused_unsafe)]
impl TermGenerator {
    pub fn set_stemmer(&mut self, mut stem: Stem) -> Result<(), cxx::Exception> {
        ffi::set_stemmer(self.cxxp.pin_mut(), stem.cxxp.pin_mut())?;
        Ok(())
    }

    pub fn set_flags(&mut self, toggle: i32, mask: i32) -> Result<(), cxx::Exception> {
        ffi::set_flags(self.cxxp.pin_mut(), toggle as i32, mask as i32)?;
        Ok(())
    }

    pub fn set_document(&mut self, doc: &mut Document) -> Result<(), cxx::Exception> {
        ffi::set_document(self.cxxp.pin_mut(), doc.cxxp.pin_mut());
        Ok(())
    }

    pub fn index_text_with_prefix(&mut self, data: &str, prefix: &str) -> Result<(), cxx::Exception> {
        ffi::index_text_with_prefix(self.cxxp.pin_mut(), data, prefix);
        Ok(())
    }

    pub fn index_text(&mut self, data: &str) -> Result<(), cxx::Exception> {
        Ok(ffi::index_text(self.cxxp.pin_mut(), data)?)
    }

    pub fn index_int(&mut self, data: i32, prefix: &str) -> Result<(), cxx::Exception> {
        Ok(ffi::index_int(self.cxxp.pin_mut(), data, prefix)?)
    }

    pub fn index_long(&mut self, data: i64, prefix: &str) -> Result<(), cxx::Exception> {
        Ok(ffi::index_long(self.cxxp.pin_mut(), data, prefix)?)
    }

    pub fn index_float(&mut self, data: f32, prefix: &str) -> Result<(), cxx::Exception> {
        Ok(ffi::index_float(self.cxxp.pin_mut(), data, prefix)?)
    }

    pub fn index_double(&mut self, data: f64, prefix: &str) -> Result<(), cxx::Exception> {
        Ok(ffi::index_double(self.cxxp.pin_mut(), data, prefix)?)
    }
}

#[warn(unused_unsafe)]

pub struct ValueCountMatchSpy {
    pub cxxp: UniquePtr<ffi::ValueCountMatchSpy>,
}

impl ValueCountMatchSpy {
    pub fn new(slot: u32) -> Result<Self, cxx::Exception> {
        Ok(Self {
            cxxp: ffi::new_value_count_match_spy(slot)?,
        })
    }

    // https://xapian.org/docs/facets#toc-entry-5
    // return Xapian::TermIterator
    pub fn values_begin(&mut self) -> Result<TermIterator, cxx::Exception> {
        Ok(TermIterator {
            cxxp: ffi::value_count_matchspy_values_begin(self.cxxp.pin_mut())?,
        })
    }

    pub fn values_end(&mut self) -> Result<TermIterator, cxx::Exception> {
        Ok(TermIterator {
            cxxp: ffi::value_count_matchspy_values_end(self.cxxp.pin_mut())?,
        })
    }

    pub fn get_total(&mut self) -> i32 {
        ffi::value_count_matchspy_get_total(self.cxxp.pin_mut())
    }
}

pub struct RangeProcessor {
    pub cxxp: UniquePtr<ffi::RangeProcessor>,
}

impl RangeProcessor {
    pub fn new(slot: u32, prefix: &str, flags: crate::constants::RangeProcessorFlags) -> Result<Self, cxx::Exception> {
        Ok(Self {
            cxxp: ffi::new_range_processor(slot, prefix, flags as i32)?,
        })
    }
}

pub struct NumberRangeProcessor {
    pub cxxp: UniquePtr<ffi::NumberRangeProcessor>,
}

impl NumberRangeProcessor {
    pub fn new(slot: u32, prefix: &str, flags: crate::constants::RangeProcessorFlags) -> Result<Self, cxx::Exception> {
        Ok(Self {
            cxxp: ffi::new_number_range_processor(slot, prefix, flags as i32)?,
        })
    }
}

#[warn(unused_unsafe)]

pub struct TermIterator {
    pub cxxp: UniquePtr<ffi::TermIterator>,
}

// std::string &term_iterator_get_termfreq_value(TermIterator &titer, int8_t &err);
// int term_iterator_get_termfreq_freq(TermIterator &titer, int8_t &err);
// bool term_iterator_eq(TermIterator &titer, TermIterator &other, int8_t &err);
impl TermIterator {
    pub fn get_termfreq_value(&mut self) -> String {
        let rs = ffi::term_iterator_get_termfreq_value(self.cxxp.pin_mut());
        return rs.to_string();
    }

    pub fn get_termfreq_freq(&mut self) -> i32 {
        let rs = ffi::term_iterator_get_termfreq_freq(self.cxxp.pin_mut());
        return rs;
    }

    pub fn eq(&mut self, other: &mut TermIterator) -> bool {
        let rs = ffi::term_iterator_eq(self.cxxp.pin_mut(), other.cxxp.pin_mut());
        return rs;
    }

    pub fn next(&mut self) {
        ffi::term_iterator_next(self.cxxp.pin_mut());
    }
}

