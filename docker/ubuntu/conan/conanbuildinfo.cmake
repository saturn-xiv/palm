include(CMakeParseArguments)

macro(conan_find_apple_frameworks FRAMEWORKS_FOUND FRAMEWORKS SUFFIX BUILD_TYPE)
    if(APPLE)
        if(CMAKE_BUILD_TYPE)
            set(_BTYPE ${CMAKE_BUILD_TYPE})
        elseif(NOT BUILD_TYPE STREQUAL "")
            set(_BTYPE ${BUILD_TYPE})
        endif()
        if(_BTYPE)
            if(${_BTYPE} MATCHES "Debug|_DEBUG")
                set(CONAN_FRAMEWORKS${SUFFIX} ${CONAN_FRAMEWORKS${SUFFIX}_DEBUG} ${CONAN_FRAMEWORKS${SUFFIX}})
                set(CONAN_FRAMEWORK_DIRS${SUFFIX} ${CONAN_FRAMEWORK_DIRS${SUFFIX}_DEBUG} ${CONAN_FRAMEWORK_DIRS${SUFFIX}})
            elseif(${_BTYPE} MATCHES "Release|_RELEASE")
                set(CONAN_FRAMEWORKS${SUFFIX} ${CONAN_FRAMEWORKS${SUFFIX}_RELEASE} ${CONAN_FRAMEWORKS${SUFFIX}})
                set(CONAN_FRAMEWORK_DIRS${SUFFIX} ${CONAN_FRAMEWORK_DIRS${SUFFIX}_RELEASE} ${CONAN_FRAMEWORK_DIRS${SUFFIX}})
            elseif(${_BTYPE} MATCHES "RelWithDebInfo|_RELWITHDEBINFO")
                set(CONAN_FRAMEWORKS${SUFFIX} ${CONAN_FRAMEWORKS${SUFFIX}_RELWITHDEBINFO} ${CONAN_FRAMEWORKS${SUFFIX}})
                set(CONAN_FRAMEWORK_DIRS${SUFFIX} ${CONAN_FRAMEWORK_DIRS${SUFFIX}_RELWITHDEBINFO} ${CONAN_FRAMEWORK_DIRS${SUFFIX}})
            elseif(${_BTYPE} MATCHES "MinSizeRel|_MINSIZEREL")
                set(CONAN_FRAMEWORKS${SUFFIX} ${CONAN_FRAMEWORKS${SUFFIX}_MINSIZEREL} ${CONAN_FRAMEWORKS${SUFFIX}})
                set(CONAN_FRAMEWORK_DIRS${SUFFIX} ${CONAN_FRAMEWORK_DIRS${SUFFIX}_MINSIZEREL} ${CONAN_FRAMEWORK_DIRS${SUFFIX}})
            endif()
        endif()
        foreach(_FRAMEWORK ${FRAMEWORKS})
            # https://cmake.org/pipermail/cmake-developers/2017-August/030199.html
            find_library(CONAN_FRAMEWORK_${_FRAMEWORK}_FOUND NAME ${_FRAMEWORK} PATHS ${CONAN_FRAMEWORK_DIRS${SUFFIX}} CMAKE_FIND_ROOT_PATH_BOTH)
            if(CONAN_FRAMEWORK_${_FRAMEWORK}_FOUND)
                list(APPEND ${FRAMEWORKS_FOUND} ${CONAN_FRAMEWORK_${_FRAMEWORK}_FOUND})
            else()
                message(FATAL_ERROR "Framework library ${_FRAMEWORK} not found in paths: ${CONAN_FRAMEWORK_DIRS${SUFFIX}}")
            endif()
        endforeach()
    endif()
endmacro()


#################
###  SOCI
#################
set(CONAN_SOCI_ROOT "/home/jeremy/.conan/data/soci/4.0.2/_/_/package/8e517ad41504b15895a219029f4ae6b5963c4c39")
set(CONAN_INCLUDE_DIRS_SOCI "/home/jeremy/.conan/data/soci/4.0.2/_/_/package/8e517ad41504b15895a219029f4ae6b5963c4c39/include")
set(CONAN_LIB_DIRS_SOCI "/home/jeremy/.conan/data/soci/4.0.2/_/_/package/8e517ad41504b15895a219029f4ae6b5963c4c39/lib")
set(CONAN_BIN_DIRS_SOCI )
set(CONAN_RES_DIRS_SOCI )
set(CONAN_SRC_DIRS_SOCI )
set(CONAN_BUILD_DIRS_SOCI "/home/jeremy/.conan/data/soci/4.0.2/_/_/package/8e517ad41504b15895a219029f4ae6b5963c4c39/")
set(CONAN_FRAMEWORK_DIRS_SOCI )
set(CONAN_LIBS_SOCI soci_empty soci_sqlite3 soci_postgresql soci_core)
set(CONAN_PKG_LIBS_SOCI soci_empty soci_sqlite3 soci_postgresql soci_core)
set(CONAN_SYSTEM_LIBS_SOCI )
set(CONAN_FRAMEWORKS_SOCI )
set(CONAN_FRAMEWORKS_FOUND_SOCI "")  # Will be filled later
set(CONAN_DEFINES_SOCI )
set(CONAN_BUILD_MODULES_PATHS_SOCI )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_SOCI )

set(CONAN_C_FLAGS_SOCI "")
set(CONAN_CXX_FLAGS_SOCI "")
set(CONAN_SHARED_LINKER_FLAGS_SOCI "")
set(CONAN_EXE_LINKER_FLAGS_SOCI "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_SOCI_LIST "")
set(CONAN_CXX_FLAGS_SOCI_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_SOCI_LIST "")
set(CONAN_EXE_LINKER_FLAGS_SOCI_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_SOCI "${CONAN_FRAMEWORKS_SOCI}" "_SOCI" "")
# Append to aggregated values variable
set(CONAN_LIBS_SOCI ${CONAN_PKG_LIBS_SOCI} ${CONAN_SYSTEM_LIBS_SOCI} ${CONAN_FRAMEWORKS_FOUND_SOCI})


#################
###  HIREDIS
#################
set(CONAN_HIREDIS_ROOT "/home/jeremy/.conan/data/hiredis/1.0.2/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7")
set(CONAN_INCLUDE_DIRS_HIREDIS "/home/jeremy/.conan/data/hiredis/1.0.2/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include")
set(CONAN_LIB_DIRS_HIREDIS "/home/jeremy/.conan/data/hiredis/1.0.2/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib")
set(CONAN_BIN_DIRS_HIREDIS )
set(CONAN_RES_DIRS_HIREDIS )
set(CONAN_SRC_DIRS_HIREDIS )
set(CONAN_BUILD_DIRS_HIREDIS "/home/jeremy/.conan/data/hiredis/1.0.2/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/")
set(CONAN_FRAMEWORK_DIRS_HIREDIS )
set(CONAN_LIBS_HIREDIS hiredis)
set(CONAN_PKG_LIBS_HIREDIS hiredis)
set(CONAN_SYSTEM_LIBS_HIREDIS )
set(CONAN_FRAMEWORKS_HIREDIS )
set(CONAN_FRAMEWORKS_FOUND_HIREDIS "")  # Will be filled later
set(CONAN_DEFINES_HIREDIS )
set(CONAN_BUILD_MODULES_PATHS_HIREDIS )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_HIREDIS )

set(CONAN_C_FLAGS_HIREDIS "")
set(CONAN_CXX_FLAGS_HIREDIS "")
set(CONAN_SHARED_LINKER_FLAGS_HIREDIS "")
set(CONAN_EXE_LINKER_FLAGS_HIREDIS "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_HIREDIS_LIST "")
set(CONAN_CXX_FLAGS_HIREDIS_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_HIREDIS_LIST "")
set(CONAN_EXE_LINKER_FLAGS_HIREDIS_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_HIREDIS "${CONAN_FRAMEWORKS_HIREDIS}" "_HIREDIS" "")
# Append to aggregated values variable
set(CONAN_LIBS_HIREDIS ${CONAN_PKG_LIBS_HIREDIS} ${CONAN_SYSTEM_LIBS_HIREDIS} ${CONAN_FRAMEWORKS_FOUND_HIREDIS})


#################
###  CPR
#################
set(CONAN_CPR_ROOT "/home/jeremy/.conan/data/cpr/1.6.2/_/_/package/955de0a2787b456106824a8ad3b4bc79ea0080c2")
set(CONAN_INCLUDE_DIRS_CPR "/home/jeremy/.conan/data/cpr/1.6.2/_/_/package/955de0a2787b456106824a8ad3b4bc79ea0080c2/include")
set(CONAN_LIB_DIRS_CPR "/home/jeremy/.conan/data/cpr/1.6.2/_/_/package/955de0a2787b456106824a8ad3b4bc79ea0080c2/lib")
set(CONAN_BIN_DIRS_CPR )
set(CONAN_RES_DIRS_CPR )
set(CONAN_SRC_DIRS_CPR )
set(CONAN_BUILD_DIRS_CPR "/home/jeremy/.conan/data/cpr/1.6.2/_/_/package/955de0a2787b456106824a8ad3b4bc79ea0080c2/")
set(CONAN_FRAMEWORK_DIRS_CPR )
set(CONAN_LIBS_CPR cpr)
set(CONAN_PKG_LIBS_CPR cpr)
set(CONAN_SYSTEM_LIBS_CPR )
set(CONAN_FRAMEWORKS_CPR )
set(CONAN_FRAMEWORKS_FOUND_CPR "")  # Will be filled later
set(CONAN_DEFINES_CPR )
set(CONAN_BUILD_MODULES_PATHS_CPR )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_CPR )

set(CONAN_C_FLAGS_CPR "")
set(CONAN_CXX_FLAGS_CPR "")
set(CONAN_SHARED_LINKER_FLAGS_CPR "")
set(CONAN_EXE_LINKER_FLAGS_CPR "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_CPR_LIST "")
set(CONAN_CXX_FLAGS_CPR_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_CPR_LIST "")
set(CONAN_EXE_LINKER_FLAGS_CPR_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_CPR "${CONAN_FRAMEWORKS_CPR}" "_CPR" "")
# Append to aggregated values variable
set(CONAN_LIBS_CPR ${CONAN_PKG_LIBS_CPR} ${CONAN_SYSTEM_LIBS_CPR} ${CONAN_FRAMEWORKS_FOUND_CPR})


#################
###  JWT-CPP
#################
set(CONAN_JWT-CPP_ROOT "/home/jeremy/.conan/data/jwt-cpp/0.5.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9")
set(CONAN_INCLUDE_DIRS_JWT-CPP "/home/jeremy/.conan/data/jwt-cpp/0.5.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include")
set(CONAN_LIB_DIRS_JWT-CPP )
set(CONAN_BIN_DIRS_JWT-CPP )
set(CONAN_RES_DIRS_JWT-CPP )
set(CONAN_SRC_DIRS_JWT-CPP )
set(CONAN_BUILD_DIRS_JWT-CPP "/home/jeremy/.conan/data/jwt-cpp/0.5.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/")
set(CONAN_FRAMEWORK_DIRS_JWT-CPP )
set(CONAN_LIBS_JWT-CPP )
set(CONAN_PKG_LIBS_JWT-CPP )
set(CONAN_SYSTEM_LIBS_JWT-CPP )
set(CONAN_FRAMEWORKS_JWT-CPP )
set(CONAN_FRAMEWORKS_FOUND_JWT-CPP "")  # Will be filled later
set(CONAN_DEFINES_JWT-CPP )
set(CONAN_BUILD_MODULES_PATHS_JWT-CPP )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_JWT-CPP )

set(CONAN_C_FLAGS_JWT-CPP "")
set(CONAN_CXX_FLAGS_JWT-CPP "")
set(CONAN_SHARED_LINKER_FLAGS_JWT-CPP "")
set(CONAN_EXE_LINKER_FLAGS_JWT-CPP "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_JWT-CPP_LIST "")
set(CONAN_CXX_FLAGS_JWT-CPP_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_JWT-CPP_LIST "")
set(CONAN_EXE_LINKER_FLAGS_JWT-CPP_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_JWT-CPP "${CONAN_FRAMEWORKS_JWT-CPP}" "_JWT-CPP" "")
# Append to aggregated values variable
set(CONAN_LIBS_JWT-CPP ${CONAN_PKG_LIBS_JWT-CPP} ${CONAN_SYSTEM_LIBS_JWT-CPP} ${CONAN_FRAMEWORKS_FOUND_JWT-CPP})


#################
###  CPP-HTTPLIB
#################
set(CONAN_CPP-HTTPLIB_ROOT "/home/jeremy/.conan/data/cpp-httplib/0.9.5/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9")
set(CONAN_INCLUDE_DIRS_CPP-HTTPLIB "/home/jeremy/.conan/data/cpp-httplib/0.9.5/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include"
			"/home/jeremy/.conan/data/cpp-httplib/0.9.5/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include/httplib")
set(CONAN_LIB_DIRS_CPP-HTTPLIB )
set(CONAN_BIN_DIRS_CPP-HTTPLIB )
set(CONAN_RES_DIRS_CPP-HTTPLIB )
set(CONAN_SRC_DIRS_CPP-HTTPLIB )
set(CONAN_BUILD_DIRS_CPP-HTTPLIB "/home/jeremy/.conan/data/cpp-httplib/0.9.5/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/")
set(CONAN_FRAMEWORK_DIRS_CPP-HTTPLIB )
set(CONAN_LIBS_CPP-HTTPLIB )
set(CONAN_PKG_LIBS_CPP-HTTPLIB )
set(CONAN_SYSTEM_LIBS_CPP-HTTPLIB pthread)
set(CONAN_FRAMEWORKS_CPP-HTTPLIB )
set(CONAN_FRAMEWORKS_FOUND_CPP-HTTPLIB "")  # Will be filled later
set(CONAN_DEFINES_CPP-HTTPLIB )
set(CONAN_BUILD_MODULES_PATHS_CPP-HTTPLIB )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_CPP-HTTPLIB )

set(CONAN_C_FLAGS_CPP-HTTPLIB "")
set(CONAN_CXX_FLAGS_CPP-HTTPLIB "")
set(CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB "")
set(CONAN_EXE_LINKER_FLAGS_CPP-HTTPLIB "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_CPP-HTTPLIB_LIST "")
set(CONAN_CXX_FLAGS_CPP-HTTPLIB_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_LIST "")
set(CONAN_EXE_LINKER_FLAGS_CPP-HTTPLIB_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_CPP-HTTPLIB "${CONAN_FRAMEWORKS_CPP-HTTPLIB}" "_CPP-HTTPLIB" "")
# Append to aggregated values variable
set(CONAN_LIBS_CPP-HTTPLIB ${CONAN_PKG_LIBS_CPP-HTTPLIB} ${CONAN_SYSTEM_LIBS_CPP-HTTPLIB} ${CONAN_FRAMEWORKS_FOUND_CPP-HTTPLIB})


#################
###  INJA
#################
set(CONAN_INJA_ROOT "/home/jeremy/.conan/data/inja/3.3.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9")
set(CONAN_INCLUDE_DIRS_INJA "/home/jeremy/.conan/data/inja/3.3.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include")
set(CONAN_LIB_DIRS_INJA )
set(CONAN_BIN_DIRS_INJA )
set(CONAN_RES_DIRS_INJA )
set(CONAN_SRC_DIRS_INJA )
set(CONAN_BUILD_DIRS_INJA "/home/jeremy/.conan/data/inja/3.3.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/")
set(CONAN_FRAMEWORK_DIRS_INJA )
set(CONAN_LIBS_INJA )
set(CONAN_PKG_LIBS_INJA )
set(CONAN_SYSTEM_LIBS_INJA )
set(CONAN_FRAMEWORKS_INJA )
set(CONAN_FRAMEWORKS_FOUND_INJA "")  # Will be filled later
set(CONAN_DEFINES_INJA )
set(CONAN_BUILD_MODULES_PATHS_INJA )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_INJA )

set(CONAN_C_FLAGS_INJA "")
set(CONAN_CXX_FLAGS_INJA "")
set(CONAN_SHARED_LINKER_FLAGS_INJA "")
set(CONAN_EXE_LINKER_FLAGS_INJA "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_INJA_LIST "")
set(CONAN_CXX_FLAGS_INJA_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_INJA_LIST "")
set(CONAN_EXE_LINKER_FLAGS_INJA_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_INJA "${CONAN_FRAMEWORKS_INJA}" "_INJA" "")
# Append to aggregated values variable
set(CONAN_LIBS_INJA ${CONAN_PKG_LIBS_INJA} ${CONAN_SYSTEM_LIBS_INJA} ${CONAN_FRAMEWORKS_FOUND_INJA})


#################
###  TOMLPLUSPLUS
#################
set(CONAN_TOMLPLUSPLUS_ROOT "/home/jeremy/.conan/data/tomlplusplus/2.5.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9")
set(CONAN_INCLUDE_DIRS_TOMLPLUSPLUS "/home/jeremy/.conan/data/tomlplusplus/2.5.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include")
set(CONAN_LIB_DIRS_TOMLPLUSPLUS )
set(CONAN_BIN_DIRS_TOMLPLUSPLUS )
set(CONAN_RES_DIRS_TOMLPLUSPLUS )
set(CONAN_SRC_DIRS_TOMLPLUSPLUS )
set(CONAN_BUILD_DIRS_TOMLPLUSPLUS "/home/jeremy/.conan/data/tomlplusplus/2.5.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/")
set(CONAN_FRAMEWORK_DIRS_TOMLPLUSPLUS )
set(CONAN_LIBS_TOMLPLUSPLUS )
set(CONAN_PKG_LIBS_TOMLPLUSPLUS )
set(CONAN_SYSTEM_LIBS_TOMLPLUSPLUS )
set(CONAN_FRAMEWORKS_TOMLPLUSPLUS )
set(CONAN_FRAMEWORKS_FOUND_TOMLPLUSPLUS "")  # Will be filled later
set(CONAN_DEFINES_TOMLPLUSPLUS )
set(CONAN_BUILD_MODULES_PATHS_TOMLPLUSPLUS )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_TOMLPLUSPLUS )

set(CONAN_C_FLAGS_TOMLPLUSPLUS "")
set(CONAN_CXX_FLAGS_TOMLPLUSPLUS "")
set(CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS "")
set(CONAN_EXE_LINKER_FLAGS_TOMLPLUSPLUS "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_TOMLPLUSPLUS_LIST "")
set(CONAN_CXX_FLAGS_TOMLPLUSPLUS_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_LIST "")
set(CONAN_EXE_LINKER_FLAGS_TOMLPLUSPLUS_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_TOMLPLUSPLUS "${CONAN_FRAMEWORKS_TOMLPLUSPLUS}" "_TOMLPLUSPLUS" "")
# Append to aggregated values variable
set(CONAN_LIBS_TOMLPLUSPLUS ${CONAN_PKG_LIBS_TOMLPLUSPLUS} ${CONAN_SYSTEM_LIBS_TOMLPLUSPLUS} ${CONAN_FRAMEWORKS_FOUND_TOMLPLUSPLUS})


#################
###  YAML-CPP
#################
set(CONAN_YAML-CPP_ROOT "/home/jeremy/.conan/data/yaml-cpp/0.7.0/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5")
set(CONAN_INCLUDE_DIRS_YAML-CPP "/home/jeremy/.conan/data/yaml-cpp/0.7.0/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include")
set(CONAN_LIB_DIRS_YAML-CPP "/home/jeremy/.conan/data/yaml-cpp/0.7.0/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib")
set(CONAN_BIN_DIRS_YAML-CPP )
set(CONAN_RES_DIRS_YAML-CPP )
set(CONAN_SRC_DIRS_YAML-CPP )
set(CONAN_BUILD_DIRS_YAML-CPP "/home/jeremy/.conan/data/yaml-cpp/0.7.0/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/"
			"/home/jeremy/.conan/data/yaml-cpp/0.7.0/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib/cmake")
set(CONAN_FRAMEWORK_DIRS_YAML-CPP )
set(CONAN_LIBS_YAML-CPP yaml-cpp)
set(CONAN_PKG_LIBS_YAML-CPP yaml-cpp)
set(CONAN_SYSTEM_LIBS_YAML-CPP m)
set(CONAN_FRAMEWORKS_YAML-CPP )
set(CONAN_FRAMEWORKS_FOUND_YAML-CPP "")  # Will be filled later
set(CONAN_DEFINES_YAML-CPP )
set(CONAN_BUILD_MODULES_PATHS_YAML-CPP )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_YAML-CPP )

set(CONAN_C_FLAGS_YAML-CPP "")
set(CONAN_CXX_FLAGS_YAML-CPP "")
set(CONAN_SHARED_LINKER_FLAGS_YAML-CPP "")
set(CONAN_EXE_LINKER_FLAGS_YAML-CPP "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_YAML-CPP_LIST "")
set(CONAN_CXX_FLAGS_YAML-CPP_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_YAML-CPP_LIST "")
set(CONAN_EXE_LINKER_FLAGS_YAML-CPP_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_YAML-CPP "${CONAN_FRAMEWORKS_YAML-CPP}" "_YAML-CPP" "")
# Append to aggregated values variable
set(CONAN_LIBS_YAML-CPP ${CONAN_PKG_LIBS_YAML-CPP} ${CONAN_SYSTEM_LIBS_YAML-CPP} ${CONAN_FRAMEWORKS_FOUND_YAML-CPP})


#################
###  TINYXML
#################
set(CONAN_TINYXML_ROOT "/home/jeremy/.conan/data/tinyxml/2.6.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5")
set(CONAN_INCLUDE_DIRS_TINYXML "/home/jeremy/.conan/data/tinyxml/2.6.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include")
set(CONAN_LIB_DIRS_TINYXML "/home/jeremy/.conan/data/tinyxml/2.6.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib")
set(CONAN_BIN_DIRS_TINYXML )
set(CONAN_RES_DIRS_TINYXML )
set(CONAN_SRC_DIRS_TINYXML )
set(CONAN_BUILD_DIRS_TINYXML "/home/jeremy/.conan/data/tinyxml/2.6.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/")
set(CONAN_FRAMEWORK_DIRS_TINYXML )
set(CONAN_LIBS_TINYXML tinyxml)
set(CONAN_PKG_LIBS_TINYXML tinyxml)
set(CONAN_SYSTEM_LIBS_TINYXML )
set(CONAN_FRAMEWORKS_TINYXML )
set(CONAN_FRAMEWORKS_FOUND_TINYXML "")  # Will be filled later
set(CONAN_DEFINES_TINYXML )
set(CONAN_BUILD_MODULES_PATHS_TINYXML )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_TINYXML )

set(CONAN_C_FLAGS_TINYXML "")
set(CONAN_CXX_FLAGS_TINYXML "")
set(CONAN_SHARED_LINKER_FLAGS_TINYXML "")
set(CONAN_EXE_LINKER_FLAGS_TINYXML "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_TINYXML_LIST "")
set(CONAN_CXX_FLAGS_TINYXML_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_TINYXML_LIST "")
set(CONAN_EXE_LINKER_FLAGS_TINYXML_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_TINYXML "${CONAN_FRAMEWORKS_TINYXML}" "_TINYXML" "")
# Append to aggregated values variable
set(CONAN_LIBS_TINYXML ${CONAN_PKG_LIBS_TINYXML} ${CONAN_SYSTEM_LIBS_TINYXML} ${CONAN_FRAMEWORKS_FOUND_TINYXML})


#################
###  RABBITMQ-C
#################
set(CONAN_RABBITMQ-C_ROOT "/home/jeremy/.conan/data/rabbitmq-c/0.11.0/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7")
set(CONAN_INCLUDE_DIRS_RABBITMQ-C "/home/jeremy/.conan/data/rabbitmq-c/0.11.0/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include")
set(CONAN_LIB_DIRS_RABBITMQ-C "/home/jeremy/.conan/data/rabbitmq-c/0.11.0/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib")
set(CONAN_BIN_DIRS_RABBITMQ-C )
set(CONAN_RES_DIRS_RABBITMQ-C )
set(CONAN_SRC_DIRS_RABBITMQ-C )
set(CONAN_BUILD_DIRS_RABBITMQ-C "/home/jeremy/.conan/data/rabbitmq-c/0.11.0/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/")
set(CONAN_FRAMEWORK_DIRS_RABBITMQ-C )
set(CONAN_LIBS_RABBITMQ-C rabbitmq)
set(CONAN_PKG_LIBS_RABBITMQ-C rabbitmq)
set(CONAN_SYSTEM_LIBS_RABBITMQ-C pthread)
set(CONAN_FRAMEWORKS_RABBITMQ-C )
set(CONAN_FRAMEWORKS_FOUND_RABBITMQ-C "")  # Will be filled later
set(CONAN_DEFINES_RABBITMQ-C "-DAMQP_STATIC")
set(CONAN_BUILD_MODULES_PATHS_RABBITMQ-C )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_RABBITMQ-C "AMQP_STATIC")

set(CONAN_C_FLAGS_RABBITMQ-C "")
set(CONAN_CXX_FLAGS_RABBITMQ-C "")
set(CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C "")
set(CONAN_EXE_LINKER_FLAGS_RABBITMQ-C "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_RABBITMQ-C_LIST "")
set(CONAN_CXX_FLAGS_RABBITMQ-C_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_LIST "")
set(CONAN_EXE_LINKER_FLAGS_RABBITMQ-C_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_RABBITMQ-C "${CONAN_FRAMEWORKS_RABBITMQ-C}" "_RABBITMQ-C" "")
# Append to aggregated values variable
set(CONAN_LIBS_RABBITMQ-C ${CONAN_PKG_LIBS_RABBITMQ-C} ${CONAN_SYSTEM_LIBS_RABBITMQ-C} ${CONAN_FRAMEWORKS_FOUND_RABBITMQ-C})


#################
###  CPPZMQ
#################
set(CONAN_CPPZMQ_ROOT "/home/jeremy/.conan/data/cppzmq/4.8.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9")
set(CONAN_INCLUDE_DIRS_CPPZMQ "/home/jeremy/.conan/data/cppzmq/4.8.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include")
set(CONAN_LIB_DIRS_CPPZMQ "/home/jeremy/.conan/data/cppzmq/4.8.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/lib")
set(CONAN_BIN_DIRS_CPPZMQ )
set(CONAN_RES_DIRS_CPPZMQ )
set(CONAN_SRC_DIRS_CPPZMQ )
set(CONAN_BUILD_DIRS_CPPZMQ "/home/jeremy/.conan/data/cppzmq/4.8.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/"
			"/home/jeremy/.conan/data/cppzmq/4.8.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/lib/cmake")
set(CONAN_FRAMEWORK_DIRS_CPPZMQ )
set(CONAN_LIBS_CPPZMQ )
set(CONAN_PKG_LIBS_CPPZMQ )
set(CONAN_SYSTEM_LIBS_CPPZMQ )
set(CONAN_FRAMEWORKS_CPPZMQ )
set(CONAN_FRAMEWORKS_FOUND_CPPZMQ "")  # Will be filled later
set(CONAN_DEFINES_CPPZMQ )
set(CONAN_BUILD_MODULES_PATHS_CPPZMQ )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_CPPZMQ )

