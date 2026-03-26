add_rules("mode.debug", "mode.release")
set_version("2026.3.26", {build = "%a %b %d %I:%M:%S %p %Z %Y"})  

add_requires(
    "argparse", "toml++", "spdlog",
    "libsodium", "cryptopp",
    "cpr", "cppcodec", "pugixml", "mailio",
    "cpp-httplib", "inja", "nlohmann_json"
)
add_requires("boost", {system = false, configs = {cmake = false, header_only=true, date_time=true, url=true, exception=true}})
add_requires("soci", {configs = {postgresql=true, boost=true}})
-- add_requires("vcpkg::redis-plus-plus", {configs = {features = {"cxx17"}}})
-- add_requires("vcpkg::librabbitmq", "vcpkg::grpc")

target("atropa")
    set_languages("c++23")
    set_kind("binary")
    add_ldflags("-static -static-libgcc -static-libstdc++")
    set_configdir("$(builddir)/$(plat)/$(arch)/$(mode)")
    add_configfiles("include/palm/version.h.in", {filename = "include/palm/version.hpp"})
    add_files("src/*.cpp")
    add_includedirs("include", "$(builddir)/$(plat)/$(arch)/$(mode)/include")    
    add_packages(
        "boost", "soci",
        -- "vcpkg::redis-plus-plus", "vcpkg::librabbitmq",
        "argparse", "toml++", "spdlog",
        "libsodium", "cryptopp",
        "cpr", "cppcodec", "pugixml", "mailio",
        "cpp-httplib", "inja", "nlohmann_json"        
    )
