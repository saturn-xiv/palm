add_rules("mode.debug", "mode.release")

add_requires(    
    "argparse", "spdlog",
    "boost", "libsodium", "libevent", "mailio",
    "amqp-cpp", "pahomqttcpp", "thrift", "grpc",
    "toml++", "nlohmann_json", "cppcodec",
    "inja",
    "soci", "libpqxx", "mysqlpp", "sqlitecpp"
)

target("loquat")
    set_kind("binary")
    add_files("src/*.cpp")
    set_languages("c11", "c++17")
    add_packages(        
        "argparse", "spdlog",
        "boost", "libsodium", "libevent", "mailio",
        "amqp-cpp", "pahomqttcpp", "thrift", "grpc",
        "toml++", "nlohmann_json", "cppcodec",
        "inja",
        "soci", "libpqxx","mysqlpp", "sqlitecpp"  
    )

