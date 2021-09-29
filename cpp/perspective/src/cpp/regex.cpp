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
t_regex::match(const std::string& str, const std::string& pattern) {
#ifdef PSP_ENABLE_WASM
    // TODO do we need to convert here?
    std::wstring_convert<utf8convert_type, wchar_t> converter("", L"");
    t_val string_val = t_val(converter.from_bytes(str));

    // TODO: don't construct this every time, cache it per-matcher
    t_val p = t_val::global("RegExp").new_(pattern);
    bool match_result = string_val.call<bool>("match", p);
    return match_result;
#else
    boost::regex p(pattern);
    return boost::regex_match(str, p);
    // return RE2::FullMatch(str, pattern);
#endif
}

} // end namespace perspective