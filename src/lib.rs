pub mod enums;

use autocxx::prelude::*;

// https://google.github.io/autocxx/other_features.html#exceptions
// Exceptions are not supported. If your C++ code is compiled with exceptions,
// you can expect serious runtime explosions.
// The underlying cxx crate has exception support, so it would be possible to add them.

include_cpp! {
    #include "xapian.h"
    safety!(unsafe_ffi)

    // Xapian::version_string
    generate!("Xapian::version_string")
    generate!("Xapian::DB_CREATE_OR_OPEN")
    generate!("Xapian::DB_BACKEND_HONEY")

    // Error
    generate!("Xapian::Error")

    // 1.5 only
    // LogicError
    generate!("Xapian::LogicError")
    // RuntimeError
    generate!("Xapian::RuntimeError")

    // 1.4 only ErrorHandler
    // generate!("Xapian::ErrorHandler")

    // // Auto
    // generate!("Xapian::Auto")
    // PositionIterator
    generate!("Xapian::PositionIterator")
    // PostingIterator
    generate!("Xapian::PostingIterator")
    generate!("Xapian::doccount")
    generate!("Xapian::TermGenerator")
    // RSet
    generate!("Xapian::RSet")

    // ESet
    generate!("Xapian::ESet")
    // ESetIterator
    generate!("Xapian::ESetIterator")
    generate!("Xapian::MSet")
    // MSetIterator
    generate!("Xapian::MSetIterator")

    // https://google.github.io/autocxx/workflow.html#my-build-entirely-failed
    // autocxx does not allow instantiation of abstract types5 (aka types with pure virtual methods).
    // Virtual base class for expand decider functor
    block!("Xapian::ExpandDecider")
    // ExpandDeciderFilterTerms
    // ExpandDeciderAnd
    generate!("Xapian::ExpandDeciderAnd")
    generate!("Xapian::ExpandDeciderFilterTerms")
    // ExpandDeciderFilterPrefix
    generate!("Xapian::ExpandDeciderFilterPrefix")

    block!("Xapian::KeyMaker")
    // MultiValueKeyMaker
    generate!("Xapian::MultiValueKeyMaker")
    // LatLongDistanceKeyMaker
    generate!("Xapian::LatLongDistanceKeyMaker")


    block!("Xapian::MatchSpy")
    // ValueCountMatchSpy
    generate!("Xapian::ValueCountMatchSpy")

    // TermIterator
    generate!("Xapian::TermIterator")

    // PostingSource
    generate!("Xapian::PostingSource")
    // ValuePostingSource
    generate!("Xapian::ValuePostingSource")
    // ValueWeightPostingSource
    generate!("Xapian::ValueWeightPostingSource")
    // DecreasingValueWeightPostingSource
    generate!("Xapian::DecreasingValueWeightPostingSource")
    // ValueMapPostingSource
    generate!("Xapian::ValueMapPostingSource")

    // Query
    generate!("Xapian::Query")

    block!("Xapian::Stopper")
    // SimpleStopper
    generate!("Xapian::SimpleStopper")

    block!("Xapian::ValueRangeProcessor")
    // RangeProcessor
    generate!("Xapian::RangeProcessor")
    // DateRangeProcessor
    generate!("Xapian::DateRangeProcessor")
    // NumberRangeProcessor
    generate!("Xapian::NumberRangeProcessor")
    // generate!("Xapian::UnitRangeProcessor")

    //StemImplementation
    generate!("Xapian::StemImplementation")
    generate!("Xapian::Stem")
    generate!("Xapian::QueryParser")

    // MatchDecider
    block!("Xapian::MatchDecider")
    // ValueSetMatchDecider
    generate!("Xapian::ValueSetMatchDecider")

    // Weight
    generate!("Xapian::Weight")
    // BoolWeight
    generate!("Xapian::BoolWeight")
    // TfIdfWeight
    generate!("Xapian::TfIdfWeight")
    // BM25Weight
    generate!("Xapian::BM25Weight")
    // BM25PlusWeight
    generate!("Xapian::BM25PlusWeight")
    // TradWeight
    generate!("Xapian::TradWeight")
    // InL2Weight
    generate!("Xapian::InL2Weight")
    // IfB2Weight
    generate!("Xapian::IfB2Weight")
    // IneB2Weight
    generate!("Xapian::IneB2Weight")
    // BB2Weight
    generate!("Xapian::BB2Weight")
    // DLHWeight
    generate!("Xapian::DLHWeight")
    // PL2Weight
    generate!("Xapian::PL2Weight")
    // PL2PlusWeight
    generate!("Xapian::PL2PlusWeight")
    // DPHWeight
    generate!("Xapian::DPHWeight")
    // LMWeight
    generate!("Xapian::LMWeight")
    // CoordWeight
    generate!("Xapian::CoordWeight")

    generate!("Xapian::Database")
    generate!("Xapian::WritableDatabase")
    generate!("Xapian::Document")
    // PositionIterator
    generate!("Xapian::PostingIterator")

    generate!("Xapian::Enquire")
    block!("Xapian::FieldProcessor")

    //Registry
    generate!("Xapian::Registry")
    // LatLongMetric
    generate!("Xapian::LatLongMetric")
    // LatLongCoord
    generate!("Xapian::LatLongCoord")
    // LatLongCoordsIterator
    generate!("Xapian::LatLongCoordsIterator")
    // LatLongCoords
    generate!("Xapian::LatLongCoords")
    // GreatCircleMetric
    generate!("Xapian::GreatCircleMetric")
    // LatLongDistancePostingSource
    generate!("Xapian::LatLongDistancePostingSource")

    // Utf8Iterator
    generate!("Xapian::Utf8Iterator")

    // Compactor
    generate!("Xapian::Compactor")
}

// write some tests
#[cfg(test)]
mod test {
    use super::*;
    use crate::ffi::Xapian::WritableDatabase;
    use crate::ffi::Xapian::{DB_CREATE_OR_OPEN, DB_BACKEND_HONEY};

    #[test]
    fn test_xapian() {
        println!("xapian lib version: {:?}", crate::ffi::Xapian::version_string());
        // https://xapian.org/docs/sourcedoc/html/namespaceXapian_1_1Chert.html#ad328887e1b0e513dff7f50f62a645a40
        let _ = std::fs::create_dir_all("./data");
        // Honey backend doesn't support updating existing databases
        cxx::let_cxx_string!(path = "./data/xapian-hello");
        let mut db = WritableDatabase::new1(&path, c_int(DB_CREATE_OR_OPEN), c_int(DB_BACKEND_HONEY)).within_unique_ptr();

        println!("open WritableDatabase ok");
        db.pin_mut().commit();
        db.pin_mut().close();
    }
}