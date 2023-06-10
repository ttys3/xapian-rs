//
// Created by ttys3 on 6/9/23.
//

#include "easy_wrapper.h"

#include <iostream>

#include <xapian.h>
#include <string>
#include <string.h>

using namespace Xapian;

rust::Str version_string()
{
    return Xapian::version_string();
}

void writable_database_close (WritableDatabase &db) {
    db.close();
}