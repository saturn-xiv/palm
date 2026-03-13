add_rules("mode.debug", "mode.release")
set_version("2026.3.13")
add_requires("argparse", "toml++", "cpr", "nlohmann_json")
-- add_requires("boost", {system = false})
add_requires("boost", {configs = {components = "thread log log_setup"}})
add_defines("BOOST_LOG_DYN_LINK", "BOOST_ALL_DYN_LINK")


target("lavender")
    set_languages("c++23")
    set_kind("binary")
    set_configdir("$(builddir)/$(plat)/$(arch)/$(mode)")
    add_configfiles("include/lavender/version.h.in", {filename = "include/lavender/version.hpp"})
    add_files("src/*.cpp")
    add_includedirs("include", "$(builddir)/$(plat)/$(arch)/$(mode)/include")    
    add_packages("argparse", "toml++", "boost", "cpr", "nlohmann_json")
    
