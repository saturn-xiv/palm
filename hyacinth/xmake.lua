add_rules("mode.debug", "mode.release")
add_requires("spdlog")

package("tink")
    set_homepage("https://github.com/tink-crypto/tink-cc")
    set_description("C++ implementation of Tink")
    set_urls("https://github.com/tink-crypto/tink-cc.git")
    -- add_versions("v2.3.0")

    on_install(function (package)
  	-- TODO
    end)

target("hyacinth")
    set_kind("shared")
    set_languages("c++20")
    add_includedirs("include")
    add_files("src/*.cpp")
    add_packages("spdlog", "tink")