set(CONAN_C_FLAGS_CPPZMQ "")
set(CONAN_CXX_FLAGS_CPPZMQ "")
set(CONAN_SHARED_LINKER_FLAGS_CPPZMQ "")
set(CONAN_EXE_LINKER_FLAGS_CPPZMQ "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_CPPZMQ_LIST "")
set(CONAN_CXX_FLAGS_CPPZMQ_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_CPPZMQ_LIST "")
set(CONAN_EXE_LINKER_FLAGS_CPPZMQ_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_CPPZMQ "${CONAN_FRAMEWORKS_CPPZMQ}" "_CPPZMQ" "")
# Append to aggregated values variable
set(CONAN_LIBS_CPPZMQ ${CONAN_PKG_LIBS_CPPZMQ} ${CONAN_SYSTEM_LIBS_CPPZMQ} ${CONAN_FRAMEWORKS_FOUND_CPPZMQ})


#################
###  PAHO-MQTT-CPP
#################
set(CONAN_PAHO-MQTT-CPP_ROOT "/home/jeremy/.conan/data/paho-mqtt-cpp/1.2.0/_/_/package/ee204b5efbf778b3a37ddd291760c795659df13c")
set(CONAN_INCLUDE_DIRS_PAHO-MQTT-CPP "/home/jeremy/.conan/data/paho-mqtt-cpp/1.2.0/_/_/package/ee204b5efbf778b3a37ddd291760c795659df13c/include")
set(CONAN_LIB_DIRS_PAHO-MQTT-CPP "/home/jeremy/.conan/data/paho-mqtt-cpp/1.2.0/_/_/package/ee204b5efbf778b3a37ddd291760c795659df13c/lib")
set(CONAN_BIN_DIRS_PAHO-MQTT-CPP )
set(CONAN_RES_DIRS_PAHO-MQTT-CPP )
set(CONAN_SRC_DIRS_PAHO-MQTT-CPP )
set(CONAN_BUILD_DIRS_PAHO-MQTT-CPP "/home/jeremy/.conan/data/paho-mqtt-cpp/1.2.0/_/_/package/ee204b5efbf778b3a37ddd291760c795659df13c/")
set(CONAN_FRAMEWORK_DIRS_PAHO-MQTT-CPP )
set(CONAN_LIBS_PAHO-MQTT-CPP paho-mqttpp3)
set(CONAN_PKG_LIBS_PAHO-MQTT-CPP paho-mqttpp3)
set(CONAN_SYSTEM_LIBS_PAHO-MQTT-CPP )
set(CONAN_FRAMEWORKS_PAHO-MQTT-CPP )
set(CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-CPP "")  # Will be filled later
set(CONAN_DEFINES_PAHO-MQTT-CPP )
set(CONAN_BUILD_MODULES_PATHS_PAHO-MQTT-CPP )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-CPP )

set(CONAN_C_FLAGS_PAHO-MQTT-CPP "")
set(CONAN_CXX_FLAGS_PAHO-MQTT-CPP "")
set(CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP "")
set(CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-CPP "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_PAHO-MQTT-CPP_LIST "")
set(CONAN_CXX_FLAGS_PAHO-MQTT-CPP_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_LIST "")
set(CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-CPP_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-CPP "${CONAN_FRAMEWORKS_PAHO-MQTT-CPP}" "_PAHO-MQTT-CPP" "")
# Append to aggregated values variable
set(CONAN_LIBS_PAHO-MQTT-CPP ${CONAN_PKG_LIBS_PAHO-MQTT-CPP} ${CONAN_SYSTEM_LIBS_PAHO-MQTT-CPP} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-CPP})


#################
###  CPPCODEC
#################
set(CONAN_CPPCODEC_ROOT "/home/jeremy/.conan/data/cppcodec/0.2/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9")
set(CONAN_INCLUDE_DIRS_CPPCODEC "/home/jeremy/.conan/data/cppcodec/0.2/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include")
set(CONAN_LIB_DIRS_CPPCODEC )
set(CONAN_BIN_DIRS_CPPCODEC )
set(CONAN_RES_DIRS_CPPCODEC )
set(CONAN_SRC_DIRS_CPPCODEC )
set(CONAN_BUILD_DIRS_CPPCODEC "/home/jeremy/.conan/data/cppcodec/0.2/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/")
set(CONAN_FRAMEWORK_DIRS_CPPCODEC )
set(CONAN_LIBS_CPPCODEC )
set(CONAN_PKG_LIBS_CPPCODEC )
set(CONAN_SYSTEM_LIBS_CPPCODEC )
set(CONAN_FRAMEWORKS_CPPCODEC )
set(CONAN_FRAMEWORKS_FOUND_CPPCODEC "")  # Will be filled later
set(CONAN_DEFINES_CPPCODEC )
set(CONAN_BUILD_MODULES_PATHS_CPPCODEC )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_CPPCODEC )

set(CONAN_C_FLAGS_CPPCODEC "")
set(CONAN_CXX_FLAGS_CPPCODEC "")
set(CONAN_SHARED_LINKER_FLAGS_CPPCODEC "")
set(CONAN_EXE_LINKER_FLAGS_CPPCODEC "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_CPPCODEC_LIST "")
set(CONAN_CXX_FLAGS_CPPCODEC_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_CPPCODEC_LIST "")
set(CONAN_EXE_LINKER_FLAGS_CPPCODEC_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_CPPCODEC "${CONAN_FRAMEWORKS_CPPCODEC}" "_CPPCODEC" "")
# Append to aggregated values variable
set(CONAN_LIBS_CPPCODEC ${CONAN_PKG_LIBS_CPPCODEC} ${CONAN_SYSTEM_LIBS_CPPCODEC} ${CONAN_FRAMEWORKS_FOUND_CPPCODEC})


#################
###  FLATBUFFERS
#################
set(CONAN_FLATBUFFERS_ROOT "/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf")
set(CONAN_INCLUDE_DIRS_FLATBUFFERS "/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/include")
set(CONAN_LIB_DIRS_FLATBUFFERS "/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/lib")
set(CONAN_BIN_DIRS_FLATBUFFERS "/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/bin")
set(CONAN_RES_DIRS_FLATBUFFERS )
set(CONAN_SRC_DIRS_FLATBUFFERS )
set(CONAN_BUILD_DIRS_FLATBUFFERS "/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/"
			"/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/bin/cmake")
set(CONAN_FRAMEWORK_DIRS_FLATBUFFERS )
set(CONAN_LIBS_FLATBUFFERS flatbuffers)
set(CONAN_PKG_LIBS_FLATBUFFERS flatbuffers)
set(CONAN_SYSTEM_LIBS_FLATBUFFERS m)
set(CONAN_FRAMEWORKS_FLATBUFFERS )
set(CONAN_FRAMEWORKS_FOUND_FLATBUFFERS "")  # Will be filled later
set(CONAN_DEFINES_FLATBUFFERS )
set(CONAN_BUILD_MODULES_PATHS_FLATBUFFERS "/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/bin/cmake/BuildFlatBuffers.cmake")
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_FLATBUFFERS )

set(CONAN_C_FLAGS_FLATBUFFERS "")
set(CONAN_CXX_FLAGS_FLATBUFFERS "")
set(CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS "")
set(CONAN_EXE_LINKER_FLAGS_FLATBUFFERS "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_FLATBUFFERS_LIST "")
set(CONAN_CXX_FLAGS_FLATBUFFERS_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_LIST "")
set(CONAN_EXE_LINKER_FLAGS_FLATBUFFERS_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_FLATBUFFERS "${CONAN_FRAMEWORKS_FLATBUFFERS}" "_FLATBUFFERS" "")
# Append to aggregated values variable
set(CONAN_LIBS_FLATBUFFERS ${CONAN_PKG_LIBS_FLATBUFFERS} ${CONAN_SYSTEM_LIBS_FLATBUFFERS} ${CONAN_FRAMEWORKS_FOUND_FLATBUFFERS})


#################
###  GRPC
#################
set(CONAN_GRPC_ROOT "/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1")
set(CONAN_INCLUDE_DIRS_GRPC "/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/include")
set(CONAN_LIB_DIRS_GRPC "/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/lib")
set(CONAN_BIN_DIRS_GRPC "/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/bin")
set(CONAN_RES_DIRS_GRPC )
set(CONAN_SRC_DIRS_GRPC )
set(CONAN_BUILD_DIRS_GRPC "/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/"
			"/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/lib/cmake")
set(CONAN_FRAMEWORK_DIRS_GRPC )
set(CONAN_LIBS_GRPC grpc++_alts grpc++_error_details grpc++_reflection grpc++ grpc++_unsecure grpc_unsecure grpcpp_channelz grpc address_sorting gpr upb grpc_plugin_support)
set(CONAN_PKG_LIBS_GRPC grpc++_alts grpc++_error_details grpc++_reflection grpc++ grpc++_unsecure grpc_unsecure grpcpp_channelz grpc address_sorting gpr upb grpc_plugin_support)
set(CONAN_SYSTEM_LIBS_GRPC m pthread)
set(CONAN_FRAMEWORKS_GRPC )
set(CONAN_FRAMEWORKS_FOUND_GRPC "")  # Will be filled later
set(CONAN_DEFINES_GRPC )
set(CONAN_BUILD_MODULES_PATHS_GRPC )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_GRPC )

set(CONAN_C_FLAGS_GRPC "")
set(CONAN_CXX_FLAGS_GRPC "")
set(CONAN_SHARED_LINKER_FLAGS_GRPC "")
set(CONAN_EXE_LINKER_FLAGS_GRPC "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_GRPC_LIST "")
set(CONAN_CXX_FLAGS_GRPC_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_GRPC_LIST "")
set(CONAN_EXE_LINKER_FLAGS_GRPC_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_GRPC "${CONAN_FRAMEWORKS_GRPC}" "_GRPC" "")
# Append to aggregated values variable
set(CONAN_LIBS_GRPC ${CONAN_PKG_LIBS_GRPC} ${CONAN_SYSTEM_LIBS_GRPC} ${CONAN_FRAMEWORKS_FOUND_GRPC})


#################
###  SERIAL
#################
set(CONAN_SERIAL_ROOT "/home/jeremy/.conan/data/serial/1.2.1/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5")
set(CONAN_INCLUDE_DIRS_SERIAL "/home/jeremy/.conan/data/serial/1.2.1/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include")
set(CONAN_LIB_DIRS_SERIAL "/home/jeremy/.conan/data/serial/1.2.1/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib")
set(CONAN_BIN_DIRS_SERIAL )
set(CONAN_RES_DIRS_SERIAL )
set(CONAN_SRC_DIRS_SERIAL )
set(CONAN_BUILD_DIRS_SERIAL "/home/jeremy/.conan/data/serial/1.2.1/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/")
set(CONAN_FRAMEWORK_DIRS_SERIAL )
set(CONAN_LIBS_SERIAL serial)
set(CONAN_PKG_LIBS_SERIAL serial)
set(CONAN_SYSTEM_LIBS_SERIAL rt pthread)
set(CONAN_FRAMEWORKS_SERIAL )
set(CONAN_FRAMEWORKS_FOUND_SERIAL "")  # Will be filled later
set(CONAN_DEFINES_SERIAL )
set(CONAN_BUILD_MODULES_PATHS_SERIAL )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_SERIAL )

set(CONAN_C_FLAGS_SERIAL "")
set(CONAN_CXX_FLAGS_SERIAL "")
set(CONAN_SHARED_LINKER_FLAGS_SERIAL "")
set(CONAN_EXE_LINKER_FLAGS_SERIAL "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_SERIAL_LIST "")
set(CONAN_CXX_FLAGS_SERIAL_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_SERIAL_LIST "")
set(CONAN_EXE_LINKER_FLAGS_SERIAL_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_SERIAL "${CONAN_FRAMEWORKS_SERIAL}" "_SERIAL" "")
# Append to aggregated values variable
set(CONAN_LIBS_SERIAL ${CONAN_PKG_LIBS_SERIAL} ${CONAN_SYSTEM_LIBS_SERIAL} ${CONAN_FRAMEWORKS_FOUND_SERIAL})


#################
###  BOOST
#################
set(CONAN_BOOST_ROOT "/home/jeremy/.conan/data/boost/1.77.0/_/_/package/f6db3e065c8e78b76924e832f0eb9cf91494e9ee")
set(CONAN_INCLUDE_DIRS_BOOST "/home/jeremy/.conan/data/boost/1.77.0/_/_/package/f6db3e065c8e78b76924e832f0eb9cf91494e9ee/include")
set(CONAN_LIB_DIRS_BOOST "/home/jeremy/.conan/data/boost/1.77.0/_/_/package/f6db3e065c8e78b76924e832f0eb9cf91494e9ee/lib")
set(CONAN_BIN_DIRS_BOOST )
set(CONAN_RES_DIRS_BOOST )
set(CONAN_SRC_DIRS_BOOST )
set(CONAN_BUILD_DIRS_BOOST "/home/jeremy/.conan/data/boost/1.77.0/_/_/package/f6db3e065c8e78b76924e832f0eb9cf91494e9ee/")
set(CONAN_FRAMEWORK_DIRS_BOOST )
set(CONAN_LIBS_BOOST boost_contract boost_coroutine boost_fiber_numa boost_fiber boost_context boost_graph boost_iostreams boost_json boost_locale boost_log_setup boost_log boost_math_c99 boost_math_c99f boost_math_c99l boost_math_tr1 boost_math_tr1f boost_math_tr1l boost_nowide boost_program_options boost_random boost_regex boost_stacktrace_addr2line boost_stacktrace_backtrace boost_stacktrace_basic boost_stacktrace_noop boost_timer boost_type_erasure boost_thread boost_chrono boost_container boost_date_time boost_unit_test_framework boost_prg_exec_monitor boost_test_exec_monitor boost_exception boost_wave boost_filesystem boost_atomic boost_wserialization boost_serialization)
set(CONAN_PKG_LIBS_BOOST boost_contract boost_coroutine boost_fiber_numa boost_fiber boost_context boost_graph boost_iostreams boost_json boost_locale boost_log_setup boost_log boost_math_c99 boost_math_c99f boost_math_c99l boost_math_tr1 boost_math_tr1f boost_math_tr1l boost_nowide boost_program_options boost_random boost_regex boost_stacktrace_addr2line boost_stacktrace_backtrace boost_stacktrace_basic boost_stacktrace_noop boost_timer boost_type_erasure boost_thread boost_chrono boost_container boost_date_time boost_unit_test_framework boost_prg_exec_monitor boost_test_exec_monitor boost_exception boost_wave boost_filesystem boost_atomic boost_wserialization boost_serialization)
set(CONAN_SYSTEM_LIBS_BOOST dl rt pthread)
set(CONAN_FRAMEWORKS_BOOST )
set(CONAN_FRAMEWORKS_FOUND_BOOST "")  # Will be filled later
set(CONAN_DEFINES_BOOST "-DBOOST_STACKTRACE_ADDR2LINE_LOCATION=\"/usr/bin/addr2line\""
			"-DBOOST_STACKTRACE_USE_ADDR2LINE"
			"-DBOOST_STACKTRACE_USE_BACKTRACE"
			"-DBOOST_STACKTRACE_USE_NOOP")
set(CONAN_BUILD_MODULES_PATHS_BOOST )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_BOOST "BOOST_STACKTRACE_ADDR2LINE_LOCATION=\"/usr/bin/addr2line\""
			"BOOST_STACKTRACE_USE_ADDR2LINE"
			"BOOST_STACKTRACE_USE_BACKTRACE"
			"BOOST_STACKTRACE_USE_NOOP")

set(CONAN_C_FLAGS_BOOST "")
set(CONAN_CXX_FLAGS_BOOST "")
set(CONAN_SHARED_LINKER_FLAGS_BOOST "")
set(CONAN_EXE_LINKER_FLAGS_BOOST "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_BOOST_LIST "")
set(CONAN_CXX_FLAGS_BOOST_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_BOOST_LIST "")
set(CONAN_EXE_LINKER_FLAGS_BOOST_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_BOOST "${CONAN_FRAMEWORKS_BOOST}" "_BOOST" "")
# Append to aggregated values variable
set(CONAN_LIBS_BOOST ${CONAN_PKG_LIBS_BOOST} ${CONAN_SYSTEM_LIBS_BOOST} ${CONAN_FRAMEWORKS_FOUND_BOOST})


#################
###  SQLITE3
#################
set(CONAN_SQLITE3_ROOT "/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65")
set(CONAN_INCLUDE_DIRS_SQLITE3 "/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/include")
set(CONAN_LIB_DIRS_SQLITE3 "/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/lib")
set(CONAN_BIN_DIRS_SQLITE3 "/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/bin")
set(CONAN_RES_DIRS_SQLITE3 )
set(CONAN_SRC_DIRS_SQLITE3 )
set(CONAN_BUILD_DIRS_SQLITE3 "/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/"
			"/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/lib/cmake")
set(CONAN_FRAMEWORK_DIRS_SQLITE3 )
set(CONAN_LIBS_SQLITE3 sqlite3)
set(CONAN_PKG_LIBS_SQLITE3 sqlite3)
set(CONAN_SYSTEM_LIBS_SQLITE3 pthread dl m)
set(CONAN_FRAMEWORKS_SQLITE3 )
set(CONAN_FRAMEWORKS_FOUND_SQLITE3 "")  # Will be filled later
set(CONAN_DEFINES_SQLITE3 )
set(CONAN_BUILD_MODULES_PATHS_SQLITE3 )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_SQLITE3 )

set(CONAN_C_FLAGS_SQLITE3 "")
set(CONAN_CXX_FLAGS_SQLITE3 "")
set(CONAN_SHARED_LINKER_FLAGS_SQLITE3 "")
set(CONAN_EXE_LINKER_FLAGS_SQLITE3 "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_SQLITE3_LIST "")
set(CONAN_CXX_FLAGS_SQLITE3_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_SQLITE3_LIST "")
set(CONAN_EXE_LINKER_FLAGS_SQLITE3_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_SQLITE3 "${CONAN_FRAMEWORKS_SQLITE3}" "_SQLITE3" "")
# Append to aggregated values variable
set(CONAN_LIBS_SQLITE3 ${CONAN_PKG_LIBS_SQLITE3} ${CONAN_SYSTEM_LIBS_SQLITE3} ${CONAN_FRAMEWORKS_FOUND_SQLITE3})


#################
###  LIBPQ
#################
set(CONAN_LIBPQ_ROOT "/home/jeremy/.conan/data/libpq/13.3/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7")
set(CONAN_INCLUDE_DIRS_LIBPQ "/home/jeremy/.conan/data/libpq/13.3/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include")
set(CONAN_LIB_DIRS_LIBPQ "/home/jeremy/.conan/data/libpq/13.3/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib")
set(CONAN_BIN_DIRS_LIBPQ "/home/jeremy/.conan/data/libpq/13.3/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/bin")
set(CONAN_RES_DIRS_LIBPQ )
set(CONAN_SRC_DIRS_LIBPQ )
set(CONAN_BUILD_DIRS_LIBPQ "/home/jeremy/.conan/data/libpq/13.3/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/")
set(CONAN_FRAMEWORK_DIRS_LIBPQ )
set(CONAN_LIBS_LIBPQ pq pgcommon pgcommon_shlib pgport pgport_shlib)
set(CONAN_PKG_LIBS_LIBPQ pq pgcommon pgcommon_shlib pgport pgport_shlib)
set(CONAN_SYSTEM_LIBS_LIBPQ pthread)
set(CONAN_FRAMEWORKS_LIBPQ )
set(CONAN_FRAMEWORKS_FOUND_LIBPQ "")  # Will be filled later
set(CONAN_DEFINES_LIBPQ )
set(CONAN_BUILD_MODULES_PATHS_LIBPQ )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_LIBPQ )

set(CONAN_C_FLAGS_LIBPQ "")
set(CONAN_CXX_FLAGS_LIBPQ "")
set(CONAN_SHARED_LINKER_FLAGS_LIBPQ "")
set(CONAN_EXE_LINKER_FLAGS_LIBPQ "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_LIBPQ_LIST "")
set(CONAN_CXX_FLAGS_LIBPQ_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_LIBPQ_LIST "")
set(CONAN_EXE_LINKER_FLAGS_LIBPQ_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_LIBPQ "${CONAN_FRAMEWORKS_LIBPQ}" "_LIBPQ" "")
# Append to aggregated values variable
set(CONAN_LIBS_LIBPQ ${CONAN_PKG_LIBS_LIBPQ} ${CONAN_SYSTEM_LIBS_LIBPQ} ${CONAN_FRAMEWORKS_FOUND_LIBPQ})


#################
###  LIBCURL
#################
set(CONAN_LIBCURL_ROOT "/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95")
set(CONAN_INCLUDE_DIRS_LIBCURL "/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/include")
set(CONAN_LIB_DIRS_LIBCURL "/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/lib")
set(CONAN_BIN_DIRS_LIBCURL "/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/bin")
set(CONAN_RES_DIRS_LIBCURL "/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/res")
set(CONAN_SRC_DIRS_LIBCURL )
set(CONAN_BUILD_DIRS_LIBCURL "/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/")
set(CONAN_FRAMEWORK_DIRS_LIBCURL )
set(CONAN_LIBS_LIBCURL curl)
set(CONAN_PKG_LIBS_LIBCURL curl)
set(CONAN_SYSTEM_LIBS_LIBCURL rt pthread)
set(CONAN_FRAMEWORKS_LIBCURL )
set(CONAN_FRAMEWORKS_FOUND_LIBCURL "")  # Will be filled later
set(CONAN_DEFINES_LIBCURL "-DCURL_STATICLIB=1")
set(CONAN_BUILD_MODULES_PATHS_LIBCURL )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_LIBCURL "CURL_STATICLIB=1")

set(CONAN_C_FLAGS_LIBCURL "")
set(CONAN_CXX_FLAGS_LIBCURL "")
set(CONAN_SHARED_LINKER_FLAGS_LIBCURL "")
set(CONAN_EXE_LINKER_FLAGS_LIBCURL "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_LIBCURL_LIST "")
set(CONAN_CXX_FLAGS_LIBCURL_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_LIBCURL_LIST "")
set(CONAN_EXE_LINKER_FLAGS_LIBCURL_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_LIBCURL "${CONAN_FRAMEWORKS_LIBCURL}" "_LIBCURL" "")
# Append to aggregated values variable
set(CONAN_LIBS_LIBCURL ${CONAN_PKG_LIBS_LIBCURL} ${CONAN_SYSTEM_LIBS_LIBCURL} ${CONAN_FRAMEWORKS_FOUND_LIBCURL})


#################
###  NLOHMANN_JSON
#################
set(CONAN_NLOHMANN_JSON_ROOT "/home/jeremy/.conan/data/nlohmann_json/3.10.3/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9")
set(CONAN_INCLUDE_DIRS_NLOHMANN_JSON "/home/jeremy/.conan/data/nlohmann_json/3.10.3/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include")
set(CONAN_LIB_DIRS_NLOHMANN_JSON )
set(CONAN_BIN_DIRS_NLOHMANN_JSON )
set(CONAN_RES_DIRS_NLOHMANN_JSON )
set(CONAN_SRC_DIRS_NLOHMANN_JSON )
set(CONAN_BUILD_DIRS_NLOHMANN_JSON "/home/jeremy/.conan/data/nlohmann_json/3.10.3/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/")
set(CONAN_FRAMEWORK_DIRS_NLOHMANN_JSON )
set(CONAN_LIBS_NLOHMANN_JSON )
set(CONAN_PKG_LIBS_NLOHMANN_JSON )
set(CONAN_SYSTEM_LIBS_NLOHMANN_JSON )
set(CONAN_FRAMEWORKS_NLOHMANN_JSON )
set(CONAN_FRAMEWORKS_FOUND_NLOHMANN_JSON "")  # Will be filled later
set(CONAN_DEFINES_NLOHMANN_JSON )
set(CONAN_BUILD_MODULES_PATHS_NLOHMANN_JSON )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_NLOHMANN_JSON )

set(CONAN_C_FLAGS_NLOHMANN_JSON "")
set(CONAN_CXX_FLAGS_NLOHMANN_JSON "")
set(CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON "")
set(CONAN_EXE_LINKER_FLAGS_NLOHMANN_JSON "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_NLOHMANN_JSON_LIST "")
set(CONAN_CXX_FLAGS_NLOHMANN_JSON_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_LIST "")
set(CONAN_EXE_LINKER_FLAGS_NLOHMANN_JSON_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_NLOHMANN_JSON "${CONAN_FRAMEWORKS_NLOHMANN_JSON}" "_NLOHMANN_JSON" "")
# Append to aggregated values variable
set(CONAN_LIBS_NLOHMANN_JSON ${CONAN_PKG_LIBS_NLOHMANN_JSON} ${CONAN_SYSTEM_LIBS_NLOHMANN_JSON} ${CONAN_FRAMEWORKS_FOUND_NLOHMANN_JSON})


#################
###  ZEROMQ
#################
set(CONAN_ZEROMQ_ROOT "/home/jeremy/.conan/data/zeromq/4.3.4/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5")
set(CONAN_INCLUDE_DIRS_ZEROMQ "/home/jeremy/.conan/data/zeromq/4.3.4/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include")
set(CONAN_LIB_DIRS_ZEROMQ "/home/jeremy/.conan/data/zeromq/4.3.4/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib")
set(CONAN_BIN_DIRS_ZEROMQ )
set(CONAN_RES_DIRS_ZEROMQ )
set(CONAN_SRC_DIRS_ZEROMQ )
set(CONAN_BUILD_DIRS_ZEROMQ "/home/jeremy/.conan/data/zeromq/4.3.4/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/")
set(CONAN_FRAMEWORK_DIRS_ZEROMQ )
set(CONAN_LIBS_ZEROMQ zmq)
set(CONAN_PKG_LIBS_ZEROMQ zmq)
set(CONAN_SYSTEM_LIBS_ZEROMQ pthread rt m)
set(CONAN_FRAMEWORKS_ZEROMQ )
set(CONAN_FRAMEWORKS_FOUND_ZEROMQ "")  # Will be filled later
set(CONAN_DEFINES_ZEROMQ "-DZMQ_STATIC")
set(CONAN_BUILD_MODULES_PATHS_ZEROMQ )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_ZEROMQ "ZMQ_STATIC")

set(CONAN_C_FLAGS_ZEROMQ "")
set(CONAN_CXX_FLAGS_ZEROMQ "")
set(CONAN_SHARED_LINKER_FLAGS_ZEROMQ "")
set(CONAN_EXE_LINKER_FLAGS_ZEROMQ "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_ZEROMQ_LIST "")
set(CONAN_CXX_FLAGS_ZEROMQ_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_ZEROMQ_LIST "")
set(CONAN_EXE_LINKER_FLAGS_ZEROMQ_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_ZEROMQ "${CONAN_FRAMEWORKS_ZEROMQ}" "_ZEROMQ" "")
# Append to aggregated values variable
set(CONAN_LIBS_ZEROMQ ${CONAN_PKG_LIBS_ZEROMQ} ${CONAN_SYSTEM_LIBS_ZEROMQ} ${CONAN_FRAMEWORKS_FOUND_ZEROMQ})


#################
###  PAHO-MQTT-C
#################
set(CONAN_PAHO-MQTT-C_ROOT "/home/jeremy/.conan/data/paho-mqtt-c/1.3.8/_/_/package/f6647b6445c791c3f4410db4ef533c1e61db8fed")
set(CONAN_INCLUDE_DIRS_PAHO-MQTT-C "/home/jeremy/.conan/data/paho-mqtt-c/1.3.8/_/_/package/f6647b6445c791c3f4410db4ef533c1e61db8fed/include")
set(CONAN_LIB_DIRS_PAHO-MQTT-C "/home/jeremy/.conan/data/paho-mqtt-c/1.3.8/_/_/package/f6647b6445c791c3f4410db4ef533c1e61db8fed/lib")
set(CONAN_BIN_DIRS_PAHO-MQTT-C )
set(CONAN_RES_DIRS_PAHO-MQTT-C )
set(CONAN_SRC_DIRS_PAHO-MQTT-C )
set(CONAN_BUILD_DIRS_PAHO-MQTT-C "/home/jeremy/.conan/data/paho-mqtt-c/1.3.8/_/_/package/f6647b6445c791c3f4410db4ef533c1e61db8fed/")
set(CONAN_FRAMEWORK_DIRS_PAHO-MQTT-C )
set(CONAN_LIBS_PAHO-MQTT-C paho-mqtt3as)
set(CONAN_PKG_LIBS_PAHO-MQTT-C paho-mqtt3as)
set(CONAN_SYSTEM_LIBS_PAHO-MQTT-C c dl pthread)
set(CONAN_FRAMEWORKS_PAHO-MQTT-C )
set(CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-C "")  # Will be filled later
set(CONAN_DEFINES_PAHO-MQTT-C )
set(CONAN_BUILD_MODULES_PATHS_PAHO-MQTT-C )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-C )

