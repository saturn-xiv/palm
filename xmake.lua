add_rules("mode.debug", "mode.release")

add_requires("boost", {system = false})
add_requires(
    "spdlog", "argparse", "toml++",
    "mailio", "cpr", "yaml-cpp", "cppcodec", "jwt-cpp",
    "pahomqttcpp",
    "cpp-httplib", "inja", "nlohmann_json",
    "redis-plus-plus",
    "grpc", "protobuf-cpp"
)
-- thrift
-- add_requires("vcpkg:mongo-cxx-driver", "vcpkg:minio-cpp", "vcpkg:librabbitmq")
-- add_requires("soci", {configs = {postgresql = true, mysql = true, sqlite3 = true,  boost = true}})

target("lavender")
    set_kind("binary")
    set_languages("c11", "c++20")

    add_packages(
        "spdlog", "argparse", "toml++",
        "mailio", "cpr", "yaml-cpp", "cppcodec",
        "pahomqttcpp",
        "cpp-httplib", "inja", "nlohmann_json",
        "redis-plus-plus", 
        -- "soci",
        "grpc", "protobuf-cpp"
        -- "vcpkg:mongo-cxx-driver", "vcpkg:minio-cpp", "vcpkg:librabbitmq"
    )

    add_files("lavender/src/*.cpp", "gourd/src/*.cpp")
    add_includedirs("gourd/include", "lavender/include")
    add_ldflags("-static-libgcc", "-static-libstdc++")

target("bamboo")
    set_kind("binary")
    set_languages("c11", "c++20")

    add_packages(
        "spdlog", "argparse", "toml++",
        "jwt-cpp", "cppcodec",
        "cpp-httplib", "inja", "nlohmann_json",
        -- "soci",
        "grpc", "protobuf-cpp"
    )

    add_files("bamboo/src/*.cpp", "gourd/src/*.cpp")
    add_includedirs("gourd/include", "bamboo/include")
    add_ldflags("-static-libgcc", "-static-libstdc++")

