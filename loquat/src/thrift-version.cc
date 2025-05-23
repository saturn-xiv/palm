#include "loquat/env.hpp"

#include <thrift/config.h>

std::string loquat::thrift_version() {
    return PACKAGE_VERSION;
}