set(CONAN_C_FLAGS_PAHO-MQTT-C "")
set(CONAN_CXX_FLAGS_PAHO-MQTT-C "")
set(CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C "")
set(CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-C "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_PAHO-MQTT-C_LIST "")
set(CONAN_CXX_FLAGS_PAHO-MQTT-C_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_LIST "")
set(CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-C_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-C "${CONAN_FRAMEWORKS_PAHO-MQTT-C}" "_PAHO-MQTT-C" "")
# Append to aggregated values variable
set(CONAN_LIBS_PAHO-MQTT-C ${CONAN_PKG_LIBS_PAHO-MQTT-C} ${CONAN_SYSTEM_LIBS_PAHO-MQTT-C} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-C})


#################
###  PROTOBUF
#################
set(CONAN_PROTOBUF_ROOT "/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6")
set(CONAN_INCLUDE_DIRS_PROTOBUF "/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/include")
set(CONAN_LIB_DIRS_PROTOBUF "/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib")
set(CONAN_BIN_DIRS_PROTOBUF "/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/bin")
set(CONAN_RES_DIRS_PROTOBUF )
set(CONAN_SRC_DIRS_PROTOBUF )
set(CONAN_BUILD_DIRS_PROTOBUF "/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib/cmake/protobuf")
set(CONAN_FRAMEWORK_DIRS_PROTOBUF )
set(CONAN_LIBS_PROTOBUF protoc protobuf)
set(CONAN_PKG_LIBS_PROTOBUF protoc protobuf)
set(CONAN_SYSTEM_LIBS_PROTOBUF pthread)
set(CONAN_FRAMEWORKS_PROTOBUF )
set(CONAN_FRAMEWORKS_FOUND_PROTOBUF "")  # Will be filled later
set(CONAN_DEFINES_PROTOBUF )
set(CONAN_BUILD_MODULES_PATHS_PROTOBUF "/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib/cmake/protobuf/protobuf-generate.cmake"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib/cmake/protobuf/protobuf-module.cmake"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib/cmake/protobuf/protobuf-options.cmake")
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_PROTOBUF )

set(CONAN_C_FLAGS_PROTOBUF "")
set(CONAN_CXX_FLAGS_PROTOBUF "")
set(CONAN_SHARED_LINKER_FLAGS_PROTOBUF "")
set(CONAN_EXE_LINKER_FLAGS_PROTOBUF "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_PROTOBUF_LIST "")
set(CONAN_CXX_FLAGS_PROTOBUF_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_PROTOBUF_LIST "")
set(CONAN_EXE_LINKER_FLAGS_PROTOBUF_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_PROTOBUF "${CONAN_FRAMEWORKS_PROTOBUF}" "_PROTOBUF" "")
# Append to aggregated values variable
set(CONAN_LIBS_PROTOBUF ${CONAN_PKG_LIBS_PROTOBUF} ${CONAN_SYSTEM_LIBS_PROTOBUF} ${CONAN_FRAMEWORKS_FOUND_PROTOBUF})


#################
###  C-ARES
#################
set(CONAN_C-ARES_ROOT "/home/jeremy/.conan/data/c-ares/1.17.1/_/_/package/c2627225292f8d772a40f1bfef7180540cbf7f0d")
set(CONAN_INCLUDE_DIRS_C-ARES "/home/jeremy/.conan/data/c-ares/1.17.1/_/_/package/c2627225292f8d772a40f1bfef7180540cbf7f0d/include")
set(CONAN_LIB_DIRS_C-ARES "/home/jeremy/.conan/data/c-ares/1.17.1/_/_/package/c2627225292f8d772a40f1bfef7180540cbf7f0d/lib")
set(CONAN_BIN_DIRS_C-ARES "/home/jeremy/.conan/data/c-ares/1.17.1/_/_/package/c2627225292f8d772a40f1bfef7180540cbf7f0d/bin")
set(CONAN_RES_DIRS_C-ARES )
set(CONAN_SRC_DIRS_C-ARES )
set(CONAN_BUILD_DIRS_C-ARES "/home/jeremy/.conan/data/c-ares/1.17.1/_/_/package/c2627225292f8d772a40f1bfef7180540cbf7f0d/")
set(CONAN_FRAMEWORK_DIRS_C-ARES )
set(CONAN_LIBS_C-ARES cares)
set(CONAN_PKG_LIBS_C-ARES cares)
set(CONAN_SYSTEM_LIBS_C-ARES rt)
set(CONAN_FRAMEWORKS_C-ARES )
set(CONAN_FRAMEWORKS_FOUND_C-ARES "")  # Will be filled later
set(CONAN_DEFINES_C-ARES "-DCARES_STATICLIB")
set(CONAN_BUILD_MODULES_PATHS_C-ARES )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_C-ARES "CARES_STATICLIB")

set(CONAN_C_FLAGS_C-ARES "")
set(CONAN_CXX_FLAGS_C-ARES "")
set(CONAN_SHARED_LINKER_FLAGS_C-ARES "")
set(CONAN_EXE_LINKER_FLAGS_C-ARES "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_C-ARES_LIST "")
set(CONAN_CXX_FLAGS_C-ARES_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_C-ARES_LIST "")
set(CONAN_EXE_LINKER_FLAGS_C-ARES_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_C-ARES "${CONAN_FRAMEWORKS_C-ARES}" "_C-ARES" "")
# Append to aggregated values variable
set(CONAN_LIBS_C-ARES ${CONAN_PKG_LIBS_C-ARES} ${CONAN_SYSTEM_LIBS_C-ARES} ${CONAN_FRAMEWORKS_FOUND_C-ARES})


#################
###  ABSEIL
#################
set(CONAN_ABSEIL_ROOT "/home/jeremy/.conan/data/abseil/20210324.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5")
set(CONAN_INCLUDE_DIRS_ABSEIL "/home/jeremy/.conan/data/abseil/20210324.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include")
set(CONAN_LIB_DIRS_ABSEIL "/home/jeremy/.conan/data/abseil/20210324.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib")
set(CONAN_BIN_DIRS_ABSEIL )
set(CONAN_RES_DIRS_ABSEIL )
set(CONAN_SRC_DIRS_ABSEIL )
set(CONAN_BUILD_DIRS_ABSEIL "/home/jeremy/.conan/data/abseil/20210324.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/")
set(CONAN_FRAMEWORK_DIRS_ABSEIL )
set(CONAN_LIBS_ABSEIL absl_periodic_sampler absl_scoped_set_env absl_strerror absl_failure_signal_handler absl_examine_stack absl_leak_check_disable absl_leak_check absl_flags_parse absl_flags_usage absl_flags_usage_internal absl_flags absl_flags_reflection absl_raw_hash_set absl_hashtablez_sampler absl_exponential_biased absl_flags_private_handle_accessor absl_flags_internal absl_flags_config absl_flags_program_name absl_flags_marshalling absl_flags_commandlineflag absl_flags_commandlineflag_internal absl_hash absl_city absl_wyhash absl_random_distributions absl_random_seed_sequences absl_random_internal_pool_urbg absl_random_seed_gen_exception absl_random_internal_seed_material absl_random_internal_randen absl_random_internal_randen_slow absl_random_internal_randen_hwaes absl_random_internal_randen_hwaes_impl absl_random_internal_platform absl_random_internal_distribution_test_util absl_statusor absl_status absl_str_format_internal absl_cord absl_synchronization absl_stacktrace absl_symbolize absl_debugging_internal absl_demangle_internal absl_graphcycles_internal absl_malloc_internal absl_time absl_strings absl_int128 absl_strings_internal absl_base absl_spinlock_wait absl_civil_time absl_time_zone absl_bad_any_cast_impl absl_throw_delegate absl_bad_optional_access absl_bad_variant_access absl_raw_logging_internal absl_log_severity)
set(CONAN_PKG_LIBS_ABSEIL absl_periodic_sampler absl_scoped_set_env absl_strerror absl_failure_signal_handler absl_examine_stack absl_leak_check_disable absl_leak_check absl_flags_parse absl_flags_usage absl_flags_usage_internal absl_flags absl_flags_reflection absl_raw_hash_set absl_hashtablez_sampler absl_exponential_biased absl_flags_private_handle_accessor absl_flags_internal absl_flags_config absl_flags_program_name absl_flags_marshalling absl_flags_commandlineflag absl_flags_commandlineflag_internal absl_hash absl_city absl_wyhash absl_random_distributions absl_random_seed_sequences absl_random_internal_pool_urbg absl_random_seed_gen_exception absl_random_internal_seed_material absl_random_internal_randen absl_random_internal_randen_slow absl_random_internal_randen_hwaes absl_random_internal_randen_hwaes_impl absl_random_internal_platform absl_random_internal_distribution_test_util absl_statusor absl_status absl_str_format_internal absl_cord absl_synchronization absl_stacktrace absl_symbolize absl_debugging_internal absl_demangle_internal absl_graphcycles_internal absl_malloc_internal absl_time absl_strings absl_int128 absl_strings_internal absl_base absl_spinlock_wait absl_civil_time absl_time_zone absl_bad_any_cast_impl absl_throw_delegate absl_bad_optional_access absl_bad_variant_access absl_raw_logging_internal absl_log_severity)
set(CONAN_SYSTEM_LIBS_ABSEIL pthread rt)
set(CONAN_FRAMEWORKS_ABSEIL )
set(CONAN_FRAMEWORKS_FOUND_ABSEIL "")  # Will be filled later
set(CONAN_DEFINES_ABSEIL )
set(CONAN_BUILD_MODULES_PATHS_ABSEIL )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_ABSEIL )

set(CONAN_C_FLAGS_ABSEIL "")
set(CONAN_CXX_FLAGS_ABSEIL "")
set(CONAN_SHARED_LINKER_FLAGS_ABSEIL "")
set(CONAN_EXE_LINKER_FLAGS_ABSEIL "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_ABSEIL_LIST "")
set(CONAN_CXX_FLAGS_ABSEIL_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_ABSEIL_LIST "")
set(CONAN_EXE_LINKER_FLAGS_ABSEIL_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_ABSEIL "${CONAN_FRAMEWORKS_ABSEIL}" "_ABSEIL" "")
# Append to aggregated values variable
set(CONAN_LIBS_ABSEIL ${CONAN_PKG_LIBS_ABSEIL} ${CONAN_SYSTEM_LIBS_ABSEIL} ${CONAN_FRAMEWORKS_FOUND_ABSEIL})


#################
###  RE2
#################
set(CONAN_RE2_ROOT "/home/jeremy/.conan/data/re2/20210601/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5")
set(CONAN_INCLUDE_DIRS_RE2 "/home/jeremy/.conan/data/re2/20210601/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include")
set(CONAN_LIB_DIRS_RE2 "/home/jeremy/.conan/data/re2/20210601/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib")
set(CONAN_BIN_DIRS_RE2 )
set(CONAN_RES_DIRS_RE2 )
set(CONAN_SRC_DIRS_RE2 )
set(CONAN_BUILD_DIRS_RE2 "/home/jeremy/.conan/data/re2/20210601/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/")
set(CONAN_FRAMEWORK_DIRS_RE2 )
set(CONAN_LIBS_RE2 re2)
set(CONAN_PKG_LIBS_RE2 re2)
set(CONAN_SYSTEM_LIBS_RE2 m pthread)
set(CONAN_FRAMEWORKS_RE2 )
set(CONAN_FRAMEWORKS_FOUND_RE2 "")  # Will be filled later
set(CONAN_DEFINES_RE2 )
set(CONAN_BUILD_MODULES_PATHS_RE2 )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_RE2 )

set(CONAN_C_FLAGS_RE2 "")
set(CONAN_CXX_FLAGS_RE2 "")
set(CONAN_SHARED_LINKER_FLAGS_RE2 "")
set(CONAN_EXE_LINKER_FLAGS_RE2 "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_RE2_LIST "")
set(CONAN_CXX_FLAGS_RE2_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_RE2_LIST "")
set(CONAN_EXE_LINKER_FLAGS_RE2_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_RE2 "${CONAN_FRAMEWORKS_RE2}" "_RE2" "")
# Append to aggregated values variable
set(CONAN_LIBS_RE2 ${CONAN_PKG_LIBS_RE2} ${CONAN_SYSTEM_LIBS_RE2} ${CONAN_FRAMEWORKS_FOUND_RE2})


#################
###  OPENSSL
#################
set(CONAN_OPENSSL_ROOT "/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c")
set(CONAN_INCLUDE_DIRS_OPENSSL "/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/include")
set(CONAN_LIB_DIRS_OPENSSL "/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/lib")
set(CONAN_BIN_DIRS_OPENSSL "/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/bin")
set(CONAN_RES_DIRS_OPENSSL )
set(CONAN_SRC_DIRS_OPENSSL )
set(CONAN_BUILD_DIRS_OPENSSL "/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/"
			"/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/lib/cmake")
set(CONAN_FRAMEWORK_DIRS_OPENSSL )
set(CONAN_LIBS_OPENSSL ssl crypto)
set(CONAN_PKG_LIBS_OPENSSL ssl crypto)
set(CONAN_SYSTEM_LIBS_OPENSSL dl pthread rt)
set(CONAN_FRAMEWORKS_OPENSSL )
set(CONAN_FRAMEWORKS_FOUND_OPENSSL "")  # Will be filled later
set(CONAN_DEFINES_OPENSSL )
set(CONAN_BUILD_MODULES_PATHS_OPENSSL )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_OPENSSL )

set(CONAN_C_FLAGS_OPENSSL "")
set(CONAN_CXX_FLAGS_OPENSSL "")
set(CONAN_SHARED_LINKER_FLAGS_OPENSSL "")
set(CONAN_EXE_LINKER_FLAGS_OPENSSL "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_OPENSSL_LIST "")
set(CONAN_CXX_FLAGS_OPENSSL_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_OPENSSL_LIST "")
set(CONAN_EXE_LINKER_FLAGS_OPENSSL_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_OPENSSL "${CONAN_FRAMEWORKS_OPENSSL}" "_OPENSSL" "")
# Append to aggregated values variable
set(CONAN_LIBS_OPENSSL ${CONAN_PKG_LIBS_OPENSSL} ${CONAN_SYSTEM_LIBS_OPENSSL} ${CONAN_FRAMEWORKS_FOUND_OPENSSL})


#################
###  BZIP2
#################
set(CONAN_BZIP2_ROOT "/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f")
set(CONAN_INCLUDE_DIRS_BZIP2 "/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/include")
set(CONAN_LIB_DIRS_BZIP2 "/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/lib")
set(CONAN_BIN_DIRS_BZIP2 "/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/bin")
set(CONAN_RES_DIRS_BZIP2 )
set(CONAN_SRC_DIRS_BZIP2 )
set(CONAN_BUILD_DIRS_BZIP2 "/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/"
			"/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/lib/cmake")
set(CONAN_FRAMEWORK_DIRS_BZIP2 )
set(CONAN_LIBS_BZIP2 bz2)
set(CONAN_PKG_LIBS_BZIP2 bz2)
set(CONAN_SYSTEM_LIBS_BZIP2 )
set(CONAN_FRAMEWORKS_BZIP2 )
set(CONAN_FRAMEWORKS_FOUND_BZIP2 "")  # Will be filled later
set(CONAN_DEFINES_BZIP2 )
set(CONAN_BUILD_MODULES_PATHS_BZIP2 )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_BZIP2 )

set(CONAN_C_FLAGS_BZIP2 "")
set(CONAN_CXX_FLAGS_BZIP2 "")
set(CONAN_SHARED_LINKER_FLAGS_BZIP2 "")
set(CONAN_EXE_LINKER_FLAGS_BZIP2 "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_BZIP2_LIST "")
set(CONAN_CXX_FLAGS_BZIP2_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_BZIP2_LIST "")
set(CONAN_EXE_LINKER_FLAGS_BZIP2_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_BZIP2 "${CONAN_FRAMEWORKS_BZIP2}" "_BZIP2" "")
# Append to aggregated values variable
set(CONAN_LIBS_BZIP2 ${CONAN_PKG_LIBS_BZIP2} ${CONAN_SYSTEM_LIBS_BZIP2} ${CONAN_FRAMEWORKS_FOUND_BZIP2})


#################
###  LIBBACKTRACE
#################
set(CONAN_LIBBACKTRACE_ROOT "/home/jeremy/.conan/data/libbacktrace/cci.20210118/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7")
set(CONAN_INCLUDE_DIRS_LIBBACKTRACE "/home/jeremy/.conan/data/libbacktrace/cci.20210118/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include")
set(CONAN_LIB_DIRS_LIBBACKTRACE "/home/jeremy/.conan/data/libbacktrace/cci.20210118/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib")
set(CONAN_BIN_DIRS_LIBBACKTRACE )
set(CONAN_RES_DIRS_LIBBACKTRACE )
set(CONAN_SRC_DIRS_LIBBACKTRACE )
set(CONAN_BUILD_DIRS_LIBBACKTRACE "/home/jeremy/.conan/data/libbacktrace/cci.20210118/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/")
set(CONAN_FRAMEWORK_DIRS_LIBBACKTRACE )
set(CONAN_LIBS_LIBBACKTRACE backtrace)
set(CONAN_PKG_LIBS_LIBBACKTRACE backtrace)
set(CONAN_SYSTEM_LIBS_LIBBACKTRACE )
set(CONAN_FRAMEWORKS_LIBBACKTRACE )
set(CONAN_FRAMEWORKS_FOUND_LIBBACKTRACE "")  # Will be filled later
set(CONAN_DEFINES_LIBBACKTRACE )
set(CONAN_BUILD_MODULES_PATHS_LIBBACKTRACE )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_LIBBACKTRACE )

set(CONAN_C_FLAGS_LIBBACKTRACE "")
set(CONAN_CXX_FLAGS_LIBBACKTRACE "")
set(CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE "")
set(CONAN_EXE_LINKER_FLAGS_LIBBACKTRACE "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_LIBBACKTRACE_LIST "")
set(CONAN_CXX_FLAGS_LIBBACKTRACE_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_LIST "")
set(CONAN_EXE_LINKER_FLAGS_LIBBACKTRACE_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_LIBBACKTRACE "${CONAN_FRAMEWORKS_LIBBACKTRACE}" "_LIBBACKTRACE" "")
# Append to aggregated values variable
set(CONAN_LIBS_LIBBACKTRACE ${CONAN_PKG_LIBS_LIBBACKTRACE} ${CONAN_SYSTEM_LIBS_LIBBACKTRACE} ${CONAN_FRAMEWORKS_FOUND_LIBBACKTRACE})


#################
###  ZLIB
#################
set(CONAN_ZLIB_ROOT "/home/jeremy/.conan/data/zlib/1.2.11/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7")
set(CONAN_INCLUDE_DIRS_ZLIB "/home/jeremy/.conan/data/zlib/1.2.11/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include")
set(CONAN_LIB_DIRS_ZLIB "/home/jeremy/.conan/data/zlib/1.2.11/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib")
set(CONAN_BIN_DIRS_ZLIB )
set(CONAN_RES_DIRS_ZLIB )
set(CONAN_SRC_DIRS_ZLIB )
set(CONAN_BUILD_DIRS_ZLIB "/home/jeremy/.conan/data/zlib/1.2.11/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/")
set(CONAN_FRAMEWORK_DIRS_ZLIB )
set(CONAN_LIBS_ZLIB z)
set(CONAN_PKG_LIBS_ZLIB z)
set(CONAN_SYSTEM_LIBS_ZLIB )
set(CONAN_FRAMEWORKS_ZLIB )
set(CONAN_FRAMEWORKS_FOUND_ZLIB "")  # Will be filled later
set(CONAN_DEFINES_ZLIB )
set(CONAN_BUILD_MODULES_PATHS_ZLIB )
# COMPILE_DEFINITIONS are equal to CONAN_DEFINES without -D, for targets
set(CONAN_COMPILE_DEFINITIONS_ZLIB )

set(CONAN_C_FLAGS_ZLIB "")
set(CONAN_CXX_FLAGS_ZLIB "")
set(CONAN_SHARED_LINKER_FLAGS_ZLIB "")
set(CONAN_EXE_LINKER_FLAGS_ZLIB "")

# For modern cmake targets we use the list variables (separated with ;)
set(CONAN_C_FLAGS_ZLIB_LIST "")
set(CONAN_CXX_FLAGS_ZLIB_LIST "")
set(CONAN_SHARED_LINKER_FLAGS_ZLIB_LIST "")
set(CONAN_EXE_LINKER_FLAGS_ZLIB_LIST "")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND_ZLIB "${CONAN_FRAMEWORKS_ZLIB}" "_ZLIB" "")
# Append to aggregated values variable
set(CONAN_LIBS_ZLIB ${CONAN_PKG_LIBS_ZLIB} ${CONAN_SYSTEM_LIBS_ZLIB} ${CONAN_FRAMEWORKS_FOUND_ZLIB})


### Definition of global aggregated variables ###

set(CONAN_PACKAGE_NAME None)
set(CONAN_PACKAGE_VERSION None)

set(CONAN_SETTINGS_ARCH "x86_64")
set(CONAN_SETTINGS_BUILD_TYPE "Release")
set(CONAN_SETTINGS_COMPILER "clang")
set(CONAN_SETTINGS_COMPILER_LIBCXX "libstdc++11")
set(CONAN_SETTINGS_COMPILER_VERSION "12")
set(CONAN_SETTINGS_OS "Linux")

set(CONAN_DEPENDENCIES soci hiredis cpr jwt-cpp cpp-httplib inja tomlplusplus yaml-cpp tinyxml rabbitmq-c cppzmq paho-mqtt-cpp cppcodec flatbuffers grpc serial boost sqlite3 libpq libcurl nlohmann_json zeromq paho-mqtt-c protobuf c-ares abseil re2 openssl bzip2 libbacktrace zlib)
# Storing original command line args (CMake helper) flags
set(CONAN_CMD_CXX_FLAGS ${CONAN_CXX_FLAGS})

set(CONAN_CMD_SHARED_LINKER_FLAGS ${CONAN_SHARED_LINKER_FLAGS})
set(CONAN_CMD_C_FLAGS ${CONAN_C_FLAGS})
# Defining accumulated conan variables for all deps

set(CONAN_INCLUDE_DIRS "/home/jeremy/.conan/data/soci/4.0.2/_/_/package/8e517ad41504b15895a219029f4ae6b5963c4c39/include"
			"/home/jeremy/.conan/data/hiredis/1.0.2/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include"
			"/home/jeremy/.conan/data/cpr/1.6.2/_/_/package/955de0a2787b456106824a8ad3b4bc79ea0080c2/include"
			"/home/jeremy/.conan/data/jwt-cpp/0.5.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include"
			"/home/jeremy/.conan/data/cpp-httplib/0.9.5/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include"
			"/home/jeremy/.conan/data/cpp-httplib/0.9.5/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include/httplib"
			"/home/jeremy/.conan/data/inja/3.3.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include"
			"/home/jeremy/.conan/data/tomlplusplus/2.5.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include"
			"/home/jeremy/.conan/data/yaml-cpp/0.7.0/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include"
			"/home/jeremy/.conan/data/tinyxml/2.6.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include"
			"/home/jeremy/.conan/data/rabbitmq-c/0.11.0/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include"
			"/home/jeremy/.conan/data/cppzmq/4.8.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include"
			"/home/jeremy/.conan/data/paho-mqtt-cpp/1.2.0/_/_/package/ee204b5efbf778b3a37ddd291760c795659df13c/include"
			"/home/jeremy/.conan/data/cppcodec/0.2/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include"
			"/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/include"
			"/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/include"
			"/home/jeremy/.conan/data/serial/1.2.1/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include"
			"/home/jeremy/.conan/data/boost/1.77.0/_/_/package/f6db3e065c8e78b76924e832f0eb9cf91494e9ee/include"
			"/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/include"
			"/home/jeremy/.conan/data/libpq/13.3/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include"
			"/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/include"
			"/home/jeremy/.conan/data/nlohmann_json/3.10.3/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/include"
			"/home/jeremy/.conan/data/zeromq/4.3.4/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include"
			"/home/jeremy/.conan/data/paho-mqtt-c/1.3.8/_/_/package/f6647b6445c791c3f4410db4ef533c1e61db8fed/include"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/include"
			"/home/jeremy/.conan/data/c-ares/1.17.1/_/_/package/c2627225292f8d772a40f1bfef7180540cbf7f0d/include"
			"/home/jeremy/.conan/data/abseil/20210324.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include"
			"/home/jeremy/.conan/data/re2/20210601/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/include"
			"/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/include"
			"/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/include"
			"/home/jeremy/.conan/data/libbacktrace/cci.20210118/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include"
			"/home/jeremy/.conan/data/zlib/1.2.11/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/include" ${CONAN_INCLUDE_DIRS})
set(CONAN_LIB_DIRS "/home/jeremy/.conan/data/soci/4.0.2/_/_/package/8e517ad41504b15895a219029f4ae6b5963c4c39/lib"
			"/home/jeremy/.conan/data/hiredis/1.0.2/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib"
			"/home/jeremy/.conan/data/cpr/1.6.2/_/_/package/955de0a2787b456106824a8ad3b4bc79ea0080c2/lib"
			"/home/jeremy/.conan/data/yaml-cpp/0.7.0/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib"
			"/home/jeremy/.conan/data/tinyxml/2.6.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib"
			"/home/jeremy/.conan/data/rabbitmq-c/0.11.0/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib"
			"/home/jeremy/.conan/data/cppzmq/4.8.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/lib"
			"/home/jeremy/.conan/data/paho-mqtt-cpp/1.2.0/_/_/package/ee204b5efbf778b3a37ddd291760c795659df13c/lib"
			"/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/lib"
			"/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/lib"
			"/home/jeremy/.conan/data/serial/1.2.1/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib"
			"/home/jeremy/.conan/data/boost/1.77.0/_/_/package/f6db3e065c8e78b76924e832f0eb9cf91494e9ee/lib"
			"/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/lib"
			"/home/jeremy/.conan/data/libpq/13.3/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib"
			"/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/lib"
			"/home/jeremy/.conan/data/zeromq/4.3.4/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib"
			"/home/jeremy/.conan/data/paho-mqtt-c/1.3.8/_/_/package/f6647b6445c791c3f4410db4ef533c1e61db8fed/lib"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib"
			"/home/jeremy/.conan/data/c-ares/1.17.1/_/_/package/c2627225292f8d772a40f1bfef7180540cbf7f0d/lib"
			"/home/jeremy/.conan/data/abseil/20210324.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib"
			"/home/jeremy/.conan/data/re2/20210601/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib"
			"/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/lib"
			"/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/lib"
			"/home/jeremy/.conan/data/libbacktrace/cci.20210118/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib"
			"/home/jeremy/.conan/data/zlib/1.2.11/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/lib" ${CONAN_LIB_DIRS})
