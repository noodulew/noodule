#include "nodule/nodule.hpp"

extern "C" int nodule_cpp_hello(void) {
    return 42;
}

extern "C" const char* nodule_cpp_version_name(void) {
    return "0.1a";
}