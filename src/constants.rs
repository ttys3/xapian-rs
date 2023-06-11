
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
pub const DB_BACKEND_HONEY: i32 = 0x500;

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

// Enquire::docid_order
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug)]
pub enum EnquireDocidOrder {
    /** docids sort in ascending order (default) */
    ASCENDING = 1,
    /** docids sort in descending order. */
    DESCENDING = 0,
    /** docids sort in whatever order is most efficient for the backend. */
    DONT_CARE = 2
}