set(CONAN_BIN_DIRS "/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/bin"
			"/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/bin"
			"/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/bin"
			"/home/jeremy/.conan/data/libpq/13.3/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/bin"
			"/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/bin"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/bin"
			"/home/jeremy/.conan/data/c-ares/1.17.1/_/_/package/c2627225292f8d772a40f1bfef7180540cbf7f0d/bin"
			"/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/bin"
			"/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/bin" ${CONAN_BIN_DIRS})
set(CONAN_RES_DIRS "/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/res" ${CONAN_RES_DIRS})
set(CONAN_FRAMEWORK_DIRS  ${CONAN_FRAMEWORK_DIRS})
set(CONAN_LIBS soci_empty soci_sqlite3 soci_postgresql soci_core hiredis cpr yaml-cpp tinyxml rabbitmq paho-mqttpp3 flatbuffers grpc++_alts grpc++_error_details grpc++_reflection grpc++ grpc++_unsecure grpc_unsecure grpcpp_channelz grpc address_sorting gpr upb grpc_plugin_support serial boost_contract boost_coroutine boost_fiber_numa boost_fiber boost_context boost_graph boost_iostreams boost_json boost_locale boost_log_setup boost_log boost_math_c99 boost_math_c99f boost_math_c99l boost_math_tr1 boost_math_tr1f boost_math_tr1l boost_nowide boost_program_options boost_random boost_regex boost_stacktrace_addr2line boost_stacktrace_backtrace boost_stacktrace_basic boost_stacktrace_noop boost_timer boost_type_erasure boost_thread boost_chrono boost_container boost_date_time boost_unit_test_framework boost_prg_exec_monitor boost_test_exec_monitor boost_exception boost_wave boost_filesystem boost_atomic boost_wserialization boost_serialization sqlite3 pq pgcommon pgcommon_shlib pgport pgport_shlib curl zmq paho-mqtt3as protoc protobuf cares absl_periodic_sampler absl_scoped_set_env absl_strerror absl_failure_signal_handler absl_examine_stack absl_leak_check_disable absl_leak_check absl_flags_parse absl_flags_usage absl_flags_usage_internal absl_flags absl_flags_reflection absl_raw_hash_set absl_hashtablez_sampler absl_exponential_biased absl_flags_private_handle_accessor absl_flags_internal absl_flags_config absl_flags_program_name absl_flags_marshalling absl_flags_commandlineflag absl_flags_commandlineflag_internal absl_hash absl_city absl_wyhash absl_random_distributions absl_random_seed_sequences absl_random_internal_pool_urbg absl_random_seed_gen_exception absl_random_internal_seed_material absl_random_internal_randen absl_random_internal_randen_slow absl_random_internal_randen_hwaes absl_random_internal_randen_hwaes_impl absl_random_internal_platform absl_random_internal_distribution_test_util absl_statusor absl_status absl_str_format_internal absl_cord absl_synchronization absl_stacktrace absl_symbolize absl_debugging_internal absl_demangle_internal absl_graphcycles_internal absl_malloc_internal absl_time absl_strings absl_int128 absl_strings_internal absl_base absl_spinlock_wait absl_civil_time absl_time_zone absl_bad_any_cast_impl absl_throw_delegate absl_bad_optional_access absl_bad_variant_access absl_raw_logging_internal absl_log_severity re2 ssl crypto bz2 backtrace z ${CONAN_LIBS})
set(CONAN_PKG_LIBS soci_empty soci_sqlite3 soci_postgresql soci_core hiredis cpr yaml-cpp tinyxml rabbitmq paho-mqttpp3 flatbuffers grpc++_alts grpc++_error_details grpc++_reflection grpc++ grpc++_unsecure grpc_unsecure grpcpp_channelz grpc address_sorting gpr upb grpc_plugin_support serial boost_contract boost_coroutine boost_fiber_numa boost_fiber boost_context boost_graph boost_iostreams boost_json boost_locale boost_log_setup boost_log boost_math_c99 boost_math_c99f boost_math_c99l boost_math_tr1 boost_math_tr1f boost_math_tr1l boost_nowide boost_program_options boost_random boost_regex boost_stacktrace_addr2line boost_stacktrace_backtrace boost_stacktrace_basic boost_stacktrace_noop boost_timer boost_type_erasure boost_thread boost_chrono boost_container boost_date_time boost_unit_test_framework boost_prg_exec_monitor boost_test_exec_monitor boost_exception boost_wave boost_filesystem boost_atomic boost_wserialization boost_serialization sqlite3 pq pgcommon pgcommon_shlib pgport pgport_shlib curl zmq paho-mqtt3as protoc protobuf cares absl_periodic_sampler absl_scoped_set_env absl_strerror absl_failure_signal_handler absl_examine_stack absl_leak_check_disable absl_leak_check absl_flags_parse absl_flags_usage absl_flags_usage_internal absl_flags absl_flags_reflection absl_raw_hash_set absl_hashtablez_sampler absl_exponential_biased absl_flags_private_handle_accessor absl_flags_internal absl_flags_config absl_flags_program_name absl_flags_marshalling absl_flags_commandlineflag absl_flags_commandlineflag_internal absl_hash absl_city absl_wyhash absl_random_distributions absl_random_seed_sequences absl_random_internal_pool_urbg absl_random_seed_gen_exception absl_random_internal_seed_material absl_random_internal_randen absl_random_internal_randen_slow absl_random_internal_randen_hwaes absl_random_internal_randen_hwaes_impl absl_random_internal_platform absl_random_internal_distribution_test_util absl_statusor absl_status absl_str_format_internal absl_cord absl_synchronization absl_stacktrace absl_symbolize absl_debugging_internal absl_demangle_internal absl_graphcycles_internal absl_malloc_internal absl_time absl_strings absl_int128 absl_strings_internal absl_base absl_spinlock_wait absl_civil_time absl_time_zone absl_bad_any_cast_impl absl_throw_delegate absl_bad_optional_access absl_bad_variant_access absl_raw_logging_internal absl_log_severity re2 ssl crypto bz2 backtrace z ${CONAN_PKG_LIBS})
set(CONAN_SYSTEM_LIBS c m dl pthread rt ${CONAN_SYSTEM_LIBS})
set(CONAN_FRAMEWORKS  ${CONAN_FRAMEWORKS})
set(CONAN_FRAMEWORKS_FOUND "")  # Will be filled later
set(CONAN_DEFINES "-DCARES_STATICLIB"
			"-DZMQ_STATIC"
			"-DCURL_STATICLIB=1"
			"-DBOOST_STACKTRACE_ADDR2LINE_LOCATION=\"/usr/bin/addr2line\""
			"-DBOOST_STACKTRACE_USE_ADDR2LINE"
			"-DBOOST_STACKTRACE_USE_BACKTRACE"
			"-DBOOST_STACKTRACE_USE_NOOP"
			"-DAMQP_STATIC" ${CONAN_DEFINES})
