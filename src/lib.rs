use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::io;

use cxx::UniquePtr;

/// WritableDatabase flags
pub const DB_BACKEND_GLASS: i32 = 0x100;

/// the chert backend, No longer supported as of Xapian 1.5.0
pub const DB_BACKEND_CHERT: i32 = 0x200;
/// Open a stub database file
pub const DB_BACKEND_STUB: i32 = 0x300;

/** Use the "in memory" backend.
*
*  The filename is currently ignored when this flag is used, but an empty
*  string should be passed to allow for future expansion.
*
*  A new empty database is created, so when creating a Database object this
*  creates an empty read-only database - sometimes useful to avoid special
*  casing this situation, but otherwise of limited use.  It's more useful
*  when creating a WritableDatabase object, though beware that the current
*  inmemory backend implementation was not built for performance and
*  scalability.
*
*  This provides an equivalent to Xapian::InMemory::open() in Xapian 1.2.
 */
pub const DB_BACKEND_INMEMORY: i32 = 0x400;

/** Use the honey backend.
*
*  When opening a WritableDatabase, this means create a honey database if a
*  new database is created.  If there's an existing database (of any type)
*  at the specified path, this flag has no effect.
*
*  When opening a Database, this flag means to only open it if it's a honey
*  database.  There's rarely a good reason to do this - it's mostly provided
*  as equivalent functionality to that provided by the namespaced open()
*  functions in Xapian 1.2.
 */
pub const DB_BACKEND_HONEY: i32	 = 0x500;


/** Create database if it doesn't already exist.
 *
 *  If no opening mode is specified, this is the default.
 */
pub const DB_CREATE_OR_OPEN: i32 = 0x00;

/** Create database if it doesn't already exist, or overwrite if it does. */
pub const DB_CREATE_OR_OVERWRITE: i32 = 0x01;

/** Create a new database.
 *
 *  If the database already exists, an exception will be thrown.
 */
pub const DB_CREATE: i32 = 0x02;

/** Open an existing database.
 *
 *  If the database doesn't exist, an exception will be thrown.
 */
pub const DB_OPEN: i32 = 0x03;

/// Enum of possible query operations
/// #[repr(i32)]
pub enum XapianOp {
    /// Return iff both subqueries are satisfied
    OpAnd,

    /// Return if either subquery is satisfied
    OpOr,

    /// Return if left but not right satisfied
    OpAndNot,

    /// Return if one query satisfied, but not both
    OpXor,

    /// Return iff left satisfied, but use weights from both
    OpAndMaybe,

    /// As AND, but use only weights from left subquery
    OpFilter,

    /** Find occurrences of a list of terms with all the terms
     *  occurring within a specified window of positions.
     *
     *  Each occurrence of a term must be at a different position,
     *  but the order they appear in is irrelevant.
     *
     *  The window parameter should be specified for this operation,
     *  but will default to the number of terms in the list.
     */
    OpNear,

    /** Find occurrences of a list of terms with all the terms
     *  occurring within a specified window of positions, and all
     *  the terms appearing in the order specified.
     *
     *  Each occurrence of a term must be at a different position.
     *
     *  The window parameter should be specified for this operation,
     *  but will default to the number of terms in the list.
     */
    OpPhrase,

    /** Filter by a range test on a document value. */
    OpValueRange,

    /** Scale the weight of a subquery by the specified factor.
     *
     *  A factor of 0 means this subquery will contribute no weight to
     *  the query - it will act as a purely boolean subquery.
     *
     *  If the factor is negative, Xapian::InvalidArgumentError will
     *  be thrown.
     */
    OpScaleWeight,

    /** Pick the best N subqueries and combine with OP_OR.
     *
     *  If you want to implement a feature which finds documents
     *  similar to a piece of text, an obvious approach is to build an
     *  "OR" query from all the terms in the text, and run this query
     *  against a database containing the documents.  However such a
     *  query can contain a lots of terms and be quite slow to perform,
     *  yet many of these terms don't contribute usefully to the
     *  results.
     *
     *  The OP_ELITE_SET operator can be used instead of OP_OR in this
     *  situation.  OP_ELITE_SET selects the most important ''N'' terms
     *  and then acts as an OP_OR query with just these, ignoring any
     *  other terms.  This will usually return results just as good as
     *  the full OP_OR query, but much faster.
     *
     *  In general, the OP_ELITE_SET operator can be used when you have
     *  a large OR query, but it doesn't matter if the search
     *  completely ignores some of the less important terms in the
     *  query.
     *
     *  The subqueries don't have to be terms, but if they aren't then
     *  OP_ELITE_SET will look at the estimated frequencies of the
     *  subqueries and so could pick a subset which don't actually
     *  match any documents even if the full OR would match some.
     *
     *  You can specify a parameter to the query constructor which
     *  control the number of terms which OP_ELITE_SET will pick.  If
     *  not specified, this defaults to 10 (or
     *  <code>ceil(sqrt(number_of_subqueries))</code> if there are more
     *  than 100 subqueries, but this rather arbitrary special case
     *  will be dropped in 1.3.0).  For example, this will pick the
     *  best 7 terms:
     *
     *  <pre>
     *  Xapian::Query query(Xapian::Query::OP_ELITE_SET, subqs.begin(), subqs.end(), 7);
     *  </pre>
     *
     * If the number of subqueries is less than this threshold,
     * OP_ELITE_SET behaves identically to OP_OR.
     */
    OpEliteSet,

