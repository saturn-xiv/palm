add_rules("mode.debug", "mode.release")

add_requires("thrift 0.22.0", "icu4c", "argparse", "spdlog", "inja", "nlohmann_json")

package("tink")
    set_homepage("https://github.com/tink-crypto/tink-cc")
    set_description("C++ implementation of Tink")
    add_urls("https://github.com/tink-crypto/tink-cc.git")
    add_versions("v2.6.0", "f47507eb5f4f5ac3f446270e10df196f02c5c007")

    add_deps("cmake")    
    on_install(function (package)
        local configs = {}
        -- table.insert(configs, "-DABSL_PROPAGATE_CXX_STD=ON")
        -- table.insert(configs, "-DTINK_USE_SYSTEM_OPENSSL=ON")
        -- table.insert(configs, "-DTINK_BUILD_TESTS=OFF")
        import("package.tools.cmake").install(package, configs)
        os.cp("tink", package:installdir("include"))
    end)    
package_end()

add_requires("tink")

target("loquat")
    set_languages("c99", "c++20")
    set_kind("binary")
    add_includedirs("include")
    add_files("src/*.cpp")
    add_packages("thrift", "argparse", "spdlog", "inja", "nlohmann_json", "tink")