set(CONAN_BUILD_MODULES_PATHS "/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/bin/cmake/BuildFlatBuffers.cmake"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib/cmake/protobuf/protobuf-generate.cmake"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib/cmake/protobuf/protobuf-module.cmake"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib/cmake/protobuf/protobuf-options.cmake" ${CONAN_BUILD_MODULES_PATHS})
set(CONAN_CMAKE_MODULE_PATH "/home/jeremy/.conan/data/soci/4.0.2/_/_/package/8e517ad41504b15895a219029f4ae6b5963c4c39/"
			"/home/jeremy/.conan/data/hiredis/1.0.2/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/"
			"/home/jeremy/.conan/data/cpr/1.6.2/_/_/package/955de0a2787b456106824a8ad3b4bc79ea0080c2/"
			"/home/jeremy/.conan/data/jwt-cpp/0.5.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/"
			"/home/jeremy/.conan/data/cpp-httplib/0.9.5/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/"
			"/home/jeremy/.conan/data/inja/3.3.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/"
			"/home/jeremy/.conan/data/tomlplusplus/2.5.0/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/"
			"/home/jeremy/.conan/data/yaml-cpp/0.7.0/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/"
			"/home/jeremy/.conan/data/yaml-cpp/0.7.0/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/lib/cmake"
			"/home/jeremy/.conan/data/tinyxml/2.6.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/"
			"/home/jeremy/.conan/data/rabbitmq-c/0.11.0/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/"
			"/home/jeremy/.conan/data/cppzmq/4.8.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/"
			"/home/jeremy/.conan/data/cppzmq/4.8.1/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/lib/cmake"
			"/home/jeremy/.conan/data/paho-mqtt-cpp/1.2.0/_/_/package/ee204b5efbf778b3a37ddd291760c795659df13c/"
			"/home/jeremy/.conan/data/cppcodec/0.2/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/"
			"/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/"
			"/home/jeremy/.conan/data/flatbuffers/2.0.0/_/_/package/fca37b4cbc75715385f9115a0fd9d343f44798cf/bin/cmake"
			"/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/"
			"/home/jeremy/.conan/data/grpc/1.39.1/_/_/package/8835978170244bbc1a72106955c26b2c147925a1/lib/cmake"
			"/home/jeremy/.conan/data/serial/1.2.1/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/"
			"/home/jeremy/.conan/data/boost/1.77.0/_/_/package/f6db3e065c8e78b76924e832f0eb9cf91494e9ee/"
			"/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/"
			"/home/jeremy/.conan/data/sqlite3/3.36.0/_/_/package/024b8b7ce26d8c2a5b8c3015baef8593826a8b65/lib/cmake"
			"/home/jeremy/.conan/data/libpq/13.3/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/"
			"/home/jeremy/.conan/data/libcurl/7.79.0/_/_/package/1c9259f5bd2d727507634c0420b5dbba539fad95/"
			"/home/jeremy/.conan/data/nlohmann_json/3.10.3/_/_/package/5ab84d6acfe1f23c4fae0ab88f26e3a396351ac9/"
			"/home/jeremy/.conan/data/zeromq/4.3.4/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/"
			"/home/jeremy/.conan/data/paho-mqtt-c/1.3.8/_/_/package/f6647b6445c791c3f4410db4ef533c1e61db8fed/"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/"
			"/home/jeremy/.conan/data/protobuf/3.17.1/_/_/package/c66c4b06a13a7733f8858e67c7066aa23657cff6/lib/cmake/protobuf"
			"/home/jeremy/.conan/data/c-ares/1.17.1/_/_/package/c2627225292f8d772a40f1bfef7180540cbf7f0d/"
			"/home/jeremy/.conan/data/abseil/20210324.2/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/"
			"/home/jeremy/.conan/data/re2/20210601/_/_/package/5cf68078cd94bae1d7345465f54e4f435eecb8e5/"
			"/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/"
			"/home/jeremy/.conan/data/openssl/3.0.0/_/_/package/7c8ca535f58f42069ca466da60b5c61f4688554c/lib/cmake"
			"/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/"
			"/home/jeremy/.conan/data/bzip2/1.0.8/_/_/package/00c750ce60d90d17c6eb9a941db66d67f523d38f/lib/cmake"
			"/home/jeremy/.conan/data/libbacktrace/cci.20210118/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/"
			"/home/jeremy/.conan/data/zlib/1.2.11/_/_/package/cf142e38e022cf2a99aadebfd8f37ead3e935ad7/" ${CONAN_CMAKE_MODULE_PATH})

set(CONAN_CXX_FLAGS " ${CONAN_CXX_FLAGS}")
set(CONAN_SHARED_LINKER_FLAGS " ${CONAN_SHARED_LINKER_FLAGS}")
set(CONAN_EXE_LINKER_FLAGS " ${CONAN_EXE_LINKER_FLAGS}")
set(CONAN_C_FLAGS " ${CONAN_C_FLAGS}")

# Apple Frameworks
conan_find_apple_frameworks(CONAN_FRAMEWORKS_FOUND "${CONAN_FRAMEWORKS}" "" "")
# Append to aggregated values variable: Use CONAN_LIBS instead of CONAN_PKG_LIBS to include user appended vars
set(CONAN_LIBS ${CONAN_LIBS} ${CONAN_SYSTEM_LIBS} ${CONAN_FRAMEWORKS_FOUND})


###  Definition of macros and functions ###

macro(conan_define_targets)
    if(${CMAKE_VERSION} VERSION_LESS "3.1.2")
        message(FATAL_ERROR "TARGETS not supported by your CMake version!")
    endif()  # CMAKE > 3.x
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${CONAN_CMD_CXX_FLAGS}")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${CONAN_CMD_C_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} ${CONAN_CMD_SHARED_LINKER_FLAGS}")


    set(_CONAN_PKG_LIBS_SOCI_DEPENDENCIES "${CONAN_SYSTEM_LIBS_SOCI} ${CONAN_FRAMEWORKS_FOUND_SOCI} CONAN_PKG::sqlite3 CONAN_PKG::libpq CONAN_PKG::boost")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SOCI_DEPENDENCIES "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SOCI}" "${CONAN_LIB_DIRS_SOCI}"
                                  CONAN_PACKAGE_TARGETS_SOCI "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES}"
                                  "" soci)
    set(_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_SOCI_DEBUG} ${CONAN_FRAMEWORKS_FOUND_SOCI_DEBUG} CONAN_PKG::sqlite3 CONAN_PKG::libpq CONAN_PKG::boost")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SOCI_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SOCI_DEBUG}" "${CONAN_LIB_DIRS_SOCI_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_SOCI_DEBUG "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_DEBUG}"
                                  "debug" soci)
    set(_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_SOCI_RELEASE} ${CONAN_FRAMEWORKS_FOUND_SOCI_RELEASE} CONAN_PKG::sqlite3 CONAN_PKG::libpq CONAN_PKG::boost")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SOCI_RELEASE}" "${CONAN_LIB_DIRS_SOCI_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_SOCI_RELEASE "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELEASE}"
                                  "release" soci)
    set(_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_SOCI_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_SOCI_RELWITHDEBINFO} CONAN_PKG::sqlite3 CONAN_PKG::libpq CONAN_PKG::boost")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SOCI_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_SOCI_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_SOCI_RELWITHDEBINFO "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" soci)
    set(_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_SOCI_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_SOCI_MINSIZEREL} CONAN_PKG::sqlite3 CONAN_PKG::libpq CONAN_PKG::boost")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SOCI_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SOCI_MINSIZEREL}" "${CONAN_LIB_DIRS_SOCI_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_SOCI_MINSIZEREL "${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" soci)

    add_library(CONAN_PKG::soci INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::soci PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_SOCI} ${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SOCI_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_SOCI_RELEASE} ${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SOCI_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_SOCI_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SOCI_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_SOCI_MINSIZEREL} ${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SOCI_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_SOCI_DEBUG} ${_CONAN_PKG_LIBS_SOCI_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SOCI_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SOCI_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::soci PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_SOCI}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_SOCI_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_SOCI_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_SOCI_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_SOCI_DEBUG}>)
    set_property(TARGET CONAN_PKG::soci PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_SOCI}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_SOCI_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_SOCI_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_SOCI_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_SOCI_DEBUG}>)
    set_property(TARGET CONAN_PKG::soci PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_SOCI_LIST} ${CONAN_CXX_FLAGS_SOCI_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_SOCI_RELEASE_LIST} ${CONAN_CXX_FLAGS_SOCI_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_SOCI_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_SOCI_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_SOCI_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_SOCI_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_SOCI_DEBUG_LIST}  ${CONAN_CXX_FLAGS_SOCI_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES "${CONAN_SYSTEM_LIBS_HIREDIS} ${CONAN_FRAMEWORKS_FOUND_HIREDIS} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_HIREDIS}" "${CONAN_LIB_DIRS_HIREDIS}"
                                  CONAN_PACKAGE_TARGETS_HIREDIS "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES}"
                                  "" hiredis)
    set(_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_HIREDIS_DEBUG} ${CONAN_FRAMEWORKS_FOUND_HIREDIS_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_HIREDIS_DEBUG}" "${CONAN_LIB_DIRS_HIREDIS_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_HIREDIS_DEBUG "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_DEBUG}"
                                  "debug" hiredis)
    set(_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_HIREDIS_RELEASE} ${CONAN_FRAMEWORKS_FOUND_HIREDIS_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_HIREDIS_RELEASE}" "${CONAN_LIB_DIRS_HIREDIS_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_HIREDIS_RELEASE "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELEASE}"
                                  "release" hiredis)
    set(_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_HIREDIS_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_HIREDIS_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_HIREDIS_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_HIREDIS_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_HIREDIS_RELWITHDEBINFO "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" hiredis)
    set(_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_HIREDIS_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_HIREDIS_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_HIREDIS_MINSIZEREL}" "${CONAN_LIB_DIRS_HIREDIS_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_HIREDIS_MINSIZEREL "${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" hiredis)

    add_library(CONAN_PKG::hiredis INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::hiredis PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_HIREDIS} ${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_HIREDIS_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_HIREDIS_RELEASE} ${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_HIREDIS_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_HIREDIS_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_HIREDIS_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_HIREDIS_MINSIZEREL} ${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_HIREDIS_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_HIREDIS_DEBUG} ${_CONAN_PKG_LIBS_HIREDIS_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_HIREDIS_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_HIREDIS_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::hiredis PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_HIREDIS}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_HIREDIS_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_HIREDIS_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_HIREDIS_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_HIREDIS_DEBUG}>)
    set_property(TARGET CONAN_PKG::hiredis PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_HIREDIS}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_HIREDIS_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_HIREDIS_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_HIREDIS_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_HIREDIS_DEBUG}>)
    set_property(TARGET CONAN_PKG::hiredis PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_HIREDIS_LIST} ${CONAN_CXX_FLAGS_HIREDIS_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_HIREDIS_RELEASE_LIST} ${CONAN_CXX_FLAGS_HIREDIS_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_HIREDIS_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_HIREDIS_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_HIREDIS_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_HIREDIS_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_HIREDIS_DEBUG_LIST}  ${CONAN_CXX_FLAGS_HIREDIS_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_CPR_DEPENDENCIES "${CONAN_SYSTEM_LIBS_CPR} ${CONAN_FRAMEWORKS_FOUND_CPR} CONAN_PKG::libcurl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPR_DEPENDENCIES "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPR}" "${CONAN_LIB_DIRS_CPR}"
                                  CONAN_PACKAGE_TARGETS_CPR "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES}"
                                  "" cpr)
    set(_CONAN_PKG_LIBS_CPR_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_CPR_DEBUG} ${CONAN_FRAMEWORKS_FOUND_CPR_DEBUG} CONAN_PKG::libcurl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPR_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPR_DEBUG}" "${CONAN_LIB_DIRS_CPR_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_CPR_DEBUG "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_DEBUG}"
                                  "debug" cpr)
    set(_CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_CPR_RELEASE} ${CONAN_FRAMEWORKS_FOUND_CPR_RELEASE} CONAN_PKG::libcurl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPR_RELEASE}" "${CONAN_LIB_DIRS_CPR_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_CPR_RELEASE "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELEASE}"
                                  "release" cpr)
    set(_CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_CPR_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_CPR_RELWITHDEBINFO} CONAN_PKG::libcurl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPR_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_CPR_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_CPR_RELWITHDEBINFO "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" cpr)
    set(_CONAN_PKG_LIBS_CPR_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_CPR_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_CPR_MINSIZEREL} CONAN_PKG::libcurl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPR_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPR_MINSIZEREL}" "${CONAN_LIB_DIRS_CPR_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_CPR_MINSIZEREL "${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" cpr)

    add_library(CONAN_PKG::cpr INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::cpr PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_CPR} ${_CONAN_PKG_LIBS_CPR_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPR_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_CPR_RELEASE} ${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPR_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_CPR_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPR_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_CPR_MINSIZEREL} ${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPR_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_CPR_DEBUG} ${_CONAN_PKG_LIBS_CPR_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPR_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPR_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::cpr PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_CPR}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_CPR_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_CPR_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_CPR_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_CPR_DEBUG}>)
    set_property(TARGET CONAN_PKG::cpr PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_CPR}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_CPR_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_CPR_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_CPR_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_CPR_DEBUG}>)
    set_property(TARGET CONAN_PKG::cpr PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_CPR_LIST} ${CONAN_CXX_FLAGS_CPR_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_CPR_RELEASE_LIST} ${CONAN_CXX_FLAGS_CPR_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_CPR_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_CPR_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_CPR_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_CPR_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_CPR_DEBUG_LIST}  ${CONAN_CXX_FLAGS_CPR_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES "${CONAN_SYSTEM_LIBS_JWT-CPP} ${CONAN_FRAMEWORKS_FOUND_JWT-CPP} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_JWT-CPP}" "${CONAN_LIB_DIRS_JWT-CPP}"
                                  CONAN_PACKAGE_TARGETS_JWT-CPP "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES}"
                                  "" jwt-cpp)
    set(_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_JWT-CPP_DEBUG} ${CONAN_FRAMEWORKS_FOUND_JWT-CPP_DEBUG} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_JWT-CPP_DEBUG}" "${CONAN_LIB_DIRS_JWT-CPP_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_JWT-CPP_DEBUG "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_DEBUG}"
                                  "debug" jwt-cpp)
    set(_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_JWT-CPP_RELEASE} ${CONAN_FRAMEWORKS_FOUND_JWT-CPP_RELEASE} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_JWT-CPP_RELEASE}" "${CONAN_LIB_DIRS_JWT-CPP_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_JWT-CPP_RELEASE "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELEASE}"
                                  "release" jwt-cpp)
    set(_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_JWT-CPP_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_JWT-CPP_RELWITHDEBINFO} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_JWT-CPP_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_JWT-CPP_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_JWT-CPP_RELWITHDEBINFO "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" jwt-cpp)
    set(_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_JWT-CPP_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_JWT-CPP_MINSIZEREL} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_JWT-CPP_MINSIZEREL}" "${CONAN_LIB_DIRS_JWT-CPP_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_JWT-CPP_MINSIZEREL "${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" jwt-cpp)

    add_library(CONAN_PKG::jwt-cpp INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::jwt-cpp PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_JWT-CPP} ${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_JWT-CPP_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_JWT-CPP_RELEASE} ${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_JWT-CPP_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_JWT-CPP_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_JWT-CPP_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_JWT-CPP_MINSIZEREL} ${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_JWT-CPP_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_JWT-CPP_DEBUG} ${_CONAN_PKG_LIBS_JWT-CPP_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_JWT-CPP_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_JWT-CPP_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::jwt-cpp PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_JWT-CPP}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_JWT-CPP_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_JWT-CPP_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_JWT-CPP_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_JWT-CPP_DEBUG}>)
    set_property(TARGET CONAN_PKG::jwt-cpp PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_JWT-CPP}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_JWT-CPP_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_JWT-CPP_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_JWT-CPP_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_JWT-CPP_DEBUG}>)
    set_property(TARGET CONAN_PKG::jwt-cpp PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_JWT-CPP_LIST} ${CONAN_CXX_FLAGS_JWT-CPP_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_JWT-CPP_RELEASE_LIST} ${CONAN_CXX_FLAGS_JWT-CPP_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_JWT-CPP_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_JWT-CPP_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_JWT-CPP_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_JWT-CPP_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_JWT-CPP_DEBUG_LIST}  ${CONAN_CXX_FLAGS_JWT-CPP_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES "${CONAN_SYSTEM_LIBS_CPP-HTTPLIB} ${CONAN_FRAMEWORKS_FOUND_CPP-HTTPLIB} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPP-HTTPLIB}" "${CONAN_LIB_DIRS_CPP-HTTPLIB}"
                                  CONAN_PACKAGE_TARGETS_CPP-HTTPLIB "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES}"
                                  "" cpp-httplib)
    set(_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_CPP-HTTPLIB_DEBUG} ${CONAN_FRAMEWORKS_FOUND_CPP-HTTPLIB_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPP-HTTPLIB_DEBUG}" "${CONAN_LIB_DIRS_CPP-HTTPLIB_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_CPP-HTTPLIB_DEBUG "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_DEBUG}"
                                  "debug" cpp-httplib)
    set(_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_CPP-HTTPLIB_RELEASE} ${CONAN_FRAMEWORKS_FOUND_CPP-HTTPLIB_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPP-HTTPLIB_RELEASE}" "${CONAN_LIB_DIRS_CPP-HTTPLIB_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_CPP-HTTPLIB_RELEASE "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELEASE}"
                                  "release" cpp-httplib)
    set(_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_CPP-HTTPLIB_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_CPP-HTTPLIB_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPP-HTTPLIB_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_CPP-HTTPLIB_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_CPP-HTTPLIB_RELWITHDEBINFO "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" cpp-httplib)
    set(_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_CPP-HTTPLIB_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_CPP-HTTPLIB_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPP-HTTPLIB_MINSIZEREL}" "${CONAN_LIB_DIRS_CPP-HTTPLIB_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_CPP-HTTPLIB_MINSIZEREL "${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" cpp-httplib)

    add_library(CONAN_PKG::cpp-httplib INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::cpp-httplib PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_CPP-HTTPLIB} ${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPP-HTTPLIB_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_CPP-HTTPLIB_RELEASE} ${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPP-HTTPLIB_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_CPP-HTTPLIB_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPP-HTTPLIB_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_CPP-HTTPLIB_MINSIZEREL} ${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPP-HTTPLIB_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_CPP-HTTPLIB_DEBUG} ${_CONAN_PKG_LIBS_CPP-HTTPLIB_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPP-HTTPLIB_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPP-HTTPLIB_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::cpp-httplib PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_CPP-HTTPLIB}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_CPP-HTTPLIB_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_CPP-HTTPLIB_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_CPP-HTTPLIB_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_CPP-HTTPLIB_DEBUG}>)
    set_property(TARGET CONAN_PKG::cpp-httplib PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_CPP-HTTPLIB}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_CPP-HTTPLIB_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_CPP-HTTPLIB_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_CPP-HTTPLIB_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_CPP-HTTPLIB_DEBUG}>)
    set_property(TARGET CONAN_PKG::cpp-httplib PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_CPP-HTTPLIB_LIST} ${CONAN_CXX_FLAGS_CPP-HTTPLIB_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_CPP-HTTPLIB_RELEASE_LIST} ${CONAN_CXX_FLAGS_CPP-HTTPLIB_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_CPP-HTTPLIB_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_CPP-HTTPLIB_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_CPP-HTTPLIB_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_CPP-HTTPLIB_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_CPP-HTTPLIB_DEBUG_LIST}  ${CONAN_CXX_FLAGS_CPP-HTTPLIB_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_INJA_DEPENDENCIES "${CONAN_SYSTEM_LIBS_INJA} ${CONAN_FRAMEWORKS_FOUND_INJA} CONAN_PKG::nlohmann_json")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_INJA_DEPENDENCIES "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_INJA}" "${CONAN_LIB_DIRS_INJA}"
                                  CONAN_PACKAGE_TARGETS_INJA "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES}"
                                  "" inja)
    set(_CONAN_PKG_LIBS_INJA_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_INJA_DEBUG} ${CONAN_FRAMEWORKS_FOUND_INJA_DEBUG} CONAN_PKG::nlohmann_json")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_INJA_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_INJA_DEBUG}" "${CONAN_LIB_DIRS_INJA_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_INJA_DEBUG "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_DEBUG}"
                                  "debug" inja)
    set(_CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_INJA_RELEASE} ${CONAN_FRAMEWORKS_FOUND_INJA_RELEASE} CONAN_PKG::nlohmann_json")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_INJA_RELEASE}" "${CONAN_LIB_DIRS_INJA_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_INJA_RELEASE "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELEASE}"
                                  "release" inja)
    set(_CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_INJA_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_INJA_RELWITHDEBINFO} CONAN_PKG::nlohmann_json")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_INJA_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_INJA_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_INJA_RELWITHDEBINFO "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" inja)
    set(_CONAN_PKG_LIBS_INJA_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_INJA_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_INJA_MINSIZEREL} CONAN_PKG::nlohmann_json")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_INJA_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_INJA_MINSIZEREL}" "${CONAN_LIB_DIRS_INJA_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_INJA_MINSIZEREL "${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" inja)

    add_library(CONAN_PKG::inja INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::inja PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_INJA} ${_CONAN_PKG_LIBS_INJA_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_INJA_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_INJA_RELEASE} ${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_INJA_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_INJA_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_INJA_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_INJA_MINSIZEREL} ${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_INJA_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_INJA_DEBUG} ${_CONAN_PKG_LIBS_INJA_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_INJA_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_INJA_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::inja PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_INJA}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_INJA_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_INJA_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_INJA_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_INJA_DEBUG}>)
    set_property(TARGET CONAN_PKG::inja PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_INJA}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_INJA_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_INJA_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_INJA_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_INJA_DEBUG}>)
    set_property(TARGET CONAN_PKG::inja PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_INJA_LIST} ${CONAN_CXX_FLAGS_INJA_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_INJA_RELEASE_LIST} ${CONAN_CXX_FLAGS_INJA_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_INJA_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_INJA_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_INJA_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_INJA_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_INJA_DEBUG_LIST}  ${CONAN_CXX_FLAGS_INJA_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES "${CONAN_SYSTEM_LIBS_TOMLPLUSPLUS} ${CONAN_FRAMEWORKS_FOUND_TOMLPLUSPLUS} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TOMLPLUSPLUS}" "${CONAN_LIB_DIRS_TOMLPLUSPLUS}"
                                  CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES}"
                                  "" tomlplusplus)
    set(_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_TOMLPLUSPLUS_DEBUG} ${CONAN_FRAMEWORKS_FOUND_TOMLPLUSPLUS_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TOMLPLUSPLUS_DEBUG}" "${CONAN_LIB_DIRS_TOMLPLUSPLUS_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS_DEBUG "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_DEBUG}"
                                  "debug" tomlplusplus)
    set(_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_TOMLPLUSPLUS_RELEASE} ${CONAN_FRAMEWORKS_FOUND_TOMLPLUSPLUS_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TOMLPLUSPLUS_RELEASE}" "${CONAN_LIB_DIRS_TOMLPLUSPLUS_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS_RELEASE "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELEASE}"
                                  "release" tomlplusplus)
    set(_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_TOMLPLUSPLUS_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_TOMLPLUSPLUS_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TOMLPLUSPLUS_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_TOMLPLUSPLUS_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS_RELWITHDEBINFO "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" tomlplusplus)
    set(_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_TOMLPLUSPLUS_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_TOMLPLUSPLUS_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TOMLPLUSPLUS_MINSIZEREL}" "${CONAN_LIB_DIRS_TOMLPLUSPLUS_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS_MINSIZEREL "${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" tomlplusplus)

    add_library(CONAN_PKG::tomlplusplus INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::tomlplusplus PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS} ${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TOMLPLUSPLUS_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS_RELEASE} ${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TOMLPLUSPLUS_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TOMLPLUSPLUS_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS_MINSIZEREL} ${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TOMLPLUSPLUS_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_TOMLPLUSPLUS_DEBUG} ${_CONAN_PKG_LIBS_TOMLPLUSPLUS_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TOMLPLUSPLUS_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TOMLPLUSPLUS_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::tomlplusplus PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_TOMLPLUSPLUS}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_TOMLPLUSPLUS_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_TOMLPLUSPLUS_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_TOMLPLUSPLUS_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_TOMLPLUSPLUS_DEBUG}>)
    set_property(TARGET CONAN_PKG::tomlplusplus PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_TOMLPLUSPLUS}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_TOMLPLUSPLUS_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_TOMLPLUSPLUS_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_TOMLPLUSPLUS_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_TOMLPLUSPLUS_DEBUG}>)
    set_property(TARGET CONAN_PKG::tomlplusplus PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_TOMLPLUSPLUS_LIST} ${CONAN_CXX_FLAGS_TOMLPLUSPLUS_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_TOMLPLUSPLUS_RELEASE_LIST} ${CONAN_CXX_FLAGS_TOMLPLUSPLUS_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_TOMLPLUSPLUS_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_TOMLPLUSPLUS_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_TOMLPLUSPLUS_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_TOMLPLUSPLUS_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_TOMLPLUSPLUS_DEBUG_LIST}  ${CONAN_CXX_FLAGS_TOMLPLUSPLUS_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES "${CONAN_SYSTEM_LIBS_YAML-CPP} ${CONAN_FRAMEWORKS_FOUND_YAML-CPP} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_YAML-CPP}" "${CONAN_LIB_DIRS_YAML-CPP}"
                                  CONAN_PACKAGE_TARGETS_YAML-CPP "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES}"
                                  "" yaml-cpp)
    set(_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_YAML-CPP_DEBUG} ${CONAN_FRAMEWORKS_FOUND_YAML-CPP_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_YAML-CPP_DEBUG}" "${CONAN_LIB_DIRS_YAML-CPP_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_YAML-CPP_DEBUG "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_DEBUG}"
                                  "debug" yaml-cpp)
    set(_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_YAML-CPP_RELEASE} ${CONAN_FRAMEWORKS_FOUND_YAML-CPP_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_YAML-CPP_RELEASE}" "${CONAN_LIB_DIRS_YAML-CPP_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_YAML-CPP_RELEASE "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELEASE}"
                                  "release" yaml-cpp)
    set(_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_YAML-CPP_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_YAML-CPP_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_YAML-CPP_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_YAML-CPP_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_YAML-CPP_RELWITHDEBINFO "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" yaml-cpp)
    set(_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_YAML-CPP_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_YAML-CPP_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_YAML-CPP_MINSIZEREL}" "${CONAN_LIB_DIRS_YAML-CPP_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_YAML-CPP_MINSIZEREL "${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" yaml-cpp)

    add_library(CONAN_PKG::yaml-cpp INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::yaml-cpp PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_YAML-CPP} ${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_YAML-CPP_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_YAML-CPP_RELEASE} ${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_YAML-CPP_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_YAML-CPP_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_YAML-CPP_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_YAML-CPP_MINSIZEREL} ${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_YAML-CPP_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_YAML-CPP_DEBUG} ${_CONAN_PKG_LIBS_YAML-CPP_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_YAML-CPP_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_YAML-CPP_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::yaml-cpp PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_YAML-CPP}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_YAML-CPP_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_YAML-CPP_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_YAML-CPP_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_YAML-CPP_DEBUG}>)
    set_property(TARGET CONAN_PKG::yaml-cpp PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_YAML-CPP}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_YAML-CPP_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_YAML-CPP_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_YAML-CPP_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_YAML-CPP_DEBUG}>)
    set_property(TARGET CONAN_PKG::yaml-cpp PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_YAML-CPP_LIST} ${CONAN_CXX_FLAGS_YAML-CPP_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_YAML-CPP_RELEASE_LIST} ${CONAN_CXX_FLAGS_YAML-CPP_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_YAML-CPP_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_YAML-CPP_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_YAML-CPP_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_YAML-CPP_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_YAML-CPP_DEBUG_LIST}  ${CONAN_CXX_FLAGS_YAML-CPP_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES "${CONAN_SYSTEM_LIBS_TINYXML} ${CONAN_FRAMEWORKS_FOUND_TINYXML} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TINYXML_DEPENDENCIES "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TINYXML}" "${CONAN_LIB_DIRS_TINYXML}"
                                  CONAN_PACKAGE_TARGETS_TINYXML "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES}"
                                  "" tinyxml)
    set(_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_TINYXML_DEBUG} ${CONAN_FRAMEWORKS_FOUND_TINYXML_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TINYXML_DEBUG}" "${CONAN_LIB_DIRS_TINYXML_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_TINYXML_DEBUG "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_DEBUG}"
                                  "debug" tinyxml)
    set(_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_TINYXML_RELEASE} ${CONAN_FRAMEWORKS_FOUND_TINYXML_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TINYXML_RELEASE}" "${CONAN_LIB_DIRS_TINYXML_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_TINYXML_RELEASE "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELEASE}"
                                  "release" tinyxml)
    set(_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_TINYXML_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_TINYXML_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TINYXML_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_TINYXML_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_TINYXML_RELWITHDEBINFO "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" tinyxml)
    set(_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_TINYXML_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_TINYXML_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_TINYXML_MINSIZEREL}" "${CONAN_LIB_DIRS_TINYXML_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_TINYXML_MINSIZEREL "${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" tinyxml)

    add_library(CONAN_PKG::tinyxml INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::tinyxml PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_TINYXML} ${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TINYXML_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_TINYXML_RELEASE} ${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TINYXML_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_TINYXML_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TINYXML_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_TINYXML_MINSIZEREL} ${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TINYXML_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_TINYXML_DEBUG} ${_CONAN_PKG_LIBS_TINYXML_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_TINYXML_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_TINYXML_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::tinyxml PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_TINYXML}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_TINYXML_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_TINYXML_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_TINYXML_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_TINYXML_DEBUG}>)
    set_property(TARGET CONAN_PKG::tinyxml PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_TINYXML}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_TINYXML_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_TINYXML_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_TINYXML_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_TINYXML_DEBUG}>)
    set_property(TARGET CONAN_PKG::tinyxml PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_TINYXML_LIST} ${CONAN_CXX_FLAGS_TINYXML_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_TINYXML_RELEASE_LIST} ${CONAN_CXX_FLAGS_TINYXML_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_TINYXML_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_TINYXML_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_TINYXML_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_TINYXML_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_TINYXML_DEBUG_LIST}  ${CONAN_CXX_FLAGS_TINYXML_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES "${CONAN_SYSTEM_LIBS_RABBITMQ-C} ${CONAN_FRAMEWORKS_FOUND_RABBITMQ-C} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RABBITMQ-C}" "${CONAN_LIB_DIRS_RABBITMQ-C}"
                                  CONAN_PACKAGE_TARGETS_RABBITMQ-C "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES}"
                                  "" rabbitmq-c)
    set(_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_RABBITMQ-C_DEBUG} ${CONAN_FRAMEWORKS_FOUND_RABBITMQ-C_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RABBITMQ-C_DEBUG}" "${CONAN_LIB_DIRS_RABBITMQ-C_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_RABBITMQ-C_DEBUG "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_DEBUG}"
                                  "debug" rabbitmq-c)
    set(_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_RABBITMQ-C_RELEASE} ${CONAN_FRAMEWORKS_FOUND_RABBITMQ-C_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RABBITMQ-C_RELEASE}" "${CONAN_LIB_DIRS_RABBITMQ-C_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_RABBITMQ-C_RELEASE "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELEASE}"
                                  "release" rabbitmq-c)
    set(_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_RABBITMQ-C_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_RABBITMQ-C_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RABBITMQ-C_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_RABBITMQ-C_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_RABBITMQ-C_RELWITHDEBINFO "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" rabbitmq-c)
    set(_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_RABBITMQ-C_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_RABBITMQ-C_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RABBITMQ-C_MINSIZEREL}" "${CONAN_LIB_DIRS_RABBITMQ-C_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_RABBITMQ-C_MINSIZEREL "${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" rabbitmq-c)

    add_library(CONAN_PKG::rabbitmq-c INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::rabbitmq-c PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_RABBITMQ-C} ${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RABBITMQ-C_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_RABBITMQ-C_RELEASE} ${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RABBITMQ-C_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_RABBITMQ-C_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RABBITMQ-C_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_RABBITMQ-C_MINSIZEREL} ${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RABBITMQ-C_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_RABBITMQ-C_DEBUG} ${_CONAN_PKG_LIBS_RABBITMQ-C_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RABBITMQ-C_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RABBITMQ-C_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::rabbitmq-c PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_RABBITMQ-C}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_RABBITMQ-C_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_RABBITMQ-C_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_RABBITMQ-C_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_RABBITMQ-C_DEBUG}>)
    set_property(TARGET CONAN_PKG::rabbitmq-c PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_RABBITMQ-C}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_RABBITMQ-C_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_RABBITMQ-C_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_RABBITMQ-C_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_RABBITMQ-C_DEBUG}>)
    set_property(TARGET CONAN_PKG::rabbitmq-c PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_RABBITMQ-C_LIST} ${CONAN_CXX_FLAGS_RABBITMQ-C_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_RABBITMQ-C_RELEASE_LIST} ${CONAN_CXX_FLAGS_RABBITMQ-C_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_RABBITMQ-C_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_RABBITMQ-C_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_RABBITMQ-C_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_RABBITMQ-C_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_RABBITMQ-C_DEBUG_LIST}  ${CONAN_CXX_FLAGS_RABBITMQ-C_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES "${CONAN_SYSTEM_LIBS_CPPZMQ} ${CONAN_FRAMEWORKS_FOUND_CPPZMQ} CONAN_PKG::zeromq")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPZMQ}" "${CONAN_LIB_DIRS_CPPZMQ}"
                                  CONAN_PACKAGE_TARGETS_CPPZMQ "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES}"
                                  "" cppzmq)
    set(_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_CPPZMQ_DEBUG} ${CONAN_FRAMEWORKS_FOUND_CPPZMQ_DEBUG} CONAN_PKG::zeromq")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPZMQ_DEBUG}" "${CONAN_LIB_DIRS_CPPZMQ_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_CPPZMQ_DEBUG "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_DEBUG}"
                                  "debug" cppzmq)
    set(_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_CPPZMQ_RELEASE} ${CONAN_FRAMEWORKS_FOUND_CPPZMQ_RELEASE} CONAN_PKG::zeromq")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPZMQ_RELEASE}" "${CONAN_LIB_DIRS_CPPZMQ_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_CPPZMQ_RELEASE "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELEASE}"
                                  "release" cppzmq)
    set(_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_CPPZMQ_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_CPPZMQ_RELWITHDEBINFO} CONAN_PKG::zeromq")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPZMQ_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_CPPZMQ_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_CPPZMQ_RELWITHDEBINFO "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" cppzmq)
    set(_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_CPPZMQ_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_CPPZMQ_MINSIZEREL} CONAN_PKG::zeromq")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPZMQ_MINSIZEREL}" "${CONAN_LIB_DIRS_CPPZMQ_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_CPPZMQ_MINSIZEREL "${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" cppzmq)

    add_library(CONAN_PKG::cppzmq INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::cppzmq PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_CPPZMQ} ${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPZMQ_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_CPPZMQ_RELEASE} ${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPZMQ_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_CPPZMQ_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPZMQ_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_CPPZMQ_MINSIZEREL} ${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPZMQ_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_CPPZMQ_DEBUG} ${_CONAN_PKG_LIBS_CPPZMQ_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPZMQ_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPZMQ_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::cppzmq PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_CPPZMQ}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_CPPZMQ_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_CPPZMQ_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_CPPZMQ_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_CPPZMQ_DEBUG}>)
    set_property(TARGET CONAN_PKG::cppzmq PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_CPPZMQ}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_CPPZMQ_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_CPPZMQ_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_CPPZMQ_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_CPPZMQ_DEBUG}>)
    set_property(TARGET CONAN_PKG::cppzmq PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_CPPZMQ_LIST} ${CONAN_CXX_FLAGS_CPPZMQ_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_CPPZMQ_RELEASE_LIST} ${CONAN_CXX_FLAGS_CPPZMQ_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_CPPZMQ_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_CPPZMQ_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_CPPZMQ_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_CPPZMQ_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_CPPZMQ_DEBUG_LIST}  ${CONAN_CXX_FLAGS_CPPZMQ_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES "${CONAN_SYSTEM_LIBS_PAHO-MQTT-CPP} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-CPP} CONAN_PKG::paho-mqtt-c")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-CPP}" "${CONAN_LIB_DIRS_PAHO-MQTT-CPP}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES}"
                                  "" paho-mqtt-cpp)
    set(_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_PAHO-MQTT-CPP_DEBUG} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-CPP_DEBUG} CONAN_PKG::paho-mqtt-c")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEBUG}" "${CONAN_LIB_DIRS_PAHO-MQTT-CPP_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP_DEBUG "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_DEBUG}"
                                  "debug" paho-mqtt-cpp)
    set(_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_PAHO-MQTT-CPP_RELEASE} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-CPP_RELEASE} CONAN_PKG::paho-mqtt-c")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-CPP_RELEASE}" "${CONAN_LIB_DIRS_PAHO-MQTT-CPP_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP_RELEASE "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELEASE}"
                                  "release" paho-mqtt-cpp)
    set(_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_PAHO-MQTT-CPP_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-CPP_RELWITHDEBINFO} CONAN_PKG::paho-mqtt-c")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-CPP_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_PAHO-MQTT-CPP_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP_RELWITHDEBINFO "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" paho-mqtt-cpp)
    set(_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_PAHO-MQTT-CPP_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-CPP_MINSIZEREL} CONAN_PKG::paho-mqtt-c")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-CPP_MINSIZEREL}" "${CONAN_LIB_DIRS_PAHO-MQTT-CPP_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP_MINSIZEREL "${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" paho-mqtt-cpp)

    add_library(CONAN_PKG::paho-mqtt-cpp INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::paho-mqtt-cpp PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP} ${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-CPP_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP_RELEASE} ${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-CPP_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-CPP_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP_MINSIZEREL} ${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-CPP_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_PAHO-MQTT-CPP_DEBUG} ${_CONAN_PKG_LIBS_PAHO-MQTT-CPP_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-CPP_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-CPP_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::paho-mqtt-cpp PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_PAHO-MQTT-CPP}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_PAHO-MQTT-CPP_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_PAHO-MQTT-CPP_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_PAHO-MQTT-CPP_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_PAHO-MQTT-CPP_DEBUG}>)
    set_property(TARGET CONAN_PKG::paho-mqtt-cpp PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-CPP}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-CPP_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-CPP_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-CPP_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-CPP_DEBUG}>)
    set_property(TARGET CONAN_PKG::paho-mqtt-cpp PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_PAHO-MQTT-CPP_LIST} ${CONAN_CXX_FLAGS_PAHO-MQTT-CPP_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_PAHO-MQTT-CPP_RELEASE_LIST} ${CONAN_CXX_FLAGS_PAHO-MQTT-CPP_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_PAHO-MQTT-CPP_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_PAHO-MQTT-CPP_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_PAHO-MQTT-CPP_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_PAHO-MQTT-CPP_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_PAHO-MQTT-CPP_DEBUG_LIST}  ${CONAN_CXX_FLAGS_PAHO-MQTT-CPP_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES "${CONAN_SYSTEM_LIBS_CPPCODEC} ${CONAN_FRAMEWORKS_FOUND_CPPCODEC} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPCODEC}" "${CONAN_LIB_DIRS_CPPCODEC}"
                                  CONAN_PACKAGE_TARGETS_CPPCODEC "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES}"
                                  "" cppcodec)
    set(_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_CPPCODEC_DEBUG} ${CONAN_FRAMEWORKS_FOUND_CPPCODEC_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPCODEC_DEBUG}" "${CONAN_LIB_DIRS_CPPCODEC_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_CPPCODEC_DEBUG "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_DEBUG}"
                                  "debug" cppcodec)
    set(_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_CPPCODEC_RELEASE} ${CONAN_FRAMEWORKS_FOUND_CPPCODEC_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPCODEC_RELEASE}" "${CONAN_LIB_DIRS_CPPCODEC_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_CPPCODEC_RELEASE "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELEASE}"
                                  "release" cppcodec)
    set(_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_CPPCODEC_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_CPPCODEC_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPCODEC_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_CPPCODEC_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_CPPCODEC_RELWITHDEBINFO "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" cppcodec)
    set(_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_CPPCODEC_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_CPPCODEC_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_CPPCODEC_MINSIZEREL}" "${CONAN_LIB_DIRS_CPPCODEC_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_CPPCODEC_MINSIZEREL "${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" cppcodec)

    add_library(CONAN_PKG::cppcodec INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::cppcodec PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_CPPCODEC} ${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPCODEC_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_CPPCODEC_RELEASE} ${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPCODEC_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_CPPCODEC_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPCODEC_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_CPPCODEC_MINSIZEREL} ${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPCODEC_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_CPPCODEC_DEBUG} ${_CONAN_PKG_LIBS_CPPCODEC_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_CPPCODEC_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_CPPCODEC_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::cppcodec PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_CPPCODEC}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_CPPCODEC_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_CPPCODEC_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_CPPCODEC_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_CPPCODEC_DEBUG}>)
    set_property(TARGET CONAN_PKG::cppcodec PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_CPPCODEC}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_CPPCODEC_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_CPPCODEC_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_CPPCODEC_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_CPPCODEC_DEBUG}>)
    set_property(TARGET CONAN_PKG::cppcodec PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_CPPCODEC_LIST} ${CONAN_CXX_FLAGS_CPPCODEC_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_CPPCODEC_RELEASE_LIST} ${CONAN_CXX_FLAGS_CPPCODEC_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_CPPCODEC_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_CPPCODEC_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_CPPCODEC_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_CPPCODEC_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_CPPCODEC_DEBUG_LIST}  ${CONAN_CXX_FLAGS_CPPCODEC_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES "${CONAN_SYSTEM_LIBS_FLATBUFFERS} ${CONAN_FRAMEWORKS_FOUND_FLATBUFFERS} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_FLATBUFFERS}" "${CONAN_LIB_DIRS_FLATBUFFERS}"
                                  CONAN_PACKAGE_TARGETS_FLATBUFFERS "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES}"
                                  "" flatbuffers)
    set(_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_FLATBUFFERS_DEBUG} ${CONAN_FRAMEWORKS_FOUND_FLATBUFFERS_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_FLATBUFFERS_DEBUG}" "${CONAN_LIB_DIRS_FLATBUFFERS_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_FLATBUFFERS_DEBUG "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_DEBUG}"
                                  "debug" flatbuffers)
    set(_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_FLATBUFFERS_RELEASE} ${CONAN_FRAMEWORKS_FOUND_FLATBUFFERS_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_FLATBUFFERS_RELEASE}" "${CONAN_LIB_DIRS_FLATBUFFERS_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_FLATBUFFERS_RELEASE "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELEASE}"
                                  "release" flatbuffers)
    set(_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_FLATBUFFERS_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_FLATBUFFERS_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_FLATBUFFERS_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_FLATBUFFERS_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_FLATBUFFERS_RELWITHDEBINFO "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" flatbuffers)
    set(_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_FLATBUFFERS_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_FLATBUFFERS_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_FLATBUFFERS_MINSIZEREL}" "${CONAN_LIB_DIRS_FLATBUFFERS_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_FLATBUFFERS_MINSIZEREL "${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" flatbuffers)

    add_library(CONAN_PKG::flatbuffers INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::flatbuffers PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_FLATBUFFERS} ${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_FLATBUFFERS_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_FLATBUFFERS_RELEASE} ${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_FLATBUFFERS_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_FLATBUFFERS_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_FLATBUFFERS_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_FLATBUFFERS_MINSIZEREL} ${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_FLATBUFFERS_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_FLATBUFFERS_DEBUG} ${_CONAN_PKG_LIBS_FLATBUFFERS_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_FLATBUFFERS_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_FLATBUFFERS_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::flatbuffers PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_FLATBUFFERS}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_FLATBUFFERS_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_FLATBUFFERS_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_FLATBUFFERS_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_FLATBUFFERS_DEBUG}>)
    set_property(TARGET CONAN_PKG::flatbuffers PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_FLATBUFFERS}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_FLATBUFFERS_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_FLATBUFFERS_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_FLATBUFFERS_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_FLATBUFFERS_DEBUG}>)
    set_property(TARGET CONAN_PKG::flatbuffers PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_FLATBUFFERS_LIST} ${CONAN_CXX_FLAGS_FLATBUFFERS_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_FLATBUFFERS_RELEASE_LIST} ${CONAN_CXX_FLAGS_FLATBUFFERS_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_FLATBUFFERS_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_FLATBUFFERS_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_FLATBUFFERS_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_FLATBUFFERS_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_FLATBUFFERS_DEBUG_LIST}  ${CONAN_CXX_FLAGS_FLATBUFFERS_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_GRPC_DEPENDENCIES "${CONAN_SYSTEM_LIBS_GRPC} ${CONAN_FRAMEWORKS_FOUND_GRPC} CONAN_PKG::zlib CONAN_PKG::openssl CONAN_PKG::protobuf CONAN_PKG::c-ares CONAN_PKG::abseil CONAN_PKG::re2")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_GRPC_DEPENDENCIES "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_GRPC}" "${CONAN_LIB_DIRS_GRPC}"
                                  CONAN_PACKAGE_TARGETS_GRPC "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES}"
                                  "" grpc)
    set(_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_GRPC_DEBUG} ${CONAN_FRAMEWORKS_FOUND_GRPC_DEBUG} CONAN_PKG::zlib CONAN_PKG::openssl CONAN_PKG::protobuf CONAN_PKG::c-ares CONAN_PKG::abseil CONAN_PKG::re2")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_GRPC_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_GRPC_DEBUG}" "${CONAN_LIB_DIRS_GRPC_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_GRPC_DEBUG "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_DEBUG}"
                                  "debug" grpc)
    set(_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_GRPC_RELEASE} ${CONAN_FRAMEWORKS_FOUND_GRPC_RELEASE} CONAN_PKG::zlib CONAN_PKG::openssl CONAN_PKG::protobuf CONAN_PKG::c-ares CONAN_PKG::abseil CONAN_PKG::re2")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_GRPC_RELEASE}" "${CONAN_LIB_DIRS_GRPC_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_GRPC_RELEASE "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELEASE}"
                                  "release" grpc)
    set(_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_GRPC_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_GRPC_RELWITHDEBINFO} CONAN_PKG::zlib CONAN_PKG::openssl CONAN_PKG::protobuf CONAN_PKG::c-ares CONAN_PKG::abseil CONAN_PKG::re2")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_GRPC_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_GRPC_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_GRPC_RELWITHDEBINFO "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" grpc)
    set(_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_GRPC_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_GRPC_MINSIZEREL} CONAN_PKG::zlib CONAN_PKG::openssl CONAN_PKG::protobuf CONAN_PKG::c-ares CONAN_PKG::abseil CONAN_PKG::re2")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_GRPC_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_GRPC_MINSIZEREL}" "${CONAN_LIB_DIRS_GRPC_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_GRPC_MINSIZEREL "${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" grpc)

    add_library(CONAN_PKG::grpc INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::grpc PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_GRPC} ${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_GRPC_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_GRPC_RELEASE} ${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_GRPC_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_GRPC_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_GRPC_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_GRPC_MINSIZEREL} ${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_GRPC_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_GRPC_DEBUG} ${_CONAN_PKG_LIBS_GRPC_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_GRPC_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_GRPC_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::grpc PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_GRPC}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_GRPC_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_GRPC_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_GRPC_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_GRPC_DEBUG}>)
    set_property(TARGET CONAN_PKG::grpc PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_GRPC}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_GRPC_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_GRPC_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_GRPC_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_GRPC_DEBUG}>)
    set_property(TARGET CONAN_PKG::grpc PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_GRPC_LIST} ${CONAN_CXX_FLAGS_GRPC_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_GRPC_RELEASE_LIST} ${CONAN_CXX_FLAGS_GRPC_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_GRPC_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_GRPC_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_GRPC_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_GRPC_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_GRPC_DEBUG_LIST}  ${CONAN_CXX_FLAGS_GRPC_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES "${CONAN_SYSTEM_LIBS_SERIAL} ${CONAN_FRAMEWORKS_FOUND_SERIAL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SERIAL_DEPENDENCIES "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SERIAL}" "${CONAN_LIB_DIRS_SERIAL}"
                                  CONAN_PACKAGE_TARGETS_SERIAL "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES}"
                                  "" serial)
    set(_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_SERIAL_DEBUG} ${CONAN_FRAMEWORKS_FOUND_SERIAL_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SERIAL_DEBUG}" "${CONAN_LIB_DIRS_SERIAL_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_SERIAL_DEBUG "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_DEBUG}"
                                  "debug" serial)
    set(_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_SERIAL_RELEASE} ${CONAN_FRAMEWORKS_FOUND_SERIAL_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SERIAL_RELEASE}" "${CONAN_LIB_DIRS_SERIAL_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_SERIAL_RELEASE "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELEASE}"
                                  "release" serial)
    set(_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_SERIAL_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_SERIAL_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SERIAL_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_SERIAL_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_SERIAL_RELWITHDEBINFO "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" serial)
    set(_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_SERIAL_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_SERIAL_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SERIAL_MINSIZEREL}" "${CONAN_LIB_DIRS_SERIAL_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_SERIAL_MINSIZEREL "${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" serial)

    add_library(CONAN_PKG::serial INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::serial PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_SERIAL} ${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SERIAL_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_SERIAL_RELEASE} ${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SERIAL_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_SERIAL_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SERIAL_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_SERIAL_MINSIZEREL} ${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SERIAL_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_SERIAL_DEBUG} ${_CONAN_PKG_LIBS_SERIAL_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SERIAL_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SERIAL_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::serial PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_SERIAL}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_SERIAL_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_SERIAL_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_SERIAL_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_SERIAL_DEBUG}>)
    set_property(TARGET CONAN_PKG::serial PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_SERIAL}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_SERIAL_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_SERIAL_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_SERIAL_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_SERIAL_DEBUG}>)
    set_property(TARGET CONAN_PKG::serial PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_SERIAL_LIST} ${CONAN_CXX_FLAGS_SERIAL_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_SERIAL_RELEASE_LIST} ${CONAN_CXX_FLAGS_SERIAL_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_SERIAL_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_SERIAL_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_SERIAL_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_SERIAL_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_SERIAL_DEBUG_LIST}  ${CONAN_CXX_FLAGS_SERIAL_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_BOOST_DEPENDENCIES "${CONAN_SYSTEM_LIBS_BOOST} ${CONAN_FRAMEWORKS_FOUND_BOOST} CONAN_PKG::zlib CONAN_PKG::bzip2 CONAN_PKG::libbacktrace")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BOOST_DEPENDENCIES "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BOOST}" "${CONAN_LIB_DIRS_BOOST}"
                                  CONAN_PACKAGE_TARGETS_BOOST "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES}"
                                  "" boost)
    set(_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_BOOST_DEBUG} ${CONAN_FRAMEWORKS_FOUND_BOOST_DEBUG} CONAN_PKG::zlib CONAN_PKG::bzip2 CONAN_PKG::libbacktrace")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BOOST_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BOOST_DEBUG}" "${CONAN_LIB_DIRS_BOOST_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_BOOST_DEBUG "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_DEBUG}"
                                  "debug" boost)
    set(_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_BOOST_RELEASE} ${CONAN_FRAMEWORKS_FOUND_BOOST_RELEASE} CONAN_PKG::zlib CONAN_PKG::bzip2 CONAN_PKG::libbacktrace")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BOOST_RELEASE}" "${CONAN_LIB_DIRS_BOOST_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_BOOST_RELEASE "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELEASE}"
                                  "release" boost)
    set(_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_BOOST_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_BOOST_RELWITHDEBINFO} CONAN_PKG::zlib CONAN_PKG::bzip2 CONAN_PKG::libbacktrace")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BOOST_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_BOOST_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_BOOST_RELWITHDEBINFO "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" boost)
    set(_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_BOOST_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_BOOST_MINSIZEREL} CONAN_PKG::zlib CONAN_PKG::bzip2 CONAN_PKG::libbacktrace")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BOOST_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BOOST_MINSIZEREL}" "${CONAN_LIB_DIRS_BOOST_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_BOOST_MINSIZEREL "${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" boost)

    add_library(CONAN_PKG::boost INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::boost PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_BOOST} ${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BOOST_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_BOOST_RELEASE} ${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BOOST_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_BOOST_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BOOST_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_BOOST_MINSIZEREL} ${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BOOST_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_BOOST_DEBUG} ${_CONAN_PKG_LIBS_BOOST_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BOOST_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BOOST_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::boost PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_BOOST}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_BOOST_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_BOOST_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_BOOST_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_BOOST_DEBUG}>)
    set_property(TARGET CONAN_PKG::boost PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_BOOST}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_BOOST_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_BOOST_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_BOOST_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_BOOST_DEBUG}>)
    set_property(TARGET CONAN_PKG::boost PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_BOOST_LIST} ${CONAN_CXX_FLAGS_BOOST_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_BOOST_RELEASE_LIST} ${CONAN_CXX_FLAGS_BOOST_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_BOOST_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_BOOST_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_BOOST_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_BOOST_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_BOOST_DEBUG_LIST}  ${CONAN_CXX_FLAGS_BOOST_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES "${CONAN_SYSTEM_LIBS_SQLITE3} ${CONAN_FRAMEWORKS_FOUND_SQLITE3} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SQLITE3}" "${CONAN_LIB_DIRS_SQLITE3}"
                                  CONAN_PACKAGE_TARGETS_SQLITE3 "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES}"
                                  "" sqlite3)
    set(_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_SQLITE3_DEBUG} ${CONAN_FRAMEWORKS_FOUND_SQLITE3_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SQLITE3_DEBUG}" "${CONAN_LIB_DIRS_SQLITE3_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_SQLITE3_DEBUG "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_DEBUG}"
                                  "debug" sqlite3)
    set(_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_SQLITE3_RELEASE} ${CONAN_FRAMEWORKS_FOUND_SQLITE3_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SQLITE3_RELEASE}" "${CONAN_LIB_DIRS_SQLITE3_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_SQLITE3_RELEASE "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELEASE}"
                                  "release" sqlite3)
    set(_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_SQLITE3_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_SQLITE3_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SQLITE3_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_SQLITE3_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_SQLITE3_RELWITHDEBINFO "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" sqlite3)
    set(_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_SQLITE3_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_SQLITE3_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_SQLITE3_MINSIZEREL}" "${CONAN_LIB_DIRS_SQLITE3_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_SQLITE3_MINSIZEREL "${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" sqlite3)

    add_library(CONAN_PKG::sqlite3 INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::sqlite3 PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_SQLITE3} ${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SQLITE3_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_SQLITE3_RELEASE} ${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SQLITE3_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_SQLITE3_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SQLITE3_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_SQLITE3_MINSIZEREL} ${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SQLITE3_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_SQLITE3_DEBUG} ${_CONAN_PKG_LIBS_SQLITE3_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_SQLITE3_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_SQLITE3_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::sqlite3 PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_SQLITE3}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_SQLITE3_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_SQLITE3_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_SQLITE3_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_SQLITE3_DEBUG}>)
    set_property(TARGET CONAN_PKG::sqlite3 PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_SQLITE3}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_SQLITE3_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_SQLITE3_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_SQLITE3_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_SQLITE3_DEBUG}>)
    set_property(TARGET CONAN_PKG::sqlite3 PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_SQLITE3_LIST} ${CONAN_CXX_FLAGS_SQLITE3_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_SQLITE3_RELEASE_LIST} ${CONAN_CXX_FLAGS_SQLITE3_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_SQLITE3_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_SQLITE3_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_SQLITE3_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_SQLITE3_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_SQLITE3_DEBUG_LIST}  ${CONAN_CXX_FLAGS_SQLITE3_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES "${CONAN_SYSTEM_LIBS_LIBPQ} ${CONAN_FRAMEWORKS_FOUND_LIBPQ} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBPQ}" "${CONAN_LIB_DIRS_LIBPQ}"
                                  CONAN_PACKAGE_TARGETS_LIBPQ "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES}"
                                  "" libpq)
    set(_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_LIBPQ_DEBUG} ${CONAN_FRAMEWORKS_FOUND_LIBPQ_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBPQ_DEBUG}" "${CONAN_LIB_DIRS_LIBPQ_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_LIBPQ_DEBUG "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_DEBUG}"
                                  "debug" libpq)
    set(_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_LIBPQ_RELEASE} ${CONAN_FRAMEWORKS_FOUND_LIBPQ_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBPQ_RELEASE}" "${CONAN_LIB_DIRS_LIBPQ_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_LIBPQ_RELEASE "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELEASE}"
                                  "release" libpq)
    set(_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_LIBPQ_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_LIBPQ_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBPQ_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_LIBPQ_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_LIBPQ_RELWITHDEBINFO "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" libpq)
    set(_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_LIBPQ_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_LIBPQ_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBPQ_MINSIZEREL}" "${CONAN_LIB_DIRS_LIBPQ_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_LIBPQ_MINSIZEREL "${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" libpq)

    add_library(CONAN_PKG::libpq INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::libpq PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_LIBPQ} ${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBPQ_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_LIBPQ_RELEASE} ${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBPQ_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_LIBPQ_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBPQ_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_LIBPQ_MINSIZEREL} ${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBPQ_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_LIBPQ_DEBUG} ${_CONAN_PKG_LIBS_LIBPQ_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBPQ_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBPQ_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::libpq PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_LIBPQ}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_LIBPQ_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_LIBPQ_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_LIBPQ_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_LIBPQ_DEBUG}>)
    set_property(TARGET CONAN_PKG::libpq PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_LIBPQ}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_LIBPQ_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_LIBPQ_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_LIBPQ_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_LIBPQ_DEBUG}>)
    set_property(TARGET CONAN_PKG::libpq PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_LIBPQ_LIST} ${CONAN_CXX_FLAGS_LIBPQ_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_LIBPQ_RELEASE_LIST} ${CONAN_CXX_FLAGS_LIBPQ_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_LIBPQ_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_LIBPQ_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_LIBPQ_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_LIBPQ_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_LIBPQ_DEBUG_LIST}  ${CONAN_CXX_FLAGS_LIBPQ_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES "${CONAN_SYSTEM_LIBS_LIBCURL} ${CONAN_FRAMEWORKS_FOUND_LIBCURL} CONAN_PKG::openssl CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBCURL}" "${CONAN_LIB_DIRS_LIBCURL}"
                                  CONAN_PACKAGE_TARGETS_LIBCURL "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES}"
                                  "" libcurl)
    set(_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_LIBCURL_DEBUG} ${CONAN_FRAMEWORKS_FOUND_LIBCURL_DEBUG} CONAN_PKG::openssl CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBCURL_DEBUG}" "${CONAN_LIB_DIRS_LIBCURL_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_LIBCURL_DEBUG "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_DEBUG}"
                                  "debug" libcurl)
    set(_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_LIBCURL_RELEASE} ${CONAN_FRAMEWORKS_FOUND_LIBCURL_RELEASE} CONAN_PKG::openssl CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBCURL_RELEASE}" "${CONAN_LIB_DIRS_LIBCURL_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_LIBCURL_RELEASE "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELEASE}"
                                  "release" libcurl)
    set(_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_LIBCURL_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_LIBCURL_RELWITHDEBINFO} CONAN_PKG::openssl CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBCURL_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_LIBCURL_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_LIBCURL_RELWITHDEBINFO "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" libcurl)
    set(_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_LIBCURL_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_LIBCURL_MINSIZEREL} CONAN_PKG::openssl CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBCURL_MINSIZEREL}" "${CONAN_LIB_DIRS_LIBCURL_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_LIBCURL_MINSIZEREL "${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" libcurl)

    add_library(CONAN_PKG::libcurl INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::libcurl PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_LIBCURL} ${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBCURL_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_LIBCURL_RELEASE} ${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBCURL_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_LIBCURL_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBCURL_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_LIBCURL_MINSIZEREL} ${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBCURL_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_LIBCURL_DEBUG} ${_CONAN_PKG_LIBS_LIBCURL_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBCURL_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBCURL_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::libcurl PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_LIBCURL}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_LIBCURL_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_LIBCURL_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_LIBCURL_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_LIBCURL_DEBUG}>)
    set_property(TARGET CONAN_PKG::libcurl PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_LIBCURL}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_LIBCURL_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_LIBCURL_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_LIBCURL_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_LIBCURL_DEBUG}>)
    set_property(TARGET CONAN_PKG::libcurl PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_LIBCURL_LIST} ${CONAN_CXX_FLAGS_LIBCURL_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_LIBCURL_RELEASE_LIST} ${CONAN_CXX_FLAGS_LIBCURL_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_LIBCURL_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_LIBCURL_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_LIBCURL_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_LIBCURL_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_LIBCURL_DEBUG_LIST}  ${CONAN_CXX_FLAGS_LIBCURL_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES "${CONAN_SYSTEM_LIBS_NLOHMANN_JSON} ${CONAN_FRAMEWORKS_FOUND_NLOHMANN_JSON} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_NLOHMANN_JSON}" "${CONAN_LIB_DIRS_NLOHMANN_JSON}"
                                  CONAN_PACKAGE_TARGETS_NLOHMANN_JSON "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES}"
                                  "" nlohmann_json)
    set(_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_NLOHMANN_JSON_DEBUG} ${CONAN_FRAMEWORKS_FOUND_NLOHMANN_JSON_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_NLOHMANN_JSON_DEBUG}" "${CONAN_LIB_DIRS_NLOHMANN_JSON_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_NLOHMANN_JSON_DEBUG "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_DEBUG}"
                                  "debug" nlohmann_json)
    set(_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_NLOHMANN_JSON_RELEASE} ${CONAN_FRAMEWORKS_FOUND_NLOHMANN_JSON_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_NLOHMANN_JSON_RELEASE}" "${CONAN_LIB_DIRS_NLOHMANN_JSON_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_NLOHMANN_JSON_RELEASE "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELEASE}"
                                  "release" nlohmann_json)
    set(_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_NLOHMANN_JSON_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_NLOHMANN_JSON_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_NLOHMANN_JSON_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_NLOHMANN_JSON_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_NLOHMANN_JSON_RELWITHDEBINFO "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" nlohmann_json)
    set(_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_NLOHMANN_JSON_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_NLOHMANN_JSON_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_NLOHMANN_JSON_MINSIZEREL}" "${CONAN_LIB_DIRS_NLOHMANN_JSON_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_NLOHMANN_JSON_MINSIZEREL "${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" nlohmann_json)

    add_library(CONAN_PKG::nlohmann_json INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::nlohmann_json PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_NLOHMANN_JSON} ${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_NLOHMANN_JSON_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_NLOHMANN_JSON_RELEASE} ${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_NLOHMANN_JSON_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_NLOHMANN_JSON_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_NLOHMANN_JSON_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_NLOHMANN_JSON_MINSIZEREL} ${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_NLOHMANN_JSON_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_NLOHMANN_JSON_DEBUG} ${_CONAN_PKG_LIBS_NLOHMANN_JSON_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_NLOHMANN_JSON_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_NLOHMANN_JSON_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::nlohmann_json PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_NLOHMANN_JSON}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_NLOHMANN_JSON_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_NLOHMANN_JSON_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_NLOHMANN_JSON_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_NLOHMANN_JSON_DEBUG}>)
    set_property(TARGET CONAN_PKG::nlohmann_json PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_NLOHMANN_JSON}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_NLOHMANN_JSON_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_NLOHMANN_JSON_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_NLOHMANN_JSON_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_NLOHMANN_JSON_DEBUG}>)
    set_property(TARGET CONAN_PKG::nlohmann_json PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_NLOHMANN_JSON_LIST} ${CONAN_CXX_FLAGS_NLOHMANN_JSON_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_NLOHMANN_JSON_RELEASE_LIST} ${CONAN_CXX_FLAGS_NLOHMANN_JSON_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_NLOHMANN_JSON_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_NLOHMANN_JSON_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_NLOHMANN_JSON_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_NLOHMANN_JSON_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_NLOHMANN_JSON_DEBUG_LIST}  ${CONAN_CXX_FLAGS_NLOHMANN_JSON_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES "${CONAN_SYSTEM_LIBS_ZEROMQ} ${CONAN_FRAMEWORKS_FOUND_ZEROMQ} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZEROMQ}" "${CONAN_LIB_DIRS_ZEROMQ}"
                                  CONAN_PACKAGE_TARGETS_ZEROMQ "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES}"
                                  "" zeromq)
    set(_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_ZEROMQ_DEBUG} ${CONAN_FRAMEWORKS_FOUND_ZEROMQ_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZEROMQ_DEBUG}" "${CONAN_LIB_DIRS_ZEROMQ_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_ZEROMQ_DEBUG "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_DEBUG}"
                                  "debug" zeromq)
    set(_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_ZEROMQ_RELEASE} ${CONAN_FRAMEWORKS_FOUND_ZEROMQ_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZEROMQ_RELEASE}" "${CONAN_LIB_DIRS_ZEROMQ_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_ZEROMQ_RELEASE "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELEASE}"
                                  "release" zeromq)
    set(_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_ZEROMQ_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_ZEROMQ_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZEROMQ_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_ZEROMQ_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_ZEROMQ_RELWITHDEBINFO "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" zeromq)
    set(_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_ZEROMQ_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_ZEROMQ_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZEROMQ_MINSIZEREL}" "${CONAN_LIB_DIRS_ZEROMQ_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_ZEROMQ_MINSIZEREL "${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" zeromq)

    add_library(CONAN_PKG::zeromq INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::zeromq PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_ZEROMQ} ${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZEROMQ_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_ZEROMQ_RELEASE} ${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZEROMQ_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_ZEROMQ_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZEROMQ_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_ZEROMQ_MINSIZEREL} ${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZEROMQ_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_ZEROMQ_DEBUG} ${_CONAN_PKG_LIBS_ZEROMQ_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZEROMQ_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZEROMQ_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::zeromq PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_ZEROMQ}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_ZEROMQ_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_ZEROMQ_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_ZEROMQ_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_ZEROMQ_DEBUG}>)
    set_property(TARGET CONAN_PKG::zeromq PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_ZEROMQ}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_ZEROMQ_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_ZEROMQ_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_ZEROMQ_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_ZEROMQ_DEBUG}>)
    set_property(TARGET CONAN_PKG::zeromq PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_ZEROMQ_LIST} ${CONAN_CXX_FLAGS_ZEROMQ_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_ZEROMQ_RELEASE_LIST} ${CONAN_CXX_FLAGS_ZEROMQ_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_ZEROMQ_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_ZEROMQ_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_ZEROMQ_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_ZEROMQ_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_ZEROMQ_DEBUG_LIST}  ${CONAN_CXX_FLAGS_ZEROMQ_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES "${CONAN_SYSTEM_LIBS_PAHO-MQTT-C} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-C} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-C}" "${CONAN_LIB_DIRS_PAHO-MQTT-C}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-C "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES}"
                                  "" paho-mqtt-c)
    set(_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_PAHO-MQTT-C_DEBUG} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-C_DEBUG} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-C_DEBUG}" "${CONAN_LIB_DIRS_PAHO-MQTT-C_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-C_DEBUG "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_DEBUG}"
                                  "debug" paho-mqtt-c)
    set(_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_PAHO-MQTT-C_RELEASE} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-C_RELEASE} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-C_RELEASE}" "${CONAN_LIB_DIRS_PAHO-MQTT-C_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-C_RELEASE "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELEASE}"
                                  "release" paho-mqtt-c)
    set(_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_PAHO-MQTT-C_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-C_RELWITHDEBINFO} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-C_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_PAHO-MQTT-C_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-C_RELWITHDEBINFO "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" paho-mqtt-c)
    set(_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_PAHO-MQTT-C_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_PAHO-MQTT-C_MINSIZEREL} CONAN_PKG::openssl")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PAHO-MQTT-C_MINSIZEREL}" "${CONAN_LIB_DIRS_PAHO-MQTT-C_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_PAHO-MQTT-C_MINSIZEREL "${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" paho-mqtt-c)

    add_library(CONAN_PKG::paho-mqtt-c INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::paho-mqtt-c PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_PAHO-MQTT-C} ${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-C_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_PAHO-MQTT-C_RELEASE} ${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-C_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_PAHO-MQTT-C_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-C_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_PAHO-MQTT-C_MINSIZEREL} ${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-C_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_PAHO-MQTT-C_DEBUG} ${_CONAN_PKG_LIBS_PAHO-MQTT-C_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PAHO-MQTT-C_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PAHO-MQTT-C_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::paho-mqtt-c PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_PAHO-MQTT-C}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_PAHO-MQTT-C_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_PAHO-MQTT-C_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_PAHO-MQTT-C_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_PAHO-MQTT-C_DEBUG}>)
    set_property(TARGET CONAN_PKG::paho-mqtt-c PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-C}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-C_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-C_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-C_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_PAHO-MQTT-C_DEBUG}>)
    set_property(TARGET CONAN_PKG::paho-mqtt-c PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_PAHO-MQTT-C_LIST} ${CONAN_CXX_FLAGS_PAHO-MQTT-C_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_PAHO-MQTT-C_RELEASE_LIST} ${CONAN_CXX_FLAGS_PAHO-MQTT-C_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_PAHO-MQTT-C_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_PAHO-MQTT-C_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_PAHO-MQTT-C_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_PAHO-MQTT-C_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_PAHO-MQTT-C_DEBUG_LIST}  ${CONAN_CXX_FLAGS_PAHO-MQTT-C_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES "${CONAN_SYSTEM_LIBS_PROTOBUF} ${CONAN_FRAMEWORKS_FOUND_PROTOBUF} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PROTOBUF}" "${CONAN_LIB_DIRS_PROTOBUF}"
                                  CONAN_PACKAGE_TARGETS_PROTOBUF "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES}"
                                  "" protobuf)
    set(_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_PROTOBUF_DEBUG} ${CONAN_FRAMEWORKS_FOUND_PROTOBUF_DEBUG} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PROTOBUF_DEBUG}" "${CONAN_LIB_DIRS_PROTOBUF_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_PROTOBUF_DEBUG "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_DEBUG}"
                                  "debug" protobuf)
    set(_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_PROTOBUF_RELEASE} ${CONAN_FRAMEWORKS_FOUND_PROTOBUF_RELEASE} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PROTOBUF_RELEASE}" "${CONAN_LIB_DIRS_PROTOBUF_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_PROTOBUF_RELEASE "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELEASE}"
                                  "release" protobuf)
    set(_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_PROTOBUF_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_PROTOBUF_RELWITHDEBINFO} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PROTOBUF_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_PROTOBUF_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_PROTOBUF_RELWITHDEBINFO "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" protobuf)
    set(_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_PROTOBUF_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_PROTOBUF_MINSIZEREL} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_PROTOBUF_MINSIZEREL}" "${CONAN_LIB_DIRS_PROTOBUF_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_PROTOBUF_MINSIZEREL "${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" protobuf)

    add_library(CONAN_PKG::protobuf INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::protobuf PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_PROTOBUF} ${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PROTOBUF_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_PROTOBUF_RELEASE} ${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PROTOBUF_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_PROTOBUF_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PROTOBUF_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_PROTOBUF_MINSIZEREL} ${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PROTOBUF_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_PROTOBUF_DEBUG} ${_CONAN_PKG_LIBS_PROTOBUF_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_PROTOBUF_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_PROTOBUF_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::protobuf PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_PROTOBUF}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_PROTOBUF_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_PROTOBUF_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_PROTOBUF_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_PROTOBUF_DEBUG}>)
    set_property(TARGET CONAN_PKG::protobuf PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_PROTOBUF}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_PROTOBUF_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_PROTOBUF_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_PROTOBUF_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_PROTOBUF_DEBUG}>)
    set_property(TARGET CONAN_PKG::protobuf PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_PROTOBUF_LIST} ${CONAN_CXX_FLAGS_PROTOBUF_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_PROTOBUF_RELEASE_LIST} ${CONAN_CXX_FLAGS_PROTOBUF_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_PROTOBUF_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_PROTOBUF_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_PROTOBUF_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_PROTOBUF_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_PROTOBUF_DEBUG_LIST}  ${CONAN_CXX_FLAGS_PROTOBUF_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES "${CONAN_SYSTEM_LIBS_C-ARES} ${CONAN_FRAMEWORKS_FOUND_C-ARES} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_C-ARES_DEPENDENCIES "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_C-ARES}" "${CONAN_LIB_DIRS_C-ARES}"
                                  CONAN_PACKAGE_TARGETS_C-ARES "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES}"
                                  "" c-ares)
    set(_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_C-ARES_DEBUG} ${CONAN_FRAMEWORKS_FOUND_C-ARES_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_C-ARES_DEBUG}" "${CONAN_LIB_DIRS_C-ARES_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_C-ARES_DEBUG "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_DEBUG}"
                                  "debug" c-ares)
    set(_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_C-ARES_RELEASE} ${CONAN_FRAMEWORKS_FOUND_C-ARES_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_C-ARES_RELEASE}" "${CONAN_LIB_DIRS_C-ARES_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_C-ARES_RELEASE "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELEASE}"
                                  "release" c-ares)
    set(_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_C-ARES_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_C-ARES_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_C-ARES_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_C-ARES_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_C-ARES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" c-ares)
    set(_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_C-ARES_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_C-ARES_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_C-ARES_MINSIZEREL}" "${CONAN_LIB_DIRS_C-ARES_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_C-ARES_MINSIZEREL "${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" c-ares)

    add_library(CONAN_PKG::c-ares INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::c-ares PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_C-ARES} ${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_C-ARES_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_C-ARES_RELEASE} ${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_C-ARES_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_C-ARES_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_C-ARES_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_C-ARES_MINSIZEREL} ${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_C-ARES_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_C-ARES_DEBUG} ${_CONAN_PKG_LIBS_C-ARES_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_C-ARES_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_C-ARES_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::c-ares PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_C-ARES}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_C-ARES_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_C-ARES_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_C-ARES_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_C-ARES_DEBUG}>)
    set_property(TARGET CONAN_PKG::c-ares PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_C-ARES}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_C-ARES_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_C-ARES_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_C-ARES_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_C-ARES_DEBUG}>)
    set_property(TARGET CONAN_PKG::c-ares PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_C-ARES_LIST} ${CONAN_CXX_FLAGS_C-ARES_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_C-ARES_RELEASE_LIST} ${CONAN_CXX_FLAGS_C-ARES_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_C-ARES_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_C-ARES_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_C-ARES_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_C-ARES_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_C-ARES_DEBUG_LIST}  ${CONAN_CXX_FLAGS_C-ARES_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES "${CONAN_SYSTEM_LIBS_ABSEIL} ${CONAN_FRAMEWORKS_FOUND_ABSEIL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ABSEIL}" "${CONAN_LIB_DIRS_ABSEIL}"
                                  CONAN_PACKAGE_TARGETS_ABSEIL "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES}"
                                  "" abseil)
    set(_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_ABSEIL_DEBUG} ${CONAN_FRAMEWORKS_FOUND_ABSEIL_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ABSEIL_DEBUG}" "${CONAN_LIB_DIRS_ABSEIL_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_ABSEIL_DEBUG "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_DEBUG}"
                                  "debug" abseil)
    set(_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_ABSEIL_RELEASE} ${CONAN_FRAMEWORKS_FOUND_ABSEIL_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ABSEIL_RELEASE}" "${CONAN_LIB_DIRS_ABSEIL_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_ABSEIL_RELEASE "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELEASE}"
                                  "release" abseil)
    set(_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_ABSEIL_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_ABSEIL_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ABSEIL_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_ABSEIL_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_ABSEIL_RELWITHDEBINFO "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" abseil)
    set(_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_ABSEIL_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_ABSEIL_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ABSEIL_MINSIZEREL}" "${CONAN_LIB_DIRS_ABSEIL_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_ABSEIL_MINSIZEREL "${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" abseil)

    add_library(CONAN_PKG::abseil INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::abseil PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_ABSEIL} ${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ABSEIL_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_ABSEIL_RELEASE} ${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ABSEIL_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_ABSEIL_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ABSEIL_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_ABSEIL_MINSIZEREL} ${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ABSEIL_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_ABSEIL_DEBUG} ${_CONAN_PKG_LIBS_ABSEIL_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ABSEIL_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ABSEIL_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::abseil PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_ABSEIL}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_ABSEIL_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_ABSEIL_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_ABSEIL_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_ABSEIL_DEBUG}>)
    set_property(TARGET CONAN_PKG::abseil PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_ABSEIL}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_ABSEIL_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_ABSEIL_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_ABSEIL_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_ABSEIL_DEBUG}>)
    set_property(TARGET CONAN_PKG::abseil PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_ABSEIL_LIST} ${CONAN_CXX_FLAGS_ABSEIL_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_ABSEIL_RELEASE_LIST} ${CONAN_CXX_FLAGS_ABSEIL_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_ABSEIL_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_ABSEIL_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_ABSEIL_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_ABSEIL_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_ABSEIL_DEBUG_LIST}  ${CONAN_CXX_FLAGS_ABSEIL_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_RE2_DEPENDENCIES "${CONAN_SYSTEM_LIBS_RE2} ${CONAN_FRAMEWORKS_FOUND_RE2} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RE2_DEPENDENCIES "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RE2}" "${CONAN_LIB_DIRS_RE2}"
                                  CONAN_PACKAGE_TARGETS_RE2 "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES}"
                                  "" re2)
    set(_CONAN_PKG_LIBS_RE2_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_RE2_DEBUG} ${CONAN_FRAMEWORKS_FOUND_RE2_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RE2_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RE2_DEBUG}" "${CONAN_LIB_DIRS_RE2_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_RE2_DEBUG "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_DEBUG}"
                                  "debug" re2)
    set(_CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_RE2_RELEASE} ${CONAN_FRAMEWORKS_FOUND_RE2_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RE2_RELEASE}" "${CONAN_LIB_DIRS_RE2_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_RE2_RELEASE "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELEASE}"
                                  "release" re2)
    set(_CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_RE2_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_RE2_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RE2_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_RE2_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_RE2_RELWITHDEBINFO "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" re2)
    set(_CONAN_PKG_LIBS_RE2_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_RE2_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_RE2_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_RE2_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_RE2_MINSIZEREL}" "${CONAN_LIB_DIRS_RE2_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_RE2_MINSIZEREL "${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" re2)

    add_library(CONAN_PKG::re2 INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::re2 PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_RE2} ${_CONAN_PKG_LIBS_RE2_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RE2_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_RE2_RELEASE} ${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RE2_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_RE2_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RE2_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_RE2_MINSIZEREL} ${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RE2_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_RE2_DEBUG} ${_CONAN_PKG_LIBS_RE2_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_RE2_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_RE2_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::re2 PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_RE2}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_RE2_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_RE2_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_RE2_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_RE2_DEBUG}>)
    set_property(TARGET CONAN_PKG::re2 PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_RE2}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_RE2_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_RE2_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_RE2_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_RE2_DEBUG}>)
    set_property(TARGET CONAN_PKG::re2 PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_RE2_LIST} ${CONAN_CXX_FLAGS_RE2_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_RE2_RELEASE_LIST} ${CONAN_CXX_FLAGS_RE2_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_RE2_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_RE2_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_RE2_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_RE2_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_RE2_DEBUG_LIST}  ${CONAN_CXX_FLAGS_RE2_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES "${CONAN_SYSTEM_LIBS_OPENSSL} ${CONAN_FRAMEWORKS_FOUND_OPENSSL} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_OPENSSL}" "${CONAN_LIB_DIRS_OPENSSL}"
                                  CONAN_PACKAGE_TARGETS_OPENSSL "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES}"
                                  "" openssl)
    set(_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_OPENSSL_DEBUG} ${CONAN_FRAMEWORKS_FOUND_OPENSSL_DEBUG} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_OPENSSL_DEBUG}" "${CONAN_LIB_DIRS_OPENSSL_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_OPENSSL_DEBUG "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_DEBUG}"
                                  "debug" openssl)
    set(_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_OPENSSL_RELEASE} ${CONAN_FRAMEWORKS_FOUND_OPENSSL_RELEASE} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_OPENSSL_RELEASE}" "${CONAN_LIB_DIRS_OPENSSL_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_OPENSSL_RELEASE "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELEASE}"
                                  "release" openssl)
    set(_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_OPENSSL_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_OPENSSL_RELWITHDEBINFO} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_OPENSSL_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_OPENSSL_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_OPENSSL_RELWITHDEBINFO "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" openssl)
    set(_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_OPENSSL_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_OPENSSL_MINSIZEREL} CONAN_PKG::zlib")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_OPENSSL_MINSIZEREL}" "${CONAN_LIB_DIRS_OPENSSL_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_OPENSSL_MINSIZEREL "${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" openssl)

    add_library(CONAN_PKG::openssl INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::openssl PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_OPENSSL} ${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_OPENSSL_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_OPENSSL_RELEASE} ${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_OPENSSL_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_OPENSSL_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_OPENSSL_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_OPENSSL_MINSIZEREL} ${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_OPENSSL_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_OPENSSL_DEBUG} ${_CONAN_PKG_LIBS_OPENSSL_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_OPENSSL_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_OPENSSL_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::openssl PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_OPENSSL}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_OPENSSL_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_OPENSSL_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_OPENSSL_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_OPENSSL_DEBUG}>)
    set_property(TARGET CONAN_PKG::openssl PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_OPENSSL}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_OPENSSL_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_OPENSSL_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_OPENSSL_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_OPENSSL_DEBUG}>)
    set_property(TARGET CONAN_PKG::openssl PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_OPENSSL_LIST} ${CONAN_CXX_FLAGS_OPENSSL_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_OPENSSL_RELEASE_LIST} ${CONAN_CXX_FLAGS_OPENSSL_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_OPENSSL_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_OPENSSL_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_OPENSSL_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_OPENSSL_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_OPENSSL_DEBUG_LIST}  ${CONAN_CXX_FLAGS_OPENSSL_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES "${CONAN_SYSTEM_LIBS_BZIP2} ${CONAN_FRAMEWORKS_FOUND_BZIP2} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BZIP2_DEPENDENCIES "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BZIP2}" "${CONAN_LIB_DIRS_BZIP2}"
                                  CONAN_PACKAGE_TARGETS_BZIP2 "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES}"
                                  "" bzip2)
    set(_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_BZIP2_DEBUG} ${CONAN_FRAMEWORKS_FOUND_BZIP2_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BZIP2_DEBUG}" "${CONAN_LIB_DIRS_BZIP2_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_BZIP2_DEBUG "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_DEBUG}"
                                  "debug" bzip2)
    set(_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_BZIP2_RELEASE} ${CONAN_FRAMEWORKS_FOUND_BZIP2_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BZIP2_RELEASE}" "${CONAN_LIB_DIRS_BZIP2_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_BZIP2_RELEASE "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELEASE}"
                                  "release" bzip2)
    set(_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_BZIP2_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_BZIP2_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BZIP2_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_BZIP2_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_BZIP2_RELWITHDEBINFO "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" bzip2)
    set(_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_BZIP2_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_BZIP2_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_BZIP2_MINSIZEREL}" "${CONAN_LIB_DIRS_BZIP2_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_BZIP2_MINSIZEREL "${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" bzip2)

    add_library(CONAN_PKG::bzip2 INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::bzip2 PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_BZIP2} ${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BZIP2_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_BZIP2_RELEASE} ${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BZIP2_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_BZIP2_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BZIP2_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_BZIP2_MINSIZEREL} ${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BZIP2_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_BZIP2_DEBUG} ${_CONAN_PKG_LIBS_BZIP2_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_BZIP2_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_BZIP2_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::bzip2 PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_BZIP2}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_BZIP2_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_BZIP2_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_BZIP2_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_BZIP2_DEBUG}>)
    set_property(TARGET CONAN_PKG::bzip2 PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_BZIP2}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_BZIP2_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_BZIP2_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_BZIP2_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_BZIP2_DEBUG}>)
    set_property(TARGET CONAN_PKG::bzip2 PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_BZIP2_LIST} ${CONAN_CXX_FLAGS_BZIP2_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_BZIP2_RELEASE_LIST} ${CONAN_CXX_FLAGS_BZIP2_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_BZIP2_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_BZIP2_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_BZIP2_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_BZIP2_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_BZIP2_DEBUG_LIST}  ${CONAN_CXX_FLAGS_BZIP2_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES "${CONAN_SYSTEM_LIBS_LIBBACKTRACE} ${CONAN_FRAMEWORKS_FOUND_LIBBACKTRACE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBBACKTRACE}" "${CONAN_LIB_DIRS_LIBBACKTRACE}"
                                  CONAN_PACKAGE_TARGETS_LIBBACKTRACE "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES}"
                                  "" libbacktrace)
    set(_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_LIBBACKTRACE_DEBUG} ${CONAN_FRAMEWORKS_FOUND_LIBBACKTRACE_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBBACKTRACE_DEBUG}" "${CONAN_LIB_DIRS_LIBBACKTRACE_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_LIBBACKTRACE_DEBUG "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_DEBUG}"
                                  "debug" libbacktrace)
    set(_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_LIBBACKTRACE_RELEASE} ${CONAN_FRAMEWORKS_FOUND_LIBBACKTRACE_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBBACKTRACE_RELEASE}" "${CONAN_LIB_DIRS_LIBBACKTRACE_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_LIBBACKTRACE_RELEASE "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELEASE}"
                                  "release" libbacktrace)
    set(_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_LIBBACKTRACE_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_LIBBACKTRACE_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBBACKTRACE_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_LIBBACKTRACE_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_LIBBACKTRACE_RELWITHDEBINFO "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" libbacktrace)
    set(_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_LIBBACKTRACE_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_LIBBACKTRACE_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_LIBBACKTRACE_MINSIZEREL}" "${CONAN_LIB_DIRS_LIBBACKTRACE_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_LIBBACKTRACE_MINSIZEREL "${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" libbacktrace)

    add_library(CONAN_PKG::libbacktrace INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::libbacktrace PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_LIBBACKTRACE} ${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBBACKTRACE_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_LIBBACKTRACE_RELEASE} ${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBBACKTRACE_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_LIBBACKTRACE_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBBACKTRACE_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_LIBBACKTRACE_MINSIZEREL} ${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBBACKTRACE_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_LIBBACKTRACE_DEBUG} ${_CONAN_PKG_LIBS_LIBBACKTRACE_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_LIBBACKTRACE_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_LIBBACKTRACE_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::libbacktrace PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_LIBBACKTRACE}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_LIBBACKTRACE_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_LIBBACKTRACE_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_LIBBACKTRACE_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_LIBBACKTRACE_DEBUG}>)
    set_property(TARGET CONAN_PKG::libbacktrace PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_LIBBACKTRACE}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_LIBBACKTRACE_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_LIBBACKTRACE_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_LIBBACKTRACE_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_LIBBACKTRACE_DEBUG}>)
    set_property(TARGET CONAN_PKG::libbacktrace PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_LIBBACKTRACE_LIST} ${CONAN_CXX_FLAGS_LIBBACKTRACE_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_LIBBACKTRACE_RELEASE_LIST} ${CONAN_CXX_FLAGS_LIBBACKTRACE_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_LIBBACKTRACE_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_LIBBACKTRACE_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_LIBBACKTRACE_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_LIBBACKTRACE_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_LIBBACKTRACE_DEBUG_LIST}  ${CONAN_CXX_FLAGS_LIBBACKTRACE_DEBUG_LIST}>)


    set(_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES "${CONAN_SYSTEM_LIBS_ZLIB} ${CONAN_FRAMEWORKS_FOUND_ZLIB} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZLIB_DEPENDENCIES "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZLIB}" "${CONAN_LIB_DIRS_ZLIB}"
                                  CONAN_PACKAGE_TARGETS_ZLIB "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES}"
                                  "" zlib)
    set(_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_DEBUG "${CONAN_SYSTEM_LIBS_ZLIB_DEBUG} ${CONAN_FRAMEWORKS_FOUND_ZLIB_DEBUG} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_DEBUG "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_DEBUG}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZLIB_DEBUG}" "${CONAN_LIB_DIRS_ZLIB_DEBUG}"
                                  CONAN_PACKAGE_TARGETS_ZLIB_DEBUG "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_DEBUG}"
                                  "debug" zlib)
    set(_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELEASE "${CONAN_SYSTEM_LIBS_ZLIB_RELEASE} ${CONAN_FRAMEWORKS_FOUND_ZLIB_RELEASE} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELEASE "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELEASE}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZLIB_RELEASE}" "${CONAN_LIB_DIRS_ZLIB_RELEASE}"
                                  CONAN_PACKAGE_TARGETS_ZLIB_RELEASE "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELEASE}"
                                  "release" zlib)
    set(_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELWITHDEBINFO "${CONAN_SYSTEM_LIBS_ZLIB_RELWITHDEBINFO} ${CONAN_FRAMEWORKS_FOUND_ZLIB_RELWITHDEBINFO} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELWITHDEBINFO "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELWITHDEBINFO}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZLIB_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_ZLIB_RELWITHDEBINFO}"
                                  CONAN_PACKAGE_TARGETS_ZLIB_RELWITHDEBINFO "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELWITHDEBINFO}"
                                  "relwithdebinfo" zlib)
    set(_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_MINSIZEREL "${CONAN_SYSTEM_LIBS_ZLIB_MINSIZEREL} ${CONAN_FRAMEWORKS_FOUND_ZLIB_MINSIZEREL} ")
    string(REPLACE " " ";" _CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_MINSIZEREL "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_MINSIZEREL}")
    conan_package_library_targets("${CONAN_PKG_LIBS_ZLIB_MINSIZEREL}" "${CONAN_LIB_DIRS_ZLIB_MINSIZEREL}"
                                  CONAN_PACKAGE_TARGETS_ZLIB_MINSIZEREL "${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_MINSIZEREL}"
                                  "minsizerel" zlib)

    add_library(CONAN_PKG::zlib INTERFACE IMPORTED)

    # Property INTERFACE_LINK_FLAGS do not work, necessary to add to INTERFACE_LINK_LIBRARIES
    set_property(TARGET CONAN_PKG::zlib PROPERTY INTERFACE_LINK_LIBRARIES ${CONAN_PACKAGE_TARGETS_ZLIB} ${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZLIB_LIST}>

                                                                 $<$<CONFIG:Release>:${CONAN_PACKAGE_TARGETS_ZLIB_RELEASE} ${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELEASE}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_RELEASE_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZLIB_RELEASE_LIST}>>

                                                                 $<$<CONFIG:RelWithDebInfo>:${CONAN_PACKAGE_TARGETS_ZLIB_RELWITHDEBINFO} ${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_RELWITHDEBINFO}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_RELWITHDEBINFO_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZLIB_RELWITHDEBINFO_LIST}>>

                                                                 $<$<CONFIG:MinSizeRel>:${CONAN_PACKAGE_TARGETS_ZLIB_MINSIZEREL} ${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_MINSIZEREL}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_MINSIZEREL_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZLIB_MINSIZEREL_LIST}>>

                                                                 $<$<CONFIG:Debug>:${CONAN_PACKAGE_TARGETS_ZLIB_DEBUG} ${_CONAN_PKG_LIBS_ZLIB_DEPENDENCIES_DEBUG}
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,SHARED_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,MODULE_LIBRARY>:${CONAN_SHARED_LINKER_FLAGS_ZLIB_DEBUG_LIST}>
                                                                 $<$<STREQUAL:$<TARGET_PROPERTY:TYPE>,EXECUTABLE>:${CONAN_EXE_LINKER_FLAGS_ZLIB_DEBUG_LIST}>>)
    set_property(TARGET CONAN_PKG::zlib PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CONAN_INCLUDE_DIRS_ZLIB}
                                                                      $<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_ZLIB_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_ZLIB_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_ZLIB_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_ZLIB_DEBUG}>)
    set_property(TARGET CONAN_PKG::zlib PROPERTY INTERFACE_COMPILE_DEFINITIONS ${CONAN_COMPILE_DEFINITIONS_ZLIB}
                                                                      $<$<CONFIG:Release>:${CONAN_COMPILE_DEFINITIONS_ZLIB_RELEASE}>
                                                                      $<$<CONFIG:RelWithDebInfo>:${CONAN_COMPILE_DEFINITIONS_ZLIB_RELWITHDEBINFO}>
                                                                      $<$<CONFIG:MinSizeRel>:${CONAN_COMPILE_DEFINITIONS_ZLIB_MINSIZEREL}>
                                                                      $<$<CONFIG:Debug>:${CONAN_COMPILE_DEFINITIONS_ZLIB_DEBUG}>)
    set_property(TARGET CONAN_PKG::zlib PROPERTY INTERFACE_COMPILE_OPTIONS ${CONAN_C_FLAGS_ZLIB_LIST} ${CONAN_CXX_FLAGS_ZLIB_LIST}
                                                                  $<$<CONFIG:Release>:${CONAN_C_FLAGS_ZLIB_RELEASE_LIST} ${CONAN_CXX_FLAGS_ZLIB_RELEASE_LIST}>
                                                                  $<$<CONFIG:RelWithDebInfo>:${CONAN_C_FLAGS_ZLIB_RELWITHDEBINFO_LIST} ${CONAN_CXX_FLAGS_ZLIB_RELWITHDEBINFO_LIST}>
                                                                  $<$<CONFIG:MinSizeRel>:${CONAN_C_FLAGS_ZLIB_MINSIZEREL_LIST} ${CONAN_CXX_FLAGS_ZLIB_MINSIZEREL_LIST}>
                                                                  $<$<CONFIG:Debug>:${CONAN_C_FLAGS_ZLIB_DEBUG_LIST}  ${CONAN_CXX_FLAGS_ZLIB_DEBUG_LIST}>)

    set(CONAN_TARGETS CONAN_PKG::soci CONAN_PKG::hiredis CONAN_PKG::cpr CONAN_PKG::jwt-cpp CONAN_PKG::cpp-httplib CONAN_PKG::inja CONAN_PKG::tomlplusplus CONAN_PKG::yaml-cpp CONAN_PKG::tinyxml CONAN_PKG::rabbitmq-c CONAN_PKG::cppzmq CONAN_PKG::paho-mqtt-cpp CONAN_PKG::cppcodec CONAN_PKG::flatbuffers CONAN_PKG::grpc CONAN_PKG::serial CONAN_PKG::boost CONAN_PKG::sqlite3 CONAN_PKG::libpq CONAN_PKG::libcurl CONAN_PKG::nlohmann_json CONAN_PKG::zeromq CONAN_PKG::paho-mqtt-c CONAN_PKG::protobuf CONAN_PKG::c-ares CONAN_PKG::abseil CONAN_PKG::re2 CONAN_PKG::openssl CONAN_PKG::bzip2 CONAN_PKG::libbacktrace CONAN_PKG::zlib)