    /** Filter by a greater-than-or-equal test on a document value. */
    OpValueGe,

    /** Filter by a less-than-or-equal test on a document value. */
    OpValueLe,

    /** Treat a set of queries as synonyms.
     *
     *  This returns all results which match at least one of the
     *  queries, but weighting as if all the sub-queries are instances
     *  of the same term: so multiple matching terms for a document
     *  increase the wdf value used, and the term frequency is based on
     *  the number of documents which would match an OR of all the
     *  subqueries.
     *
     *  The term frequency used will usually be an approximation,
     *  because calculating the precise combined term frequency would
     *  be overly expensive.
     *
     *  Identical to OP_OR, except for the weightings returned.
     */
    OpSynonym,
}

/// Flags to OR together and pass to TermGenerator::set_flags().
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum TermGeneratorFlag {
    FLAG_DEFAULT = 0,
    /// Index data required for spelling correction.
    FLAG_SPELLING = 128, // Value matches QueryParser flag.

    /** Enable generation of n-grams from CJK text.
     *
     *  With this enabled, spans of CJK characters are split into unigrams
     *  and bigrams, with the unigrams carrying positional information.
     *  Non-CJK characters are split into words as normal.
     *
     *  The corresponding option needs to be passed to QueryParser.
     *
     *  Flag added in Xapian 1.3.4 and 1.2.22.  This mode can be
     *  enabled in 1.2.8 and later by setting environment variable
     *  XAPIAN_CJK_NGRAM to a non-empty value (but doing so was deprecated
     *  in 1.4.11).
     */
    FLAG_CJK_NGRAM = 2048, // Value matches QueryParser flag.
}

/// QueryParser::feature_flag
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug)]
pub enum QueryParserFeatureFlag {
    /// Support AND, OR, etc and bracketed subexpressions.
    FLAG_BOOLEAN = 1,
    /// Support quoted phrases.
    FLAG_PHRASE = 2,
    /// Support + and -.
    FLAG_LOVEHATE = 4,
    /// Support AND, OR, etc even if they aren't in ALLCAPS.
    FLAG_BOOLEAN_ANY_CASE = 8,
    /** Support wildcards.
     *
     *  At present only right truncation (e.g. Xap*) is supported.
     *
     *  Currently you can't use wildcards with boolean filter prefixes,
     *  or in a phrase (either an explicitly quoted one, or one implicitly
     *  generated by hyphens or other punctuation).
     *
     *  In Xapian 1.2.x, you needed to tell the QueryParser object which
     *  database to expand wildcards from by calling set_database().  In
     *  Xapian 1.3.3, OP_WILDCARD was added and wildcards are now
     *  expanded when Enquire::get_mset() is called, with the expansion
     *  using the database being searched.
     */
    FLAG_WILDCARD = 16,
    /** Allow queries such as 'NOT apples'.
     *
     *  These require the use of a list of all documents in the database
     *  which is potentially expensive, so this feature isn't enabled by
     *  default.
     */
    FLAG_PURE_NOT = 32,
    /** Enable partial matching.
     *
     *  Partial matching causes the parser to treat the query as a
     *  "partially entered" search.  This will automatically treat the
     *  final word as a wildcarded match, unless it is followed by
     *  whitespace, to produce more stable results from interactive
     *  searches.
     *
     *  Currently FLAG_PARTIAL doesn't do anything if the final word
     *  in the query has a boolean filter prefix, or if it is in a phrase
     *  (either an explicitly quoted one, or one implicitly generated by
     *  hyphens or other punctuation).  It also doesn't do anything if
     *  if the final word is part of a value range.
     *
     *  In Xapian 1.2.x, you needed to tell the QueryParser object which
     *  database to expand wildcards from by calling set_database().  In
     *  Xapian 1.3.3, OP_WILDCARD was added and wildcards are now
     *  expanded when Enquire::get_mset() is called, with the expansion
     *  using the database being searched.
     */
    FLAG_PARTIAL = 64,

    /** Enable spelling correction.
     *
     *  For each word in the query which doesn't exist as a term in the
     *  database, Database::get_spelling_suggestion() will be called and if
     *  a suggestion is returned, a corrected version of the query string
     *  will be built up which can be read using
     *  QueryParser::get_corrected_query_string().  The query returned is
     *  based on the uncorrected query string however - if you want a
     *  parsed query based on the corrected query string, you must call
     *  QueryParser::parse_query() again.
     *
     *  NB: You must also call set_database() for this to work.
     */
    FLAG_SPELLING_CORRECTION = 128,

    /** Enable synonym operator '~'.
     *
     *  NB: You must also call set_database() for this to work.
     */
    FLAG_SYNONYM = 256,

    /** Enable automatic use of synonyms for single terms.
     *
     *  NB: You must also call set_database() for this to work.
     */
    FLAG_AUTO_SYNONYMS = 512,

    /** Enable automatic use of synonyms for single terms and groups of
     *  terms.
     *
     *  NB: You must also call set_database() for this to work.
     */
    FLAG_AUTO_MULTIWORD_SYNONYMS = 1024,

