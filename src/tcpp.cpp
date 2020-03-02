#include <cstring>
#define TCPP_IMPLEMENTATION
#include "preprocessor.h"
#include <cstdio>

std::string ErrorTypeToString(const E_ERROR_TYPE &errorType) TCPP_NOEXCEPT;

extern "C" {

    char *process(char *data);

    const char* error_type_to_string(const E_ERROR_TYPE errorType) {
        auto source = new std::string(ErrorTypeToString(errorType).c_str());
        return source->c_str();
    }

    IInputStream* create_input_stream(const char* data) TCPP_NOEXCEPT {
        auto source = new std::string(data);
        return new StringInputStream((std::string &) source);
    }

    /// Template from preprocessor.h (with additional callbacks)
    char *process_with_specs(char *data
            , const void* callback
            , void (*error)(const void* ,const TErrorInfo &)
            , IInputStream * (*_include)(const void*, const char*, bool)) {
        std::string sdata(data);
        StringInputStream sin(sdata);
        Lexer lex(sin);
        Preprocessor pre(lex,
                [callback, error] (auto err) {
                return error(callback, err);
            } , [callback, _include] (auto name, auto boolean) {
                return _include(callback, name.c_str(), boolean);
            });
        auto x = pre.Process();
        char *cstr = new char[sdata.length() + 1];
        strcpy(cstr, x.c_str());
        return cstr;
    }

}