endmacro()


macro(conan_basic_setup)
    set(options TARGETS NO_OUTPUT_DIRS SKIP_RPATH KEEP_RPATHS SKIP_STD SKIP_FPIC)
    cmake_parse_arguments(ARGUMENTS "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN} )

    if(CONAN_EXPORTED)
        conan_message(STATUS "Conan: called by CMake conan helper")
    endif()

    if(CONAN_IN_LOCAL_CACHE)
        conan_message(STATUS "Conan: called inside local cache")
    endif()

    if(NOT ARGUMENTS_NO_OUTPUT_DIRS)
        conan_message(STATUS "Conan: Adjusting output directories")
        conan_output_dirs_setup()
    endif()

    if(NOT ARGUMENTS_TARGETS)
        conan_message(STATUS "Conan: Using cmake global configuration")
        conan_global_flags()
    else()
        conan_message(STATUS "Conan: Using cmake targets configuration")
        conan_define_targets()
    endif()

    if(ARGUMENTS_SKIP_RPATH)
        # Change by "DEPRECATION" or "SEND_ERROR" when we are ready
        conan_message(WARNING "Conan: SKIP_RPATH is deprecated, it has been renamed to KEEP_RPATHS")
    endif()

    if(NOT ARGUMENTS_SKIP_RPATH AND NOT ARGUMENTS_KEEP_RPATHS)
        # Parameter has renamed, but we keep the compatibility with old SKIP_RPATH
        conan_set_rpath()
    endif()

    if(NOT ARGUMENTS_SKIP_STD)
        conan_set_std()
    endif()

    if(NOT ARGUMENTS_SKIP_FPIC)
        conan_set_fpic()
    endif()

    conan_check_compiler()
    conan_set_libcxx()
    conan_set_vs_runtime()
    conan_set_find_paths()
    conan_include_build_modules()
    conan_set_find_library_paths()
endmacro()


macro(conan_set_find_paths)
    # CMAKE_MODULE_PATH does not have Debug/Release config, but there are variables
    # CONAN_CMAKE_MODULE_PATH_DEBUG to be used by the consumer
    # CMake can find findXXX.cmake files in the root of packages
    set(CMAKE_MODULE_PATH ${CONAN_CMAKE_MODULE_PATH} ${CMAKE_MODULE_PATH})

    # Make find_package() to work
    set(CMAKE_PREFIX_PATH ${CONAN_CMAKE_MODULE_PATH} ${CMAKE_PREFIX_PATH})

    # Set the find root path (cross build)
    set(CMAKE_FIND_ROOT_PATH ${CONAN_CMAKE_FIND_ROOT_PATH} ${CMAKE_FIND_ROOT_PATH})
    if(CONAN_CMAKE_FIND_ROOT_PATH_MODE_PROGRAM)
        set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM ${CONAN_CMAKE_FIND_ROOT_PATH_MODE_PROGRAM})
    endif()
    if(CONAN_CMAKE_FIND_ROOT_PATH_MODE_LIBRARY)
        set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ${CONAN_CMAKE_FIND_ROOT_PATH_MODE_LIBRARY})
    endif()
    if(CONAN_CMAKE_FIND_ROOT_PATH_MODE_INCLUDE)
        set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ${CONAN_CMAKE_FIND_ROOT_PATH_MODE_INCLUDE})
    endif()
