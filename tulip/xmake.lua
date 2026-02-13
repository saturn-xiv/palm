add_rules("mode.debug", "mode.release")

-- add_requires("openssl", {system = true})
-- add_requires("pcre2")
-- add_requires("mysql", {system = true})
-- add_requires("postgresql", {system = true})
add_requires("poco", {configs = {crypto = true, json = true, jwt = true, utils = true, net = true, data = true, mysql = true, postgresql = true, redis = true}})
add_requires("grpc")

-- package("poco")
--     set_homepage("https://pocoproject.org/")
--     set_description("POCO (Portable Components) C++ Libraries")
--     set_urls("https://github.com/pocoproject/poco/releases/download/poco-$(version)-release/poco-$(version).tar.bz2")
--     add_versions("1.15.0", "daeecda0d99d2fdc155b3bdb3f05e428dc55d0598a0c7b53985ab041c6aa7837")
    
--     set_policy("package.install_always", true)

--     -- on_load(function (package)
--     --     package:add("deps", "openssl")
--     --     package:add("deps", "mysql")
--     --     package:add("deps", "postgresql")
--     -- end)

--     add_deps("cmake", "expat", "zlib", "openssl", "mysql", "postgresql")
    
--     on_install(function (package)        
--         local configs = {}        
--         table.insert(configs, "-DCMAKE_BUILD_TYPE=" .. (package:debug() and "Debug" or "Release"))

--         -- https://docs.pocoproject.org/current/00200-GettingStarted.html
--         table.insert(configs, "-DENABLE_DATA=ON")
--         table.insert(configs, "-DENABLE_DATA_MYSQL=ON")
--         table.insert(configs, "-DENABLE_DATA_POSTGRESQL=ON")
--         table.insert(configs, "-DENABLE_REDIS=ON")
--         table.insert(configs, "-DENABLE_CRYPTO=ON")
--         table.insert(configs, "-DENABLE_JWT=ON")
--         table.insert(configs, "-DENABLE_UTIL=ON")

--         import("package.tools.cmake").install(package, configs)
--     end)
-- package_end()

-- package("grpc")
--     set_homepage("https://grpc.io/")
--     set_description("A high performance, open source universal RPC framework")
    
--     -- set_urls("https://github.com/grpc/grpc/archive/refs/tags/v$(version).tar.gz")
--     -- add_versions("1.78.0", "e2ace790a5f2d0f83259d1390a816a33b013ea34df2e86084d927e58daa4c5d9")

--     add_urls("https://github.com/grpc/grpc.git")
--     add_versions("1.78.0", "5e6ba94242b92e363220bc2163d55ce3554d4ecc")
   
--     on_install(function (package)
--         local configs = {}
--         table.insert(configs, "-DCMAKE_BUILD_TYPE=" .. (package:debug() and "Debug" or "Release"))
--         -- https://github.com/protocolbuffers/protobuf/blob/main/cmake/README.md#c-version
--         table.insert(configs, "-DCMAKE_CXX_STANDARD=17")
--         table.insert(configs, "-DABSL_PROPAGATE_CXX_STD=ON")        
--         table.insert(configs, "-DgRPC_SSL_PROVIDER=package")
--         table.insert(configs, "-DgRPC_BUILD_TESTS=OFF")
        
--         import("package.tools.cmake").install(package, configs)
--     end)
-- package_end()

-- -- "poco",
-- add_requires( "grpc")

target("tulip")
    set_kind("binary")
    set_languages("c++20")    
    add_files("protocols/src/*.cc", "src/*.cpp")    
    add_includedirs("protocols/include", "include")
    -- "openssl", "mysql", "postgresql",
    add_packages("poco", "grpc")
    add_syslinks("m", "dl", "anl", "pthread")
    add_links("rabbitmq", "sodium")