    /** Enable generation of n-grams from CJK text.
     *
     *  With this enabled, spans of CJK characters are split into unigrams
     *  and bigrams, with the unigrams carrying positional information.
     *  Non-CJK characters are split into words as normal.
     *
     *  The corresponding option needs to have been used at index time.
     *
     *  Flag added in Xapian 1.3.4 and 1.2.22.  This mode can be
     *  enabled in 1.2.8 and later by setting environment variable
     *  XAPIAN_CJK_NGRAM to a non-empty value (but doing so was deprecated
     *  in 1.4.11).
     */
    FLAG_CJK_NGRAM = 2048,

    /** Accumulate unstem and stoplist results.
     *
     *  By default, the unstem and stoplist data is reset by a call to
     *  parse_query(), which makes sense if you use the same QueryParser
     *  object to parse a series of independent queries.
     *
     *  If you're using the same QueryParser object to parse several
     *  fields on the same query form, you may want to have the unstem
     *  and stoplist data combined for all of them, in which case you
     *  can use this flag to prevent this data from being reset.
     *
     *  @since Added in Xapian 1.4.18.
     */
    FLAG_ACCUMULATE = 65536,

    /** Produce a query which doesn't use positional information.
     *
     *  With this flag enabled, no positional information will be used
     *  and any query operations which would use it are replaced by
     *  the nearest equivalent which doesn't (so phrase searches, NEAR
     *  and ADJ will result in OP_AND).
     *
     *  @since Added in Xapian 1.4.19.
     */
    FLAG_NO_POSITIONS = 0x20000,

    /** The default flags.
     *
     *  Used if you don't explicitly pass any to @a parse_query().
     *  The default flags are FLAG_PHRASE|FLAG_BOOLEAN|FLAG_LOVEHATE.
     *
     *  Added in Xapian 1.0.11.
     */
    FLAG_DEFAULT = Self::FLAG_PHRASE as i32 | Self::FLAG_BOOLEAN as i32 | Self::FLAG_LOVEHATE as i32,
}

/// Enum of possible query operations
#[allow(non_camel_case_types)]
/// #[repr(i32)]
pub enum RangeProcessorFlags {
    RP_PREFIX = 0,
    /// as a suffix
    RP_SUFFIX = 1,
    /// optionally allow str_ on both ends of the range - e.g. $1..$10 or 5m..50m.
    RP_REPEATED = 2,
    RP_DATE_PREFER_MDY = 4,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug)]
pub enum SnippetFlags {
    /** Model the relevancy of non-query terms in MSet::snippet().
     *
     *  Non-query terms will be assigned a small weight, and the snippet
     *  will tend to prefer snippets which contain a more interesting
     *  background (where the query term content is equivalent).
     */
    SNIPPET_BACKGROUND_MODEL = 1,
    /** Exhaustively evaluate candidate snippets in MSet::snippet().
     *
     *  Without this flag, snippet generation will stop once it thinks
     *  it has found a "good enough" snippet, which will generally reduce
     *  the time taken to generate a snippet.
     */
    SNIPPET_EXHAUSTIVE = 2,
    /** Return the empty string if no term got matched.
     *
     *  If enabled, snippet() returns an empty string if not a single match
     *  was found in text. If not enabled, snippet() returns a (sub)string
     *  of text without any highlighted terms.
     */
    SNIPPET_EMPTY_WITHOUT_MATCH = 4,

