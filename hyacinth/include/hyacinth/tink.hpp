#pragma once

#include <bits/stdc++.h>
#include <sys/utsname.h>
#include <unistd.h>
#include <algorithm>
#include <chrono>
#include <climits>
#include <cstdint>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <exception>
#include <filesystem>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <random>
#include <ranges>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <typeinfo>
#include <unordered_map>
#include <utility>
#include <variant>
#include <vector>

// TODO
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#include <tink/internal/ssl_unique_ptr.h>
#pragma GCC diagnostic pop

#include <tink/aead/aead_key_templates.h>
#include <tink/jwt/jwt_mac.h>
#include <tink/keyset_handle.h>
#include <tink/mac.h>
#include <tink/mac/mac_key_templates.h>

#include <spdlog/spdlog.h>
