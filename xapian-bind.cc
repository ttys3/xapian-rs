#include "xapian/xapian-bind.h"
#include "xapian/src/lib.rs.h"
#include <iostream>

#include <xapian.h>
#include <string>
#include <string.h>

using namespace Xapian;

/** Open for read/write; create if no db exists. */
const int DB_CREATE_OR_OPEN = 1;
/** Create a new database; fail if db exists. */
const int DB_CREATE = 2;
/** Overwrite existing db; create if none exists. */
const int DB_CREATE_OR_OVERWRITE = 3;

rust::Str version_string()
{
    return Xapian::version_string();
}

std::unique_ptr<Database> new_database()
{
    return std::make_unique<Xapian::Database>();
}

std::unique_ptr<Database> new_database_with_path(rust::Str path, int32_t db_type)
{
    return std::make_unique<Database>(std::string(path), db_type);
}

void add_database(Database &db, Database &add_db)
{
    db.add_database(add_db);
}

void database_close(Database &db)
{
    db.close();
}

void database_reopen(Database &db)
{
    db.reopen();
}

std::unique_ptr<Enquire> new_enquire(Database &db)
{
    return std::make_unique<Xapian::Enquire>(db);
}


//////

std::unique_ptr<Stem> new_stem(rust::Str lang)
{
    return std::make_unique<Stem>(std::string(lang));
}

///////////////////////////////////////////////////////////////
std::unique_ptr<WritableDatabase> new_writable_database_with_path(rust::Str path, int32_t action, int32_t db_type)
{
    // "Honey backend doesn't support updating existing databases"
    return std::make_unique<WritableDatabase>(std::string(path), action | db_type, 0);
}

void commit(WritableDatabase &db)
{
    db.commit();
}

void close(WritableDatabase &db)
{
    db.close();
}

ulong get_doccount (WritableDatabase &db) {
    return db.get_doccount();
}

docid replace_document(WritableDatabase &db, rust::Str unique_term, Document &doc)
{
    return db.replace_document(std::string(unique_term), doc);
}

void delete_document(WritableDatabase &db, rust::Str unique_term)
{
    db.delete_document(std::string(unique_term));
}


////////////////////////////////////////////////////////////////

std::unique_ptr<TermGenerator> new_termgenerator()
{
    return std::make_unique<TermGenerator>();
}

void set_stemmer(TermGenerator &tg, Stem &stem)
{
    tg.set_stemmer(stem);
}

void set_flags (TermGenerator &tg, int32_t toggle, int32_t mask)
{
    tg.set_flags(toggle, mask);
}

void set_document(TermGenerator &tg, Document &doc)
{
    tg.set_document(doc);
}

void index_text(TermGenerator &tg, rust::Str data)
{
    tg.index_text(std::string(data));
}

void index_text_with_prefix(TermGenerator &tg, rust::Str data, rust::Str prefix)
{
    tg.index_text(std::string(data), 1, std::string(prefix));
}

void index_int(TermGenerator &tg, int32_t in_data, rust::Str prefix)
{
    std::string data = sortable_serialise(in_data);
    tg.index_text(data, 1, std::string(prefix));
}

void index_long(TermGenerator &tg, int64_t in_data, rust::Str prefix)
{
    std::string data = sortable_serialise(in_data);
    tg.index_text(data, 1, std::string(prefix));
}

void index_float(TermGenerator &tg, float in_data, rust::Str prefix)
{
    std::string data = sortable_serialise(in_data);
    tg.index_text(data, 1, std::string(prefix));
}

void index_double(TermGenerator &tg, double in_data, rust::Str prefix)
{
    std::string data = sortable_serialise(in_data);
    tg.index_text(data, 1, std::string(prefix));
}

////////////////////////////////////////////////////////////////

std::unique_ptr<Document> new_document()
{
    return std::make_unique<Document>();
}

void add_string(Document &doc, valueno slot, rust::Str data)
{
    doc.add_value(slot, std::string(data));
}

void add_int(Document &doc, valueno slot, int in_data)
{
    std::string data = sortable_serialise(in_data);
    doc.add_value(slot, data);
}

void add_long(Document &doc, valueno slot, int64_t in_data)
{
    std::string data = sortable_serialise(in_data);
    doc.add_value(slot, data);
}

void add_float(Document &doc, valueno slot, float in_data)
{
    std::string data = sortable_serialise(in_data);
    doc.add_value(slot, data);
}

