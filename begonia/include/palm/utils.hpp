#pragma once

#include <cpr/cpr.h>
#include <httplib.h>
#include <spdlog/spdlog.h>
#include <cppcodec/base32_crockford.hpp>
#include <cppcodec/base64_rfc4648.hpp>
#include <cppcodec/base64_url.hpp>
#include <inja/inja.hpp>
#include <nlohmann/json.hpp>

namespace palm {
void init(bool debug = false);
}
