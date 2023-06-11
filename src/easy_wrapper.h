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

using namespace Xapian;

rust::Str version_string();

void writable_database_close (WritableDatabase &db);

#endif //XAPIAN_RS_EASY_WRAPPER_H