endmacro()


macro(conan_set_find_library_paths)
    # CMAKE_INCLUDE_PATH, CMAKE_LIBRARY_PATH does not have Debug/Release config, but there are variables
    # CONAN_INCLUDE_DIRS_DEBUG/RELEASE CONAN_LIB_DIRS_DEBUG/RELEASE to be used by the consumer
    # For find_library
    set(CMAKE_INCLUDE_PATH ${CONAN_INCLUDE_DIRS} ${CMAKE_INCLUDE_PATH})
    set(CMAKE_LIBRARY_PATH ${CONAN_LIB_DIRS} ${CMAKE_LIBRARY_PATH})
endmacro()


macro(conan_set_vs_runtime)
    if(CONAN_LINK_RUNTIME)
        conan_get_policy(CMP0091 policy_0091)
        if(policy_0091 STREQUAL "NEW")
            if(CONAN_LINK_RUNTIME MATCHES "MTd")
                set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDebug")
            elseif(CONAN_LINK_RUNTIME MATCHES "MDd")
                set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDebugDLL")
            elseif(CONAN_LINK_RUNTIME MATCHES "MT")
                set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
            elseif(CONAN_LINK_RUNTIME MATCHES "MD")
                set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
            endif()
        else()
            foreach(flag CMAKE_C_FLAGS_RELEASE CMAKE_CXX_FLAGS_RELEASE
                         CMAKE_C_FLAGS_RELWITHDEBINFO CMAKE_CXX_FLAGS_RELWITHDEBINFO
                         CMAKE_C_FLAGS_MINSIZEREL CMAKE_CXX_FLAGS_MINSIZEREL)
                if(DEFINED ${flag})
                    string(REPLACE "/MD" ${CONAN_LINK_RUNTIME} ${flag} "${${flag}}")
                endif()
            endforeach()
            foreach(flag CMAKE_C_FLAGS_DEBUG CMAKE_CXX_FLAGS_DEBUG)
                if(DEFINED ${flag})
                    string(REPLACE "/MDd" ${CONAN_LINK_RUNTIME} ${flag} "${${flag}}")
                endif()
            endforeach()
        endif()
    endif()
endmacro()


macro(conan_flags_setup)
    # Macro maintained for backwards compatibility
    conan_set_find_library_paths()
    conan_global_flags()
    conan_set_rpath()
    conan_set_vs_runtime()
    conan_set_libcxx()
endmacro()


function(conan_message MESSAGE_OUTPUT)
    if(NOT CONAN_CMAKE_SILENT_OUTPUT)
        message(${ARGV${0}})
    endif()
endfunction()


function(conan_get_policy policy_id policy)
    if(POLICY "${policy_id}")
        cmake_policy(GET "${policy_id}" _policy)
        set(${policy} "${_policy}" PARENT_SCOPE)
    else()
        set(${policy} "" PARENT_SCOPE)
    endif()
endfunction()


