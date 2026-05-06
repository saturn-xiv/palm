add_rules("mode.debug", "mode.release")

add_requires("thrift 0.22.0", "argparse", "spdlog", "inja", "nlohmann_json")

package("tink")
    set_homepage("https://github.com/tink-crypto/tink-cc")
    set_description("C++ implementation of Tink")
    add_urls("https://github.com/tink-crypto/tink-cc.git")
    add_versions("v2.6.0", "f47507e")

    add_deps("cmake")
    on_install(function (package)
        local configs = {}
        import("package.tools.cmake").install(package, configs)
    end)


target("loquat")
    set_languages("c99", "c++20")
    set_kind("binary")
    add_includedirs("include")
    add_files("src/*.cpp")
    add_packages("thrift", "argparse", "spdlog", "inja", "nlohmann_json", "tink")