void add_double(Document &doc, valueno slot, double in_data)
{
    std::string data = sortable_serialise(in_data);
    doc.add_value(slot, data);
}

void set_data(Document &doc, rust::Str data)
{
    doc.set_data(std::string(data));
}

void add_boolean_term(Document &doc, rust::Str data)
{
    doc.add_boolean_term(std::string(data));
}

rust::String get_doc_data (Document &doc) {
   return doc.get_data();
}

//////

std::unique_ptr<QueryParser> new_query_parser()
{
    return std::make_unique<Xapian::QueryParser>();
}

void set_max_wildcard_expansion(QueryParser &qp, int32_t limit) {
    qp.set_max_expansion (limit, Query::WILDCARD_LIMIT_MOST_FREQUENT, QueryParser::FLAG_WILDCARD);
}

void set_stemmer_to_qp(QueryParser &qp, Stem &stem) {
    qp.set_stemmer(stem);
}

void set_database(QueryParser &qp, Database &db)
{
    qp.set_database(db);
}

void add_prefix(QueryParser &qp, rust::Str field, rust::Str prefix)
{
    qp.add_prefix(std::string(field), std::string(prefix));
}

void add_rangeprocessor(QueryParser &qp, RangeProcessor &range_proc) {
    std::string empty_grouping;
    qp.add_rangeprocessor(&range_proc, &empty_grouping);
}

void add_number_rangeprocessor(QueryParser &qp, NumberRangeProcessor &range_proc) {
    std::string empty_grouping;
    qp.add_rangeprocessor(&range_proc, &empty_grouping);
}

void add_boolean_prefix(QueryParser &qp, rust::Str field, rust::Str prefix)
{
    std::string empty_grouping;
    qp.add_boolean_prefix(std::string(field), std::string(prefix), &empty_grouping);
}

std::unique_ptr<Query> parse_query(QueryParser &qp, rust::Str data, int32_t flags) {
    return std::make_unique<Xapian::Query>(qp.parse_query(std::string(data), flags));
}

std::unique_ptr<Query> parse_query_with_prefix(QueryParser &qp, rust::Str query, int32_t flags, rust::Str prefix) {
    return std::make_unique<Xapian::Query>(qp.parse_query(std::string(query), flags, std::string(prefix)));
}

////////

std::unique_ptr<Query> new_query() {
    return std::make_unique<Xapian::Query>();
}

std::unique_ptr<Query> new_query_range(int32_t _op, valueno slot, double _begin, double _end) {
    std::string s_begin = Xapian::sortable_serialise(_begin);
    std::string s_end = Xapian::sortable_serialise(_end);
    Xapian::Query _query ((Xapian::Query::op)_op, slot, s_begin, s_end);

    return std::make_unique<Xapian::Query>(_query);
}

std::unique_ptr<Query> add_right_query(Query &this_q, int32_t _op, Query &q) {
    return std::make_unique<Xapian::Query>((Xapian::Query::op)_op, this_q, q);
}

std::unique_ptr<Query> new_query_double_with_prefix(rust::Str prefix, double _d) {
    std::string s = std::string(prefix) + Xapian::sortable_serialise(_d);

    Xapian::Query _query (s);
    return std::make_unique<Xapian::Query>(_query);
}

bool query_is_empty (Query &q) {
    return q.empty();
}

rust::String get_description (Query &q) {
    return q.get_description();
}

////

std::unique_ptr<MSet> get_mset(Enquire &en, int32_t from, int32_t size) {
    return std::make_unique<Xapian::MSet>(en.get_mset(from, size));
}

void set_query(Enquire &en, Query &query) {
    en.set_query(query);
}

void set_sort_by_key(Enquire &en, MultiValueKeyMaker &sorter, bool reverse) {
    en.set_sort_by_key(&sorter, reverse);
}

void add_matchspy_value_count(Enquire &en, ValueCountMatchSpy &vcms) {
    en.add_matchspy(&vcms);
}

void enquire_set_weighting_scheme_bool(Enquire &en, BoolWeight &weight) {
    en.set_weighting_scheme(weight);
}

void enquire_set_weighting_scheme_bm25(Enquire &en, BM25Weight &weight) {
    en.set_weighting_scheme(weight);
}

/////

int get_matches_estimated (MSet &set) {
    return set.get_matches_estimated();
}

int mset_size (MSet &set) {
    return set.size();
}

std::unique_ptr<MSetIterator> mset_begin (MSet &set) {
    return std::make_unique<Xapian::MSetIterator>(set.begin());
}

