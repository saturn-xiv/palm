add_rules("mode.debug", "mode.release")

target("lavender")
    set_kind("binary")
    set_languages("c++20")
    add_files("src/*.cpp")
    add_includedirs("include")

