//
// Created by ttys3 on 6/9/23.
//

#ifndef XAPIAN_RS_EASY_WRAPPER_H
#define XAPIAN_RS_EASY_WRAPPER_H


#pragma once
#include "cxx.h"
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

void writable_database_close (WritableDatabase &db);

#endif //XAPIAN_RS_EASY_WRAPPER_H
