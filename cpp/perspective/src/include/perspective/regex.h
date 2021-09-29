/******************************************************************************
 *
 * Copyright (c) 2019, the Perspective Authors.
 *
 * This file is part of the Perspective library, distributed under the terms of
 * the Apache License 2.0.  The full license can be found in the LICENSE file.
 *
 */

#pragma once

#include <perspective/first.h>
#include <perspective/base.h>
#include <perspective/exports.h>
#include <perspective/raw_types.h>
#include <perspective/scalar.h>
#include <boost/regex.hpp>
#include <chrono>

#ifdef PSP_ENABLE_WASM
#include <codecvt>
#include <emscripten.h>
#include <emscripten/val.h>
typedef emscripten::val t_val;
typedef std::codecvt_utf8<wchar_t> utf8convert_type;
typedef std::codecvt_utf8_utf16<wchar_t> utf16convert_type;
#elif defined PSP_ENABLE_PYTHON
#include <pybind11/pybind11.h>
typedef py::object t_val;
#endif

namespace perspective {

struct t_regex_find_result {
    t_index m_sidx;
    t_index m_eidx;
};

class PERSPECTIVE_EXPORT t_regex {
public:
    static bool match(const std::string& str, const std::shared_ptr<boost::regex>& pattern);
    static bool find(const std::string& str, const std::string& pattern,
        t_regex_find_result& result);
    static bool split(const std::string& str, const std::string& pattern,
        std::vector<std::string>& result);
};

} // end namespace perspective
