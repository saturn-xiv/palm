add_rules("mode.debug", "mode.release")

add_requires(
    "spdlog", "taywee_args", "boost",
    "cppcodec", "toml++", "tinyxml2", "nlohmann_json", "inja", "cpp-httplib", 
    "libpqxx", "redis-plus-plus", "pahomqttcpp", "minio-cpp",
    "mailio", "cpr",
    "grpc"
)
-- add_requires("protobuf-cpp", {configs = {cmake = false}})


target("lavender")
    set_kind("binary")
    set_languages("c17", "c++20")
    add_files("src/*.cpp")
    add_includedirs("include")
    add_packages(
        "spdlog", "taywee_args", "boost",
        "cppcodec", "toml++", "tinyxml2", "nlohmann_json", "inja", "cpp-httplib", 
        "libpqxx", "libpqxx", "redis-plus-plus", "pahomqttcpp", "minio-cpp",
        "mailio", "cpr",
        -- "grpc"
    )

