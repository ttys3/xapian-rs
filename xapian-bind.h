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
const std::string &get_doc_data (Document &doc);

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
std::unique_ptr<Query> new_query(int8_t &err);
std::unique_ptr<Query> new_query_range(int32_t op, valueno slot, double begin, double end, int8_t &err);
std::unique_ptr<Query> new_query_double_with_prefix(rust::Str prefix, double _d, int8_t &err);
std::unique_ptr<Query> add_right_query(Query &this_q, int32_t _op, Query &q, int8_t &err);
bool query_is_empty (Query &q, int8_t &err);
const std::string &get_description (Query &q);

//
std::unique_ptr<MSet> get_mset(Enquire &en, int32_t from, int32_t size, int8_t &err);
void set_query(Enquire &en, Query &query, int8_t &err);
void set_sort_by_key(Enquire &en, MultiValueKeyMaker & sorter, bool reverse, int8_t &err);
void add_matchspy_value_count(Enquire &en, ValueCountMatchSpy &vcms, int8_t &err);
//
int get_matches_estimated (MSet &set, int8_t &err);
int mset_size (MSet &set, int8_t &err);
const std::string &mset_snippet(MSet &set, rust::Str text, int32_t length, Stem &stem, int32_t flags, rust::Str hi_start,rust::Str hi_end, rust::Str omit,int8_t &err);
std::unique_ptr<MSetIterator> mset_begin (MSet &set, int8_t &err);
std::unique_ptr<MSetIterator> mset_end (MSet &set, int8_t &err);
std::unique_ptr<MSetIterator> mset_back (MSet &set, int8_t &err);

//
std::unique_ptr<Document> mset_iterator_get_document(MSetIterator &iter, int8_t &err);
bool mset_iterator_eq(MSetIterator &iter, MSetIterator &other, int8_t &err);
void mset_iterator_next (MSetIterator &iter, int8_t &err);

//
std::unique_ptr<MultiValueKeyMaker> new_multi_value_key_maker (int8_t &err);
void add_value_to_multi_value_key_maker(MultiValueKeyMaker &this_m, valueno slot, bool asc_desc, int8_t &err);

std::unique_ptr<ValueCountMatchSpy> new_value_count_match_spy (valueno slot, int8_t &err);

std::unique_ptr<RangeProcessor> new_range_processor (valueno slot, rust::Str str, int32_t flags, int8_t &err);
std::unique_ptr<NumberRangeProcessor> new_number_range_processor (valueno slot, rust::Str prefix, int32_t flags, int8_t &err);

//
std::unique_ptr<TermIterator> value_count_matchspy_values_begin(ValueCountMatchSpy &vcms, int8_t &err);
std::unique_ptr<TermIterator> value_count_matchspy_values_end(ValueCountMatchSpy &vcms, int8_t &err);
int value_count_matchspy_get_total(ValueCountMatchSpy &vcms, int8_t &err);

//
const std::string &term_iterator_get_termfreq_value(TermIterator &titer, int8_t &err);
int term_iterator_get_termfreq_freq(TermIterator &titer, int8_t &err);
bool term_iterator_eq(TermIterator &titer, TermIterator &other, int8_t &err);
void term_iterator_next(TermIterator &titer, int8_t &err);