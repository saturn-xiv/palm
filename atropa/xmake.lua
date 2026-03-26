add_rules("mode.debug", "mode.release")

target("atropa")
    set_kind("binary")
    add_files("src/*.cpp")
