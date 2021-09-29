/******************************************************************************
 *
 * Copyright (c) 2019, the Perspective Authors.
 *
 * This file is part of the Perspective library, distributed under the terms of
 * the Apache License 2.0.  The full license can be found in the LICENSE file.
 *
 */

#include <perspective/regex.h>

namespace perspective {

bool
t_regex::match(const std::string& str, const std::shared_ptr<boost::regex>& pattern) {
    return boost::regex_match(str, *pattern);
}

} // end namespace perspective