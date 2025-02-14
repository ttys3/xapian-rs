#pragma once
#include "rust/cxx.h"
#include <memory>
#include <xapian.h>
#include <string>
#include <string.h>

#include <stdexcept>

// ref https://github.com/brson/wasm-opt-rs/blob/66b161e294bb332947f8319993ae1f8d3498e1e8/components/wasm-opt-cxx-sys/src/shims.h#L13
// https://brson.github.io/2022/10/26/creating-wasm-opt-rust-bindings-with-cxx
// https://github.com/dtolnay/cxx/pull/74/files#diff-b43c1d065c83e99920c09c2d8dbed19687b44a3aeb8e1400a6f5228064a3629f
// https://cxx.rs/binding/result.html
namespace rust::behavior {
    template <typename Try, typename Fail>
    static void trycatch(Try &&func, Fail &&fail) noexcept try {
        func();
    } catch (const Xapian::Error& e) {
        fail("[Xapian Error] " + std::string(e.get_type()) + ": " + e.get_msg());
    } catch (const std::exception &e) {
        fail(e.what());
    }
}

using namespace Xapian;

rust::Str version_string();

std::unique_ptr<Database> new_database();
std::unique_ptr<Enquire> new_enquire(Database &db);

//
std::unique_ptr<Database> new_database_with_path(rust::Str path, int32_t action);
void database_reopen (Database &db);
void add_database(Database &db, Database &add_db);
void database_close(Database &db);

//
std::unique_ptr<Stem> new_stem(rust::Str lang);

//
std::unique_ptr<WritableDatabase> new_writable_database_with_path(rust::Str path, int32_t action, int32_t db_type);
void commit (WritableDatabase &db);
void close (WritableDatabase &db);
docid replace_document(WritableDatabase &db, rust::Str unique_term, Document &doc);
void delete_document(WritableDatabase &db, rust::Str unique_term);
const std::string &get_db_description (WritableDatabase &db);
ulong get_doccount (WritableDatabase &db);

//
std::unique_ptr<TermGenerator> new_termgenerator();
void set_stemmer (TermGenerator &tg, Stem &stem);
void set_flags (TermGenerator &tg, int32_t toggle, int32_t mask);
void set_document (TermGenerator &tg, Document &doc);
void index_text_with_prefix (TermGenerator &tg, rust::Str data, rust::Str prefix);
void index_text (TermGenerator &tg, rust::Str data);
void index_int (TermGenerator &tg, int32_t data, rust::Str prefix);
void index_long (TermGenerator &tg, int64_t data, rust::Str prefix);
void index_float(TermGenerator &tg, float in_data, rust::Str prefix);
void index_double (TermGenerator &tg, double data, rust::Str prefix);

//
std::unique_ptr<Document> new_document ();
void add_string (Document &doc, valueno slot, rust::Str data);
void add_int (Document &doc, valueno slot, int data);
void add_long(Document &doc, valueno slot, int64_t in_data);
void add_float(Document &doc, valueno slot, float in_data);
void add_double(Document &doc, valueno slot, double in_data);
void set_data (Document &doc, rust::Str data);
void add_boolean_term(Document &doc, rust::Str data);
rust::String get_doc_data (Document &doc);

//
std::unique_ptr<QueryParser> new_query_parser();
void set_max_wildcard_expansion(QueryParser &qp, int32_t limit);
void set_stemmer_to_qp(QueryParser &qp, Stem &stem);
void set_database(QueryParser &qp, Database &db);
void add_prefix(QueryParser &qp, rust::Str field, rust::Str prefix);
void add_boolean_prefix(QueryParser &qp, rust::Str field, rust::Str prefix);
void add_rangeprocessor(QueryParser &qp, RangeProcessor &range_proc);
void add_number_rangeprocessor(QueryParser &qp, NumberRangeProcessor &range_proc);
std::unique_ptr<Query> parse_query(QueryParser &qp, rust::Str data, int32_t flags);
std::unique_ptr<Query> parse_query_with_prefix(QueryParser &qp, rust::Str query, int32_t flags, rust::Str prefix);

//
std::unique_ptr<Query> new_query();
std::unique_ptr<Query> new_query_range(int32_t op, valueno slot, double begin, double end);
std::unique_ptr<Query> new_query_double_with_prefix(rust::Str prefix, double _d);
std::unique_ptr<Query> add_right_query(Query &this_q, int32_t _op, Query &q);
bool query_is_empty (Query &q);
rust::String get_description (Query &q);

// Weight
// BoolWeight
std::unique_ptr<BoolWeight> new_bool_weight();
// BM25Weight
std::unique_ptr<BM25Weight> new_bm25_weight(double k1, double k2, double k3, double b, double min_normlen);

//
std::unique_ptr <MSet> get_mset(Enquire &en, int32_t from, int32_t size);
void set_query(Enquire &en, Query &query);
void set_sort_by_key(Enquire &en, MultiValueKeyMaker &sorter, bool reverse);
void add_matchspy_value_count(Enquire &en, ValueCountMatchSpy &vcms);
void enquire_set_weighting_scheme_bool(Enquire &en, BoolWeight &weight);
void enquire_set_weighting_scheme_bm25(Enquire &en, BM25Weight &weight);
void enquire_set_docid_order(Enquire &en, int32_t order);
void enquire_set_sort_by_relevance(Enquire &en);
void enquire_set_sort_by_value(Enquire &en, valueno sort_key, bool reverse);
void enquire_set_sort_by_relevance_then_value(Enquire &en, valueno sort_key, bool reverse);
void enquire_set_collapse_key(Enquire &en, valueno collapse_key, doccount collapse_max);

//
int get_matches_estimated (MSet &set);
int mset_size (MSet &set);
rust::String mset_snippet(MSet &set, rust::Str text, int32_t length, Stem &stem, int32_t flags, rust::Str hi_start,rust::Str hi_end, rust::Str omit);
std::unique_ptr<MSetIterator> mset_begin (MSet &set);
std::unique_ptr<MSetIterator> mset_end (MSet &set);
std::unique_ptr<MSetIterator> mset_back (MSet &set);

//
std::unique_ptr<Document> mset_iterator_get_document(MSetIterator &iter);
bool mset_iterator_eq(MSetIterator &iter, MSetIterator &other);
void mset_iterator_next (MSetIterator &iter);

//
std::unique_ptr<MultiValueKeyMaker> new_multi_value_key_maker ();
void add_value_to_multi_value_key_maker(MultiValueKeyMaker &this_m, valueno slot, bool asc_desc);

std::unique_ptr<ValueCountMatchSpy> new_value_count_match_spy (valueno slot);

std::unique_ptr<RangeProcessor> new_range_processor (valueno slot, rust::Str str, int32_t flags);
std::unique_ptr<NumberRangeProcessor> new_number_range_processor (valueno slot, rust::Str prefix, int32_t flags);

//
std::unique_ptr<TermIterator> value_count_matchspy_values_begin(ValueCountMatchSpy &vcms);
std::unique_ptr<TermIterator> value_count_matchspy_values_end(ValueCountMatchSpy &vcms);
int value_count_matchspy_get_total(ValueCountMatchSpy &vcms);

//
rust::String term_iterator_get_termfreq_value(TermIterator &titer);
int term_iterator_get_termfreq_freq(TermIterator &titer);
bool term_iterator_eq(TermIterator &titer, TermIterator &other);
void term_iterator_next(TermIterator &titer);