    /** Enable generation of n-grams from CJK text.
     *
     *  This option highlights CJK searches made using the QueryParser
     *  FLAG_CJK_NGRAM flag.  Non-CJK characters are split into words as
     *  normal.
     *
     *  The TermGenerator FLAG_CJK_NGRAM flag needs to have been used at
     *  index time.
     *
     *  This mode can also be enabled by setting environment variable
     *  XAPIAN_CJK_NGRAM to a non-empty value (but doing so was deprecated
     *  in 1.4.11).
     *
     *  @since Added in Xapian 1.4.11.
     */
    SNIPPET_CJK_NGRAM = 2048,
}

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

        pub(crate) fn new_database(err: &mut i8) -> UniquePtr<Database>;
        pub(crate) fn new_database_with_path(path: &str, db_type: i32, err: &mut i8) -> UniquePtr<Database>;
        pub(crate) fn database_reopen(db: Pin<&mut Database>, err: &mut i8);
        pub(crate) fn database_close(db: Pin<&mut Database>, err: &mut i8);
        pub(crate) fn new_enquire(db: Pin<&mut Database>, err: &mut i8) -> UniquePtr<Enquire>;
        pub(crate) fn add_database(db: Pin<&mut Database>, add_db: Pin<&mut Database>, err: &mut i8);

        pub(crate) fn new_stem(lang: &str, err: &mut i8) -> UniquePtr<Stem>;

        pub(crate) fn new_writable_database_with_path(path: &str, action: i32, db_type: i32) -> Result<UniquePtr<WritableDatabase>>;
        pub(crate) fn commit(db: Pin<&mut WritableDatabase>) -> Result<()>;
        pub(crate) fn close(db: Pin<&mut WritableDatabase>, err: &mut i8);
        pub(crate) fn replace_document(db: Pin<&mut WritableDatabase>, unique_term: &str, doc: Pin<&mut Document>, err: &mut i8) -> u32;
        pub(crate) fn delete_document(db: Pin<&mut WritableDatabase>, unique_term: &str, err: &mut i8);
        pub(crate) fn get_doccount(db: Pin<&mut WritableDatabase>, err: &mut i8) -> i32;

        pub(crate) fn new_termgenerator(err: &mut i8) -> UniquePtr<TermGenerator>;
        pub(crate) fn set_stemmer(tg: Pin<&mut TermGenerator>, stem: Pin<&mut Stem>, err: &mut i8);
        pub(crate) fn set_flags(tg: Pin<&mut TermGenerator>, toggle: i32, mask: i32, err: &mut i8);
        pub(crate) fn set_document(tg: Pin<&mut TermGenerator>, doc: Pin<&mut Document>, err: &mut i8);
        pub(crate) fn index_text_with_prefix(tg: Pin<&mut TermGenerator>, data: &str, prefix: &str, err: &mut i8);
        pub(crate) fn index_text(tg: Pin<&mut TermGenerator>, data: &str, err: &mut i8);
        pub(crate) fn index_int(tg: Pin<&mut TermGenerator>, data: i32, prefix: &str, err: &mut i8);
        pub(crate) fn index_long(tg: Pin<&mut TermGenerator>, data: i64, prefix: &str, err: &mut i8);
        pub(crate) fn index_float(tg: Pin<&mut TermGenerator>, data: f32, prefix: &str, err: &mut i8);
        pub(crate) fn index_double(tg: Pin<&mut TermGenerator>, data: f64, prefix: &str, err: &mut i8);

        pub(crate) fn new_document(err: &mut i8) -> UniquePtr<Document>;
        pub(crate) fn add_string(doc: Pin<&mut Document>, slot: u32, data: &str, err: &mut i8);
        pub(crate) fn add_int(doc: Pin<&mut Document>, slot: u32, data: i32, err: &mut i8);
        pub(crate) fn add_long(doc: Pin<&mut Document>, slot: u32, data: i64, err: &mut i8);
        pub(crate) fn add_double(doc: Pin<&mut Document>, slot: u32, data: f64, err: &mut i8);
        pub(crate) fn set_data(doc: Pin<&mut Document>, data: &str, err: &mut i8);
        pub(crate) fn get_doc_data(doc: Pin<&mut Document>) -> &CxxString;
        pub(crate) fn add_boolean_term(doc: Pin<&mut Document>, data: &str, err: &mut i8);

        pub(crate) fn get_matches_estimated(set: Pin<&mut MSet>, err: &mut i8) -> i32;
        pub(crate) fn mset_size(set: Pin<&mut MSet>, err: &mut i8) -> i32;
        pub(crate) fn mset_snippet<'a>(
            set: Pin<&'a mut MSet>,
            text: &'a str,
            length: i32,
            stem: Pin<&'a mut Stem>,
            flags: i32,
            hi_start: &'a str,
            hi_end: &'a str,
            omit: &'a str,
            err: &'a mut i8,
        ) -> &'a CxxString;
        pub(crate) fn mset_iterator_get_document(iter: Pin<&mut MSetIterator>, err: &mut i8) -> UniquePtr<Document>;
        pub(crate) fn mset_iterator_eq(iter: Pin<&mut MSetIterator>, other: Pin<&mut MSetIterator>, err: &mut i8) -> bool;
        pub(crate) fn mset_iterator_next(iter: Pin<&mut MSetIterator>, err: &mut i8);

        pub(crate) fn mset_begin(set: Pin<&mut MSet>, err: &mut i8) -> UniquePtr<MSetIterator>;
        pub(crate) fn mset_end(set: Pin<&mut MSet>, err: &mut i8) -> UniquePtr<MSetIterator>;
        pub(crate) fn mset_back(set: Pin<&mut MSet>, err: &mut i8) -> UniquePtr<MSetIterator>;
        // pub(crate) fn get_doc_by_index(set: Pin<&mut MSet>, index: i32, err: &mut i8) -> UniquePtr<Document>;

        pub(crate) fn get_mset(en: Pin<&mut Enquire>, from: i32, size: i32, err: &mut i8) -> UniquePtr<MSet>;
        pub(crate) fn set_query(en: Pin<&mut Enquire>, query: Pin<&mut Query>, err: &mut i8);
        pub(crate) fn set_sort_by_key(en: Pin<&mut Enquire>, sorter: Pin<&mut MultiValueKeyMaker>, reverse: bool, err: &mut i8);
        pub(crate) fn add_matchspy_value_count(en: Pin<&mut Enquire>, vcms: Pin<&mut ValueCountMatchSpy>, err: &mut i8);

        pub(crate) fn new_query_parser(err: &mut i8) -> UniquePtr<QueryParser>;
        pub(crate) fn set_max_wildcard_expansion(qp: Pin<&mut QueryParser>, limit: i32, err: &mut i8);
        pub(crate) fn set_stemmer_to_qp(qp: Pin<&mut QueryParser>, stem: Pin<&mut Stem>, err: &mut i8);
        pub(crate) fn set_database(qp: Pin<&mut QueryParser>, add_db: Pin<&mut Database>, err: &mut i8);
        pub(crate) fn add_prefix(qp: Pin<&mut QueryParser>, field: &str, prefix: &str, err: &mut i8);
        pub(crate) fn add_boolean_prefix(qp: Pin<&mut QueryParser>, field: &str, prefix: &str, err: &mut i8);
        pub(crate) fn add_rangeprocessor(qp: Pin<&mut QueryParser>, range_proc: Pin<&mut RangeProcessor>, err: &mut i8);
        pub(crate) fn add_number_rangeprocessor(qp: Pin<&mut QueryParser>, range_proc: Pin<&mut NumberRangeProcessor>, err: &mut i8);
        pub(crate) fn parse_query(qp: Pin<&mut QueryParser>, query_string: &str, flags: i32, err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn parse_query_with_prefix(qp: Pin<&mut QueryParser>, query_string: &str, flags: i32, prefix: &str, err: &mut i8) -> UniquePtr<Query>;

        // pub(crate) fn new_query(err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn new_query_range(op: i32, slot: u32, begin: f64, end: f64, err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn add_right_query(this_q: Pin<&mut Query>, op: i32, q: Pin<&mut Query>, err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn new_query_double_with_prefix(prefix: &str, d: f64, err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn query_is_empty(this_q: Pin<&mut Query>, err: &mut i8) -> bool;
        pub(crate) fn get_description(this_q: Pin<&mut Query>) -> &CxxString;

        pub(crate) fn new_multi_value_key_maker(err: &mut i8) -> UniquePtr<MultiValueKeyMaker>;
        pub(crate) fn add_value_to_multi_value_key_maker(this_m: Pin<&mut MultiValueKeyMaker>, slot: u32, asc_desc: bool, err: &mut i8);

        pub(crate) fn new_value_count_match_spy(slot: u32, err: &mut i8) -> UniquePtr<ValueCountMatchSpy>;
        pub(crate) fn new_range_processor(slot: u32, prefix: &str, flags: i32, err: &mut i8) -> UniquePtr<RangeProcessor>;
        pub(crate) fn new_number_range_processor(slot: u32, prefix: &str, flags: i32, err: &mut i8) -> UniquePtr<NumberRangeProcessor>;

        pub(crate) fn value_count_matchspy_values_begin(vcms: Pin<&mut ValueCountMatchSpy>, err: &mut i8) -> UniquePtr<TermIterator>;
        pub(crate) fn value_count_matchspy_values_end(vcms: Pin<&mut ValueCountMatchSpy>, err: &mut i8) -> UniquePtr<TermIterator>;
        pub(crate) fn value_count_matchspy_get_total(vcms: Pin<&mut ValueCountMatchSpy>, err: &mut i8) -> i32;

        pub(crate) fn term_iterator_get_termfreq_value<'a>(titer: Pin<&'a mut TermIterator>, err: &'a mut i8) -> &'a CxxString;
        pub(crate) fn term_iterator_get_termfreq_freq(titer: Pin<&mut TermIterator>, err: &mut i8) -> i32;
        pub(crate) fn term_iterator_eq(titer: Pin<&mut TermIterator>, other: Pin<&mut TermIterator>, err: &mut i8) -> bool;
        pub(crate) fn term_iterator_next(titer: Pin<&mut TermIterator>, err: &mut i8);
    }
}