function(conan_find_libraries_abs_path libraries package_libdir libraries_abs_path)
    foreach(_LIBRARY_NAME ${libraries})
        find_library(CONAN_FOUND_LIBRARY NAME ${_LIBRARY_NAME} PATHS ${package_libdir}
                     NO_DEFAULT_PATH NO_CMAKE_FIND_ROOT_PATH)
        if(CONAN_FOUND_LIBRARY)
            conan_message(STATUS "Library ${_LIBRARY_NAME} found ${CONAN_FOUND_LIBRARY}")
            set(CONAN_FULLPATH_LIBS ${CONAN_FULLPATH_LIBS} ${CONAN_FOUND_LIBRARY})
        else()
            conan_message(STATUS "Library ${_LIBRARY_NAME} not found in package, might be system one")
            set(CONAN_FULLPATH_LIBS ${CONAN_FULLPATH_LIBS} ${_LIBRARY_NAME})
        endif()
        unset(CONAN_FOUND_LIBRARY CACHE)
    endforeach()
    set(${libraries_abs_path} ${CONAN_FULLPATH_LIBS} PARENT_SCOPE)
endfunction()


function(conan_package_library_targets libraries package_libdir libraries_abs_path deps build_type package_name)
    unset(_CONAN_ACTUAL_TARGETS CACHE)
    unset(_CONAN_FOUND_SYSTEM_LIBS CACHE)
    foreach(_LIBRARY_NAME ${libraries})
        find_library(CONAN_FOUND_LIBRARY NAME ${_LIBRARY_NAME} PATHS ${package_libdir}
                     NO_DEFAULT_PATH NO_CMAKE_FIND_ROOT_PATH)
        if(CONAN_FOUND_LIBRARY)
            conan_message(STATUS "Library ${_LIBRARY_NAME} found ${CONAN_FOUND_LIBRARY}")
            set(_LIB_NAME CONAN_LIB::${package_name}_${_LIBRARY_NAME}${build_type})
            add_library(${_LIB_NAME} UNKNOWN IMPORTED)
            set_target_properties(${_LIB_NAME} PROPERTIES IMPORTED_LOCATION ${CONAN_FOUND_LIBRARY})
            set(CONAN_FULLPATH_LIBS ${CONAN_FULLPATH_LIBS} ${_LIB_NAME})
            set(_CONAN_ACTUAL_TARGETS ${_CONAN_ACTUAL_TARGETS} ${_LIB_NAME})
        else()
            conan_message(STATUS "Library ${_LIBRARY_NAME} not found in package, might be system one")
            set(CONAN_FULLPATH_LIBS ${CONAN_FULLPATH_LIBS} ${_LIBRARY_NAME})
            set(_CONAN_FOUND_SYSTEM_LIBS "${_CONAN_FOUND_SYSTEM_LIBS};${_LIBRARY_NAME}")
        endif()
        unset(CONAN_FOUND_LIBRARY CACHE)
    endforeach()

    # Add all dependencies to all targets
    string(REPLACE " " ";" deps_list "${deps}")
    foreach(_CONAN_ACTUAL_TARGET ${_CONAN_ACTUAL_TARGETS})
        set_property(TARGET ${_CONAN_ACTUAL_TARGET} PROPERTY INTERFACE_LINK_LIBRARIES "${_CONAN_FOUND_SYSTEM_LIBS};${deps_list}")
    endforeach()

    set(${libraries_abs_path} ${CONAN_FULLPATH_LIBS} PARENT_SCOPE)
endfunction()


macro(conan_set_libcxx)
    if(DEFINED CONAN_LIBCXX)
        conan_message(STATUS "Conan: C++ stdlib: ${CONAN_LIBCXX}")
        if(CONAN_COMPILER STREQUAL "clang" OR CONAN_COMPILER STREQUAL "apple-clang")
            if(CONAN_LIBCXX STREQUAL "libstdc++" OR CONAN_LIBCXX STREQUAL "libstdc++11" )
                set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -stdlib=libstdc++")
            elseif(CONAN_LIBCXX STREQUAL "libc++")
                set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -stdlib=libc++")
            endif()
        endif()
        if(CONAN_COMPILER STREQUAL "sun-cc")
            if(CONAN_LIBCXX STREQUAL "libCstd")
                set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -library=Cstd")
            elseif(CONAN_LIBCXX STREQUAL "libstdcxx")
                set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -library=stdcxx4")
            elseif(CONAN_LIBCXX STREQUAL "libstlport")
                set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -library=stlport4")
            elseif(CONAN_LIBCXX STREQUAL "libstdc++")
                set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -library=stdcpp")
            endif()
        endif()
        if(CONAN_LIBCXX STREQUAL "libstdc++11")
            add_definitions(-D_GLIBCXX_USE_CXX11_ABI=1)
        elseif(CONAN_LIBCXX STREQUAL "libstdc++")
            add_definitions(-D_GLIBCXX_USE_CXX11_ABI=0)
        endif()
    endif()
endmacro()


macro(conan_set_std)
    conan_message(STATUS "Conan: Adjusting language standard")
    # Do not warn "Manually-specified variables were not used by the project"
    set(ignorevar "${CONAN_STD_CXX_FLAG}${CONAN_CMAKE_CXX_STANDARD}${CONAN_CMAKE_CXX_EXTENSIONS}")
    if (CMAKE_VERSION VERSION_LESS "3.1" OR
        (CMAKE_VERSION VERSION_LESS "3.12" AND ("${CONAN_CMAKE_CXX_STANDARD}" STREQUAL "20" OR "${CONAN_CMAKE_CXX_STANDARD}" STREQUAL "gnu20")))
        if(CONAN_STD_CXX_FLAG)
            conan_message(STATUS "Conan setting CXX_FLAGS flags: ${CONAN_STD_CXX_FLAG}")
            set(CMAKE_CXX_FLAGS "${CONAN_STD_CXX_FLAG} ${CMAKE_CXX_FLAGS}")
        endif()
    else()
        if(CONAN_CMAKE_CXX_STANDARD)
            conan_message(STATUS "Conan setting CPP STANDARD: ${CONAN_CMAKE_CXX_STANDARD} WITH EXTENSIONS ${CONAN_CMAKE_CXX_EXTENSIONS}")
            set(CMAKE_CXX_STANDARD ${CONAN_CMAKE_CXX_STANDARD})
            set(CMAKE_CXX_EXTENSIONS ${CONAN_CMAKE_CXX_EXTENSIONS})
        endif()
    endif()
endmacro()


macro(conan_set_rpath)
    conan_message(STATUS "Conan: Adjusting default RPATHs Conan policies")
    if(APPLE)
        # https://cmake.org/Wiki/CMake_RPATH_handling
        # CONAN GUIDE: All generated libraries should have the id and dependencies to other
        # dylibs without path, just the name, EX:
        # libMyLib1.dylib:
        #     libMyLib1.dylib (compatibility version 0.0.0, current version 0.0.0)
        #     libMyLib0.dylib (compatibility version 0.0.0, current version 0.0.0)
        #     /usr/lib/libc++.1.dylib (compatibility version 1.0.0, current version 120.0.0)
        #     /usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1197.1.1)
        # AVOID RPATH FOR *.dylib, ALL LIBS BETWEEN THEM AND THE EXE
        # SHOULD BE ON THE LINKER RESOLVER PATH (./ IS ONE OF THEM)
        set(CMAKE_SKIP_RPATH 1 CACHE BOOL "rpaths" FORCE)
        # Policy CMP0068
        # We want the old behavior, in CMake >= 3.9 CMAKE_SKIP_RPATH won't affect the install_name in OSX
        set(CMAKE_INSTALL_NAME_DIR "")
    endif()
endmacro()


macro(conan_set_fpic)
    if(DEFINED CONAN_CMAKE_POSITION_INDEPENDENT_CODE)
        conan_message(STATUS "Conan: Adjusting fPIC flag (${CONAN_CMAKE_POSITION_INDEPENDENT_CODE})")
        set(CMAKE_POSITION_INDEPENDENT_CODE ${CONAN_CMAKE_POSITION_INDEPENDENT_CODE})
    endif()
endmacro()


macro(conan_output_dirs_setup)
    set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/bin)
    set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})
    set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELWITHDEBINFO ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})
    set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_MINSIZEREL ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})
    set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})

    set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/lib)
    set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY_RELEASE ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY})
    set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY_RELWITHDEBINFO ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY})
    set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY_MINSIZEREL ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY})
    set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY_DEBUG ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY})

    set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/lib)
    set(CMAKE_LIBRARY_OUTPUT_DIRECTORY_RELEASE ${CMAKE_LIBRARY_OUTPUT_DIRECTORY})
    set(CMAKE_LIBRARY_OUTPUT_DIRECTORY_RELWITHDEBINFO ${CMAKE_LIBRARY_OUTPUT_DIRECTORY})
    set(CMAKE_LIBRARY_OUTPUT_DIRECTORY_MINSIZEREL ${CMAKE_LIBRARY_OUTPUT_DIRECTORY})
    set(CMAKE_LIBRARY_OUTPUT_DIRECTORY_DEBUG ${CMAKE_LIBRARY_OUTPUT_DIRECTORY})
endmacro()


macro(conan_split_version VERSION_STRING MAJOR MINOR)
    #make a list from the version string
    string(REPLACE "." ";" VERSION_LIST "${VERSION_STRING}")

    #write output values
    list(LENGTH VERSION_LIST _version_len)
    list(GET VERSION_LIST 0 ${MAJOR})
    if(${_version_len} GREATER 1)
        list(GET VERSION_LIST 1 ${MINOR})
    endif()
endmacro()


macro(conan_error_compiler_version)
    message(FATAL_ERROR "Detected a mismatch for the compiler version between your conan profile settings and CMake: \n"
                        "Compiler version specified in your conan profile: ${CONAN_COMPILER_VERSION}\n"
                        "Compiler version detected in CMake: ${VERSION_MAJOR}.${VERSION_MINOR}\n"
                        "Please check your conan profile settings (conan profile show [default|your_profile_name])\n"
                        "P.S. You may set CONAN_DISABLE_CHECK_COMPILER CMake variable in order to disable this check."
           )
endmacro()

set(_CONAN_CURRENT_DIR ${CMAKE_CURRENT_LIST_DIR})

function(conan_get_compiler CONAN_INFO_COMPILER CONAN_INFO_COMPILER_VERSION)
    conan_message(STATUS "Current conanbuildinfo.cmake directory: " ${_CONAN_CURRENT_DIR})
    if(NOT EXISTS ${_CONAN_CURRENT_DIR}/conaninfo.txt)
        conan_message(STATUS "WARN: conaninfo.txt not found")
        return()
    endif()

    file (READ "${_CONAN_CURRENT_DIR}/conaninfo.txt" CONANINFO)

    # MATCHALL will match all, including the last one, which is the full_settings one
    string(REGEX MATCH "full_settings.*" _FULL_SETTINGS_MATCHED ${CONANINFO})
    string(REGEX MATCH "compiler=([-A-Za-z0-9_ ]+)" _MATCHED ${_FULL_SETTINGS_MATCHED})
    if(DEFINED CMAKE_MATCH_1)
        string(STRIP "${CMAKE_MATCH_1}" _CONAN_INFO_COMPILER)
        set(${CONAN_INFO_COMPILER} ${_CONAN_INFO_COMPILER} PARENT_SCOPE)
    endif()

    string(REGEX MATCH "compiler.version=([-A-Za-z0-9_.]+)" _MATCHED ${_FULL_SETTINGS_MATCHED})
    if(DEFINED CMAKE_MATCH_1)
        string(STRIP "${CMAKE_MATCH_1}" _CONAN_INFO_COMPILER_VERSION)
        set(${CONAN_INFO_COMPILER_VERSION} ${_CONAN_INFO_COMPILER_VERSION} PARENT_SCOPE)
    endif()
endfunction()


function(check_compiler_version)
    conan_split_version(${CMAKE_CXX_COMPILER_VERSION} VERSION_MAJOR VERSION_MINOR)
    if(DEFINED CONAN_SETTINGS_COMPILER_TOOLSET)
       conan_message(STATUS "Conan: Skipping compiler check: Declared 'compiler.toolset'")
       return()
    endif()
    if(CMAKE_CXX_COMPILER_ID MATCHES MSVC)
        # MSVC_VERSION is defined since 2.8.2 at least
        # https://cmake.org/cmake/help/v2.8.2/cmake.html#variable:MSVC_VERSION
        # https://cmake.org/cmake/help/v3.14/variable/MSVC_VERSION.html
        if(
            # 1930 = VS 17.0 (v143 toolset)
            (CONAN_COMPILER_VERSION STREQUAL "17" AND NOT MSVC_VERSION EQUAL 1930) OR
            # 1920-1929 = VS 16.0 (v142 toolset)
            (CONAN_COMPILER_VERSION STREQUAL "16" AND NOT((MSVC_VERSION GREATER 1919) AND (MSVC_VERSION LESS 1930))) OR
            # 1910-1919 = VS 15.0 (v141 toolset)
            (CONAN_COMPILER_VERSION STREQUAL "15" AND NOT((MSVC_VERSION GREATER 1909) AND (MSVC_VERSION LESS 1920))) OR
            # 1900      = VS 14.0 (v140 toolset)
            (CONAN_COMPILER_VERSION STREQUAL "14" AND NOT(MSVC_VERSION EQUAL 1900)) OR
            # 1800      = VS 12.0 (v120 toolset)
            (CONAN_COMPILER_VERSION STREQUAL "12" AND NOT VERSION_MAJOR STREQUAL "18") OR
            # 1700      = VS 11.0 (v110 toolset)
            (CONAN_COMPILER_VERSION STREQUAL "11" AND NOT VERSION_MAJOR STREQUAL "17") OR
            # 1600      = VS 10.0 (v100 toolset)
            (CONAN_COMPILER_VERSION STREQUAL "10" AND NOT VERSION_MAJOR STREQUAL "16") OR
            # 1500      = VS  9.0 (v90 toolset)
            (CONAN_COMPILER_VERSION STREQUAL "9" AND NOT VERSION_MAJOR STREQUAL "15") OR
            # 1400      = VS  8.0 (v80 toolset)
            (CONAN_COMPILER_VERSION STREQUAL "8" AND NOT VERSION_MAJOR STREQUAL "14") OR
            # 1310      = VS  7.1, 1300      = VS  7.0
            (CONAN_COMPILER_VERSION STREQUAL "7" AND NOT VERSION_MAJOR STREQUAL "13") OR
            # 1200      = VS  6.0
            (CONAN_COMPILER_VERSION STREQUAL "6" AND NOT VERSION_MAJOR STREQUAL "12") )
            conan_error_compiler_version()
        endif()
    elseif(CONAN_COMPILER STREQUAL "gcc")
        conan_split_version(${CONAN_COMPILER_VERSION} CONAN_COMPILER_MAJOR CONAN_COMPILER_MINOR)
        set(_CHECK_VERSION ${VERSION_MAJOR}.${VERSION_MINOR})
        set(_CONAN_VERSION ${CONAN_COMPILER_MAJOR}.${CONAN_COMPILER_MINOR})
        if(NOT ${CONAN_COMPILER_VERSION} VERSION_LESS 5.0)
            conan_message(STATUS "Conan: Compiler GCC>=5, checking major version ${CONAN_COMPILER_VERSION}")
            conan_split_version(${CONAN_COMPILER_VERSION} CONAN_COMPILER_MAJOR CONAN_COMPILER_MINOR)
            if("${CONAN_COMPILER_MINOR}" STREQUAL "")
                set(_CHECK_VERSION ${VERSION_MAJOR})
                set(_CONAN_VERSION ${CONAN_COMPILER_MAJOR})
            endif()
        endif()
        conan_message(STATUS "Conan: Checking correct version: ${_CHECK_VERSION}")
        if(NOT ${_CHECK_VERSION} VERSION_EQUAL ${_CONAN_VERSION})
            conan_error_compiler_version()
        endif()
    elseif(CONAN_COMPILER STREQUAL "clang")
        conan_split_version(${CONAN_COMPILER_VERSION} CONAN_COMPILER_MAJOR CONAN_COMPILER_MINOR)
        set(_CHECK_VERSION ${VERSION_MAJOR}.${VERSION_MINOR})
        set(_CONAN_VERSION ${CONAN_COMPILER_MAJOR}.${CONAN_COMPILER_MINOR})
        if(NOT ${CONAN_COMPILER_VERSION} VERSION_LESS 8.0)
            conan_message(STATUS "Conan: Compiler Clang>=8, checking major version ${CONAN_COMPILER_VERSION}")
            if("${CONAN_COMPILER_MINOR}" STREQUAL "")
                set(_CHECK_VERSION ${VERSION_MAJOR})
                set(_CONAN_VERSION ${CONAN_COMPILER_MAJOR})
            endif()
        endif()
        conan_message(STATUS "Conan: Checking correct version: ${_CHECK_VERSION}")
        if(NOT ${_CHECK_VERSION} VERSION_EQUAL ${_CONAN_VERSION})
            conan_error_compiler_version()
        endif()
    elseif(CONAN_COMPILER STREQUAL "apple-clang" OR CONAN_COMPILER STREQUAL "sun-cc" OR CONAN_COMPILER STREQUAL "mcst-lcc")
        conan_split_version(${CONAN_COMPILER_VERSION} CONAN_COMPILER_MAJOR CONAN_COMPILER_MINOR)
        if(NOT ${VERSION_MAJOR}.${VERSION_MINOR} VERSION_EQUAL ${CONAN_COMPILER_MAJOR}.${CONAN_COMPILER_MINOR})
           conan_error_compiler_version()
        endif()
    elseif(CONAN_COMPILER STREQUAL "intel")
        conan_split_version(${CONAN_COMPILER_VERSION} CONAN_COMPILER_MAJOR CONAN_COMPILER_MINOR)
        if(NOT ${CONAN_COMPILER_VERSION} VERSION_LESS 19.1)
            if(NOT ${VERSION_MAJOR}.${VERSION_MINOR} VERSION_EQUAL ${CONAN_COMPILER_MAJOR}.${CONAN_COMPILER_MINOR})
               conan_error_compiler_version()
            endif()
        else()
            if(NOT ${VERSION_MAJOR} VERSION_EQUAL ${CONAN_COMPILER_MAJOR})
               conan_error_compiler_version()
            endif()
        endif()
    else()
        conan_message(STATUS "WARN: Unknown compiler '${CONAN_COMPILER}', skipping the version check...")
    endif()
endfunction()


function(conan_check_compiler)
    if(CONAN_DISABLE_CHECK_COMPILER)
        conan_message(STATUS "WARN: Disabled conan compiler checks")
        return()
    endif()
    if(NOT DEFINED CMAKE_CXX_COMPILER_ID)
        if(DEFINED CMAKE_C_COMPILER_ID)
            conan_message(STATUS "This project seems to be plain C, using '${CMAKE_C_COMPILER_ID}' compiler")
            set(CMAKE_CXX_COMPILER_ID ${CMAKE_C_COMPILER_ID})
            set(CMAKE_CXX_COMPILER_VERSION ${CMAKE_C_COMPILER_VERSION})
        else()
            message(FATAL_ERROR "This project seems to be plain C, but no compiler defined")
        endif()
    endif()
    if(NOT CMAKE_CXX_COMPILER_ID AND NOT CMAKE_C_COMPILER_ID)
        # This use case happens when compiler is not identified by CMake, but the compilers are there and work
        conan_message(STATUS "*** WARN: CMake was not able to identify a C or C++ compiler ***")
        conan_message(STATUS "*** WARN: Disabling compiler checks. Please make sure your settings match your environment ***")
        return()
    endif()
    if(NOT DEFINED CONAN_COMPILER)
        conan_get_compiler(CONAN_COMPILER CONAN_COMPILER_VERSION)
        if(NOT DEFINED CONAN_COMPILER)
            conan_message(STATUS "WARN: CONAN_COMPILER variable not set, please make sure yourself that "
                          "your compiler and version matches your declared settings")
            return()
        endif()
    endif()

    if(NOT CMAKE_HOST_SYSTEM_NAME STREQUAL ${CMAKE_SYSTEM_NAME})
        set(CROSS_BUILDING 1)
    endif()

    # If using VS, verify toolset
    if (CONAN_COMPILER STREQUAL "Visual Studio")
        if (CONAN_SETTINGS_COMPILER_TOOLSET MATCHES "LLVM" OR
            CONAN_SETTINGS_COMPILER_TOOLSET MATCHES "llvm" OR
            CONAN_SETTINGS_COMPILER_TOOLSET MATCHES "clang" OR
            CONAN_SETTINGS_COMPILER_TOOLSET MATCHES "Clang")
            set(EXPECTED_CMAKE_CXX_COMPILER_ID "Clang")
        elseif (CONAN_SETTINGS_COMPILER_TOOLSET MATCHES "Intel")
            set(EXPECTED_CMAKE_CXX_COMPILER_ID "Intel")
        else()
            set(EXPECTED_CMAKE_CXX_COMPILER_ID "MSVC")
        endif()

        if (NOT CMAKE_CXX_COMPILER_ID MATCHES ${EXPECTED_CMAKE_CXX_COMPILER_ID})
            message(FATAL_ERROR "Incorrect '${CONAN_COMPILER}'. Toolset specifies compiler as '${EXPECTED_CMAKE_CXX_COMPILER_ID}' "
                                "but CMake detected '${CMAKE_CXX_COMPILER_ID}'")
        endif()

    # Avoid checks when cross compiling, apple-clang crashes because its APPLE but not apple-clang
    # Actually CMake is detecting "clang" when you are using apple-clang, only if CMP0025 is set to NEW will detect apple-clang
    elseif((CONAN_COMPILER STREQUAL "gcc" AND NOT CMAKE_CXX_COMPILER_ID MATCHES "GNU") OR
        (CONAN_COMPILER STREQUAL "apple-clang" AND NOT CROSS_BUILDING AND (NOT APPLE OR NOT CMAKE_CXX_COMPILER_ID MATCHES "Clang")) OR
        (CONAN_COMPILER STREQUAL "clang" AND NOT CMAKE_CXX_COMPILER_ID MATCHES "Clang") OR
        (CONAN_COMPILER STREQUAL "sun-cc" AND NOT CMAKE_CXX_COMPILER_ID MATCHES "SunPro") )
        message(FATAL_ERROR "Incorrect '${CONAN_COMPILER}', is not the one detected by CMake: '${CMAKE_CXX_COMPILER_ID}'")
    endif()


    if(NOT DEFINED CONAN_COMPILER_VERSION)
        conan_message(STATUS "WARN: CONAN_COMPILER_VERSION variable not set, please make sure yourself "
                             "that your compiler version matches your declared settings")
        return()
    endif()
    check_compiler_version()
endfunction()


macro(conan_set_flags build_type)
    set(CMAKE_CXX_FLAGS${build_type} "${CMAKE_CXX_FLAGS${build_type}} ${CONAN_CXX_FLAGS${build_type}}")
    set(CMAKE_C_FLAGS${build_type} "${CMAKE_C_FLAGS${build_type}} ${CONAN_C_FLAGS${build_type}}")
    set(CMAKE_SHARED_LINKER_FLAGS${build_type} "${CMAKE_SHARED_LINKER_FLAGS${build_type}} ${CONAN_SHARED_LINKER_FLAGS${build_type}}")
    set(CMAKE_EXE_LINKER_FLAGS${build_type} "${CMAKE_EXE_LINKER_FLAGS${build_type}} ${CONAN_EXE_LINKER_FLAGS${build_type}}")
endmacro()


macro(conan_global_flags)
    if(CONAN_SYSTEM_INCLUDES)
        include_directories(SYSTEM ${CONAN_INCLUDE_DIRS}
                                   "$<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_RELEASE}>"
                                   "$<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_RELWITHDEBINFO}>"
                                   "$<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_MINSIZEREL}>"
                                   "$<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_DEBUG}>")
    else()
        include_directories(${CONAN_INCLUDE_DIRS}
                            "$<$<CONFIG:Release>:${CONAN_INCLUDE_DIRS_RELEASE}>"
                            "$<$<CONFIG:RelWithDebInfo>:${CONAN_INCLUDE_DIRS_RELWITHDEBINFO}>"
                            "$<$<CONFIG:MinSizeRel>:${CONAN_INCLUDE_DIRS_MINSIZEREL}>"
                            "$<$<CONFIG:Debug>:${CONAN_INCLUDE_DIRS_DEBUG}>")
    endif()

    link_directories(${CONAN_LIB_DIRS})

    conan_find_libraries_abs_path("${CONAN_LIBS_DEBUG}" "${CONAN_LIB_DIRS_DEBUG}"
                                  CONAN_LIBS_DEBUG)
    conan_find_libraries_abs_path("${CONAN_LIBS_RELEASE}" "${CONAN_LIB_DIRS_RELEASE}"
                                  CONAN_LIBS_RELEASE)
    conan_find_libraries_abs_path("${CONAN_LIBS_RELWITHDEBINFO}" "${CONAN_LIB_DIRS_RELWITHDEBINFO}"
                                  CONAN_LIBS_RELWITHDEBINFO)
    conan_find_libraries_abs_path("${CONAN_LIBS_MINSIZEREL}" "${CONAN_LIB_DIRS_MINSIZEREL}"
                                  CONAN_LIBS_MINSIZEREL)

    add_compile_options(${CONAN_DEFINES}
                        "$<$<CONFIG:Debug>:${CONAN_DEFINES_DEBUG}>"
                        "$<$<CONFIG:Release>:${CONAN_DEFINES_RELEASE}>"
                        "$<$<CONFIG:RelWithDebInfo>:${CONAN_DEFINES_RELWITHDEBINFO}>"
                        "$<$<CONFIG:MinSizeRel>:${CONAN_DEFINES_MINSIZEREL}>")

    conan_set_flags("")
    conan_set_flags("_RELEASE")
    conan_set_flags("_DEBUG")

endmacro()


macro(conan_target_link_libraries target)
    if(CONAN_TARGETS)
        target_link_libraries(${target} ${CONAN_TARGETS})
    else()
        target_link_libraries(${target} ${CONAN_LIBS})
        foreach(_LIB ${CONAN_LIBS_RELEASE})
            target_link_libraries(${target} optimized ${_LIB})
        endforeach()
        foreach(_LIB ${CONAN_LIBS_DEBUG})
            target_link_libraries(${target} debug ${_LIB})
        endforeach()
    endif()
endmacro()


macro(conan_include_build_modules)
    if(CMAKE_BUILD_TYPE)
        if(${CMAKE_BUILD_TYPE} MATCHES "Debug")
            set(CONAN_BUILD_MODULES_PATHS ${CONAN_BUILD_MODULES_PATHS_DEBUG} ${CONAN_BUILD_MODULES_PATHS})
        elseif(${CMAKE_BUILD_TYPE} MATCHES "Release")
            set(CONAN_BUILD_MODULES_PATHS ${CONAN_BUILD_MODULES_PATHS_RELEASE} ${CONAN_BUILD_MODULES_PATHS})
        elseif(${CMAKE_BUILD_TYPE} MATCHES "RelWithDebInfo")
            set(CONAN_BUILD_MODULES_PATHS ${CONAN_BUILD_MODULES_PATHS_RELWITHDEBINFO} ${CONAN_BUILD_MODULES_PATHS})
        elseif(${CMAKE_BUILD_TYPE} MATCHES "MinSizeRel")
            set(CONAN_BUILD_MODULES_PATHS ${CONAN_BUILD_MODULES_PATHS_MINSIZEREL} ${CONAN_BUILD_MODULES_PATHS})
        endif()
    endif()

    foreach(_BUILD_MODULE_PATH ${CONAN_BUILD_MODULES_PATHS})
        include(${_BUILD_MODULE_PATH})
    endforeach()
endmacro()


### Definition of user declared vars (user_info) ###

set(CONAN_USER_BOOST_stacktrace_addr2line_available "True")