add_rules("mode.debug", "mode.release")


package("poco")
    set_homepage("https://pocoproject.org/")
    set_description("POCO (Portable Components) C++ Libraries")
    set_urls("https://github.com/pocoproject/poco/releases/download/poco-$(version)-release/poco-$(version).tar.bz2")
    add_versions("1.15.0", "daeecda0d99d2fdc155b3bdb3f05e428dc55d0598a0c7b53985ab041c6aa7837")
    on_install(function (package)
        local configs = {}
        table.insert(configs, "-DCMAKE_BUILD_TYPE=" .. (package:debug() and "Debug" or "Release"))
        import("package.tools.cmake").install(package, configs)
    end)
package_end()

package("grpc")
    set_homepage("https://grpc.io/")
    set_description("A high performance, open source universal RPC framework")
    
    -- set_urls("https://github.com/grpc/grpc/archive/refs/tags/v$(version).tar.gz")
    -- add_versions("1.78.0", "e2ace790a5f2d0f83259d1390a816a33b013ea34df2e86084d927e58daa4c5d9")

    add_urls("https://github.com/grpc/grpc.git")
    add_versions("1.78.0", "5e6ba94242b92e363220bc2163d55ce3554d4ecc")
    
    on_install(function (package)
        local configs = {}
        table.insert(configs, "-DCMAKE_BUILD_TYPE=" .. (package:debug() and "Debug" or "Release"))
        table.insert(configs, "-DABSL_PROPAGATE_CXX_STD=ON")        
        table.insert(configs, "-DgRPC_SSL_PROVIDER=package")
        table.insert(configs, "-DgRPC_BUILD_TESTS=OFF")
        import("package.tools.cmake").install(package, configs)
    end)
package_end()

add_requires("poco", "grpc")

target("tulip")
    set_kind("binary")
    set_languages("c++20")    
    add_files("protocols/src/*.cc", "src/*.cpp")    
    add_includedirs("protocols/include", "include")
    add_packages("poco", "grpc")
    add_syslinks("m", "dl", "anl", "pthread")
    add_links("mysqlclient", "pq")