#[warn(unused_unsafe)]

pub struct MultiValueKeyMaker {
    pub cxxp: UniquePtr<ffi::MultiValueKeyMaker>,
}

impl MultiValueKeyMaker {
    pub fn new() -> Result<Self, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let obj = ffi::new_multi_value_key_maker(&mut err);

            if err == 0 {
                Ok(Self { cxxp: obj })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_value(&mut self, slot: u32, asc_desc: bool) -> Result<(), XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            ffi::add_value_to_multi_value_key_maker(self.cxxp.pin_mut(), slot, asc_desc, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

pub struct Query {
    pub cxxp: UniquePtr<ffi::Query>,
}

impl Query {
    pub fn new() -> Result<Self, XError> {
        Ok(Self { cxxp: UniquePtr::null() })
    }

    pub fn new_range(op: XapianOp, slot: u32, begin: f64, end: f64) -> Result<Self, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let obj = ffi::new_query_range(op as i32, slot, begin, end, &mut err);

            if err == 0 {
                Ok(Self { cxxp: obj })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_right(&mut self, op: XapianOp, q: &mut Query) -> Result<Self, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let obj = ffi::add_right_query(self.cxxp.pin_mut(), op as i32, q.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(Self { cxxp: obj })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn new_double_with_prefix(prefix: &str, d: f64) -> Result<Self, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let obj = ffi::new_query_double_with_prefix(prefix, d, &mut err);

            if err == 0 {
                Ok(Self { cxxp: obj })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.cxxp.is_null()
    }

    pub fn is_empty_content_query(&mut self) -> bool {
        if !self.cxxp.is_null() {
            #[allow(unused_unsafe)]
            unsafe {
                let mut err = 0;
                let res = ffi::query_is_empty(self.cxxp.pin_mut(), &mut err);
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
            #[allow(unused_unsafe)]
            unsafe {
                //let mut err = 0;
                let res = ffi::get_description(self.cxxp.pin_mut());
                //if err == 0 {
                return res.to_string();
                //} else {
                //    None
                //}
            }
        }
        String::default()
    }
}

pub struct QueryParser {
    pub cxxp: UniquePtr<ffi::QueryParser>,
}

#[allow(unused_unsafe)]
impl QueryParser {
    pub fn new() -> Result<Self, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_query_parser(&mut err);

            if err == 0 {
                Ok(Self { cxxp: obj })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn set_max_wildcard_expansion(&mut self, limit: i32) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::set_max_wildcard_expansion(self.cxxp.pin_mut(), limit, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn set_stemmer(&mut self, mut stem: Stem) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::set_stemmer_to_qp(self.cxxp.pin_mut(), stem.cxxp.pin_mut(), &mut err);
            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn set_database(&mut self, database: &mut Database) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::set_database(self.cxxp.pin_mut(), database.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_prefix(&mut self, field: &str, prefix: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::add_prefix(self.cxxp.pin_mut(), field, prefix, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_boolean_prefix(&mut self, field: &str, prefix: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::add_boolean_prefix(self.cxxp.pin_mut(), field, prefix, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_rangeprocessor(&mut self, range_proc: &mut RangeProcessor) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::add_rangeprocessor(self.cxxp.pin_mut(), range_proc.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_number_rangeprocessor(&mut self, range_proc: &mut NumberRangeProcessor) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::add_number_rangeprocessor(self.cxxp.pin_mut(), range_proc.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn parse_query(&mut self, query: &str, flags: i32) -> Result<Query, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::parse_query(self.cxxp.pin_mut(), query, flags, &mut err);
            if err == 0 {
                Ok(Query { cxxp: obj })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn parse_query_with_prefix(&mut self, query: &str, flags: i32, prefix: &str) -> Result<Query, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::parse_query_with_prefix(self.cxxp.pin_mut(), query, flags, prefix, &mut err);
            if err == 0 {
                Ok(Query { cxxp: obj })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

pub struct MSetIterator {
    // pub mset: &'a mut MSet,
    // pub index: i32,
    pub cxxp: UniquePtr<ffi::MSetIterator>,
}

impl MSetIterator {
    // pub fn is_next(&mut self) -> Result<bool, XError> {
    //     #[allow(unused_unsafe)]
    //     unsafe {
    //         let mut err = 0;
    //         let res = ffi::mset_size(self.mset.cxxp.pin_mut(), &mut err) > self.index;
    //
    //         if err == 0 {
    //             Ok(res)
    //         } else {
    //             Err(XError::Xapian(err))
    //         }
    //     }
    // }

    // pub fn next(&mut self) -> Result<(), XError> {
    //     #[allow(unused_unsafe)]
    //     unsafe {
    //         let mut err = 0;
    //         if ffi::mset_size(self.mset.cxxp.pin_mut(), &mut err) > self.index {
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

    pub fn get_document(&mut self) -> Result<Document, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let doc = ffi::mset_iterator_get_document(self.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(Document { cxxp: doc })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn eq(&mut self, other: &mut MSetIterator) -> Result<bool, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let res = ffi::mset_iterator_eq(self.cxxp.pin_mut(), other.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(res)
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn next(&mut self) -> Result<(), XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;

            ffi::mset_iterator_next(self.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
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
    pub fn begin(&mut self) -> Result<MSetIterator, XError> {
        let mut err = 0;
        let obj = ffi::mset_begin(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(MSetIterator { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn end(&mut self) -> Result<MSetIterator, XError> {
        let mut err = 0;
        let obj = ffi::mset_end(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(MSetIterator {
                // mset: self,
                // index: 0,
                cxxp: obj,
            })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn back(&mut self) -> Result<MSetIterator, XError> {
        let mut err = 0;
        let obj = ffi::mset_back(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(MSetIterator {
                // mset: self,
                // index: 0,
                cxxp: obj,
            })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn get_matches_estimated(&mut self) -> Result<i32, XError> {
        #[allow(unused_unsafe)]
        let mut err = 0;
        let res = ffi::get_matches_estimated(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(res)
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn size(&mut self) -> Result<i32, XError> {
        #[allow(unused_unsafe)]
        let mut err = 0;
        let res = ffi::mset_size(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(res)
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn snippet(&mut self, text: &str, length: i32, stem: &mut Stem, flags: i32, hi_start: &str, hi_end: &str, omit: &str) -> String {
        #[allow(unused_unsafe)]
        let mut err = 0;
        let res = ffi::mset_snippet(self.cxxp.pin_mut(), text, length, stem.cxxp.pin_mut(), flags, hi_start, hi_end, omit, &mut err);

        return res.to_string();
    }
}

pub struct Enquire {
    pub cxxp: UniquePtr<ffi::Enquire>,
    sorter: Option<MultiValueKeyMaker>,
}

impl Enquire {
    pub fn get_mset(&mut self, from: i32, size: i32) -> Result<MSet, XError> {
        #[allow(unused_unsafe)]
        let mut err = 0;
        let obj = ffi::get_mset(self.cxxp.pin_mut(), from, size, &mut err);

        if err == 0 {
            Ok(MSet { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn set_query(&mut self, query: &mut Query) -> Result<(), XError> {
        #[allow(unused_unsafe)]
        let mut err = 0;
        ffi::set_query(self.cxxp.pin_mut(), query.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(())
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn set_sort_by_key(&mut self, mut sorter: MultiValueKeyMaker, reverse: bool) -> Result<(), XError> {
        #[allow(unused_unsafe)]
        let mut err = 0;
        ffi::set_sort_by_key(self.cxxp.pin_mut(), sorter.cxxp.pin_mut(), reverse, &mut err);
        self.sorter = Some(sorter);

        if err == 0 {
            Ok(())
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn add_matchspy() {
        unimplemented!()
    }

    pub fn add_matchspy_value_count(&mut self, vcms: &mut ValueCountMatchSpy) -> Result<(), XError> {
        #[allow(unused_unsafe)]
        let mut err = 0;
        ffi::add_matchspy_value_count(self.cxxp.pin_mut(), vcms.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(())
        } else {
            Err(XError::Xapian(err))
        }
    }
}

pub struct Database {
    pub cxxp: UniquePtr<ffi::Database>,
}

#[allow(unused_unsafe)]
impl Database {
    pub fn new() -> Result<Self, XError> {
        let mut err = 0;
        let obj = ffi::new_database(&mut err);

        if err == 0 {
            Ok(Self { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn new_with_path(path: &str, db_type: i32) -> Result<Self, XError> {
        let mut err = 0;
        let obj = ffi::new_database_with_path(path, db_type, &mut err);

        if err == 0 {
            Ok(Self { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn new_enquire(&mut self) -> Result<Enquire, XError> {
        let mut err = 0;
        let obj = ffi::new_enquire(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(Enquire { cxxp: obj, sorter: None })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn add_database(&mut self, database: &mut Database) -> Result<(), XError> {
        let mut err = 0;
        ffi::add_database(self.cxxp.pin_mut(), database.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(())
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn reopen(&mut self) -> Result<(), XError> {
        let mut err = 0;
        ffi::database_reopen(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(())
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn close(&mut self) -> Result<(), XError> {
        let mut err = 0;
        ffi::database_close(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(())
        } else {
            Err(XError::Xapian(err))
        }
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

    pub fn delete_document(&mut self, unique_term: &str) -> Result<(), XError> {
        let mut err = 0;
        ffi::delete_document(self.cxxp.pin_mut(), unique_term, &mut err);
        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn replace_document(&mut self, unique_term: &str, doc: &mut Document) -> Result<(), XError> {
        let mut err = 0;
        ffi::replace_document(self.cxxp.pin_mut(), unique_term, doc.cxxp.pin_mut(), &mut err);
        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), cxx::Exception> {
        ffi::commit(self.cxxp.pin_mut())
    }

    pub fn close(&mut self) -> Result<(), XError> {
        let mut err = 0;
        ffi::close(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(())
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn get_doccount(&mut self) -> Result<i32, XError> {
        let mut err = 0;
        let res = ffi::get_doccount(self.cxxp.pin_mut(), &mut err);
        if err < 0 {
            return Err(XError::Xapian(err));
        } else {
            Ok(res)
        }
    }
}

pub struct Document {
    cxxp: UniquePtr<ffi::Document>,
}

#[allow(unused_unsafe)]
impl Document {
    pub fn new() -> Result<Self, XError> {
        let mut err = 0;
        let obj = ffi::new_document(&mut err);
        if err == 0 {
            Ok(Self { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn add_string(&mut self, slot: u32, data: &str) -> Result<(), XError> {
        let mut err = 0;

        ffi::add_string(self.cxxp.pin_mut(), slot, data, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn add_int(&mut self, slot: u32, data: i32) -> Result<(), XError> {
        let mut err = 0;

        ffi::add_int(self.cxxp.pin_mut(), slot, data, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn add_long(&mut self, slot: u32, data: i64) -> Result<(), XError> {
        let mut err = 0;

        ffi::add_long(self.cxxp.pin_mut(), slot, data, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn add_double(&mut self, slot: u32, data: f64) -> Result<(), XError> {
        let mut err = 0;

        ffi::add_double(self.cxxp.pin_mut(), slot, data, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn set_data(&mut self, data: &str) -> Result<(), XError> {
        let mut err = 0;

        ffi::set_data(self.cxxp.pin_mut(), data, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn get_data(&mut self) -> String {
        let res = ffi::get_doc_data(self.cxxp.pin_mut());

        res.to_string()
    }

    pub fn add_boolean_term(&mut self, data: &str) -> Result<(), XError> {
        let mut err = 0;

        ffi::add_boolean_term(self.cxxp.pin_mut(), data, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }
}

pub struct Stem {
    cxxp: UniquePtr<ffi::Stem>,
}

#[allow(unused_unsafe)]
impl Stem {
    pub fn new(lang: &str) -> Result<Self, XError> {
        let mut err = 0;
        let obj = ffi::new_stem(lang, &mut err);
        if err == 0 {
            Ok(Self { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }
}

pub struct TermGenerator {
    cxxp: UniquePtr<ffi::TermGenerator>,
}

impl TermGenerator {
    pub fn new() -> Result<Self> {
        let mut err = 0;
        let obj = ffi::new_termgenerator(&mut err);
        if err == 0 {
            Ok(Self { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }
}

#[allow(unused_unsafe)]
impl TermGenerator {
    pub fn set_stemmer(&mut self, mut stem: Stem) -> Result<(), XError> {
        let mut err = 0;
        ffi::set_stemmer(self.cxxp.pin_mut(), stem.cxxp.pin_mut(), &mut err);
        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn set_flags(&mut self, toggle: i32, mask: i32) -> Result<(), XError> {
        let mut err = 0;
        ffi::set_flags(self.cxxp.pin_mut(), toggle as i32, mask as i32, &mut err);
        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn set_document(&mut self, doc: &mut Document) -> Result<(), XError> {
        let mut err = 0;

        ffi::set_document(self.cxxp.pin_mut(), doc.cxxp.pin_mut(), &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn index_text_with_prefix(&mut self, data: &str, prefix: &str) -> Result<(), XError> {
        let mut err = 0;

        ffi::index_text_with_prefix(self.cxxp.pin_mut(), data, prefix, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn index_text(&mut self, data: &str) -> Result<(), XError> {
        let mut err = 0;

        ffi::index_text(self.cxxp.pin_mut(), data, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn index_int(&mut self, data: i32, prefix: &str) -> Result<(), XError> {
        let mut err = 0;

        ffi::index_int(self.cxxp.pin_mut(), data, prefix, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn index_long(&mut self, data: i64, prefix: &str) -> Result<(), XError> {
        let mut err = 0;

        ffi::index_long(self.cxxp.pin_mut(), data, prefix, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn index_float(&mut self, data: f32, prefix: &str) -> Result<(), XError> {
        let mut err = 0;

        ffi::index_float(self.cxxp.pin_mut(), data, prefix, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }

    pub fn index_double(&mut self, data: f64, prefix: &str) -> Result<(), XError> {
        let mut err = 0;

        ffi::index_double(self.cxxp.pin_mut(), data, prefix, &mut err);

        if err < 0 {
            return Err(XError::Xapian(err));
        }
        Ok(())
    }
}

#[warn(unused_unsafe)]

pub struct ValueCountMatchSpy {
    pub cxxp: UniquePtr<ffi::ValueCountMatchSpy>,
}

impl ValueCountMatchSpy {
    pub fn new(slot: u32) -> Result<Self, XError> {
        let mut err = 0;
        let obj = ffi::new_value_count_match_spy(slot, &mut err);
        if err == 0 {
            Ok(Self { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }

    // https://xapian.org/docs/facets#toc-entry-5
    // return Xapian::TermIterator
    pub fn values_begin(&mut self) -> Result<TermIterator, XError> {
        let mut err = 0;
        let obj = ffi::value_count_matchspy_values_begin(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(TermIterator { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn values_end(&mut self) -> Result<TermIterator, XError> {
        let mut err = 0;
        let obj = ffi::value_count_matchspy_values_end(self.cxxp.pin_mut(), &mut err);

        if err == 0 {
            Ok(TermIterator { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }

    pub fn get_total(&mut self) -> i32 {
        let mut err = 0;
        let rs = ffi::value_count_matchspy_get_total(self.cxxp.pin_mut(), &mut err);

        return rs;
    }
}

pub struct RangeProcessor {
    pub cxxp: UniquePtr<ffi::RangeProcessor>,
}

impl RangeProcessor {
    pub fn new(slot: u32, prefix: &str, flags: RangeProcessorFlags) -> Result<Self, XError> {
        let mut err = 0;
        let obj = ffi::new_range_processor(slot, prefix, flags as i32, &mut err);
        if err == 0 {
            Ok(Self { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
    }
}

pub struct NumberRangeProcessor {
    pub cxxp: UniquePtr<ffi::NumberRangeProcessor>,
}

impl NumberRangeProcessor {
    pub fn new(slot: u32, prefix: &str, flags: RangeProcessorFlags) -> Result<Self, XError> {
        let mut err = 0;
        let obj = ffi::new_number_range_processor(slot, prefix, flags as i32, &mut err);
        if err == 0 {
            Ok(Self { cxxp: obj })
        } else {
            Err(XError::Xapian(err))
        }
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
        let mut err = 0;
        let rs = ffi::term_iterator_get_termfreq_value(self.cxxp.pin_mut(), &mut err);

        return rs.to_string();
    }

    pub fn get_termfreq_freq(&mut self) -> i32 {
        let mut err = 0;
        let rs = ffi::term_iterator_get_termfreq_freq(self.cxxp.pin_mut(), &mut err);

        return rs;
    }

    pub fn eq(&mut self, other: &mut TermIterator) -> bool {
        let mut err = 0;
        let rs = ffi::term_iterator_eq(self.cxxp.pin_mut(), other.cxxp.pin_mut(), &mut err);

        return rs;
    }

    pub fn next(&mut self) {
        let mut err = 0;
        ffi::term_iterator_next(self.cxxp.pin_mut(), &mut err);
    }
}

pub fn get_xapian_err_type(errcode: i8) -> &'static str {
    match errcode {
        0 => "AssertionError",
        -1 => "InvalidArgumentError",
        -2 => "InvalidOperationError",
        -3 => "UnimplementedError",
        -4 => "DatabaseError",
        -5 => "DatabaseCorruptError",
        -6 => "DatabaseCreateError",
        -7 => "DatabaseLockError",
        -10 => "DatabaseModifiedError",
        -11 => "DatabaseOpeningError",
        -12 => "DatabaseVersionError",
        -13 => "DocNotFoundError",
        -14 => "FeatureUnavailableError",
        -15 => "InternalError",
        -16 => "NetworkError",
        -17 => "NetworkTimeoutError",
        -20 => "QueryParserError",
        -21 => "SerialisationError",
        -22 => "RangeError",
        -23 => "WildcardError",
        -24 => "DatabaseNotFoundError",
        -25 => "DatabaseClosedError",
        _ => "Unknown",
    }
}

pub type Result<T, E = XError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum XError {
    Xapian(i8),
    Io(io::Error),
}

impl Display for XError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XError::Xapian(err) => write!(f, "xapian err={}", err),
            XError::Io(err) => err.fmt(f),
        }
    }
}

impl StdError for XError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            XError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for XError {
    fn from(err: io::Error) -> Self {
        XError::Io(err)
    }
}

impl From<i8> for XError {
    fn from(err: i8) -> Self {
        XError::Xapian(err)
    }
}

impl From<XError> for i8 {
    fn from(err: XError) -> i8 {
        err.into()
    }
}