std::unique_ptr<MSetIterator> mset_end (MSet &set) {
    return std::make_unique<Xapian::MSetIterator>(set.end());
}

std::unique_ptr<MSetIterator> mset_back (MSet &set) {
    return std::make_unique<Xapian::MSetIterator>(set.back());
}

rust::String mset_snippet(MSet &set, rust::Str text, int32_t length, Stem &stem, int32_t flags, rust::Str hi_start,rust::Str hi_end, rust::Str omit) {
    return set.snippet(std::string(text), length, stem, flags, std::string(hi_start), std::string(hi_end), std::string(omit));;
}

std::unique_ptr<Document> mset_iterator_get_document(MSetIterator &iter) {
    return std::make_unique<Xapian::Document>(iter.get_document());
}

bool mset_iterator_eq(MSetIterator &iter, MSetIterator &other) {
    return iter == other;
}

void mset_iterator_next (MSetIterator &iter) {
    iter++;
}

/////

std::unique_ptr<MultiValueKeyMaker> new_multi_value_key_maker () {
    return std::make_unique<Xapian::MultiValueKeyMaker>();
}

void add_value_to_multi_value_key_maker(MultiValueKeyMaker &this_m, valueno slot, bool asc_desc) {
    this_m.add_value(slot, asc_desc);
}

/////

std::unique_ptr<ValueCountMatchSpy> new_value_count_match_spy (valueno slot) {
    return std::make_unique<Xapian::ValueCountMatchSpy>(slot);
}

/////

std::unique_ptr<RangeProcessor> new_range_processor (valueno slot, rust::Str str, int32_t flags) {
    // https://xapian.org/docs/sourcedoc/html/classXapian_1_1RangeProcessor.html#aca78f2633f761f70a2e94e62e741f0ba
    //        Xapian::RangeProcessor::RangeProcessor (	Xapian::valueno 	slot_,const std::string & 	str_ = std::string(),unsigned 	flags_ = 0 )
    //        slot_	Which value slot to generate ranges over.
    //                str_	A string to look for to recognise values as belonging to this range (as a prefix by default, or as a suffix if flags Xapian::RP_SUFFIX is specified).
    //        flags_	Zero or more of the following flags, combined with bitwise-or (| in C++):
    //        Xapian::RP_SUFFIX - require str_ as a suffix instead of a prefix.
    //        Xapian::RP_REPEATED - optionally allow str_ on both ends of the range - e.g. $1..$10 or 5m..50m.
    //                By default a prefix is only checked for on the start (e.g. date:1/1/1980..31/12/1989), and a suffix only on the end (e.g. 2..12kg).

    // enum {
    //    RP_SUFFIX = 1,
    //    RP_REPEATED = 2,
    //    RP_DATE_PREFER_MDY = 4
    //};
    // when flags = 0, str as a prefix by default
    return std::make_unique<Xapian::RangeProcessor>(slot, std::string(str), flags);
}

/////

std::unique_ptr<NumberRangeProcessor> new_number_range_processor (valueno slot, rust::Str prefix, int32_t flags) {
    return std::make_unique<Xapian::NumberRangeProcessor>(slot, std::string(prefix), flags);
}

/////
int value_count_matchspy_get_total(ValueCountMatchSpy &vcms) {
    return vcms.get_total();
}

std::unique_ptr<TermIterator> value_count_matchspy_values_begin(ValueCountMatchSpy &vcms) {
    return std::make_unique<Xapian::TermIterator>(vcms.values_begin());
}

std::unique_ptr<TermIterator> value_count_matchspy_values_end(ValueCountMatchSpy &vcms) {
    return std::make_unique<Xapian::TermIterator>(vcms.values_end());
}

rust::String term_iterator_get_termfreq_value(TermIterator &titer) {
   return rust::String(*titer);
}

int term_iterator_get_termfreq_freq(TermIterator &titer) {
    return titer.get_termfreq();
}

bool term_iterator_eq(TermIterator &titer, TermIterator &other) {
    return titer == other;
}

void term_iterator_next(TermIterator &titer) {
    ++titer;
}

//// Weight

// BoolWeight
std::unique_ptr<BoolWeight> new_bool_weight() {
    return std::make_unique<Xapian::BoolWeight>();
}

// BM25
std::unique_ptr<BM25Weight> new_bm25_weight(double k1, double k2, double k3, double b, double min_normlen) {
    return std::make_unique<Xapian::BM25Weight>(k1, k2, k3, b, min_normlen);
}