#include <cstring>
#define TCPP_IMPLEMENTATION
#include "preprocessor.h"
#include <cstdio>
#include <iostream>

using namespace tcpp;

// std::string ErrorTypeToString(const E_ERROR_TYPE &errorType) TCPP_NOEXCEPT;

extern "C"
{
    void delete_char_arr(char *ptr)
    {
        delete[] ptr;
    }

    const char *error_type_to_string(const E_ERROR_TYPE errorType)
    {
        auto source = ErrorTypeToString(errorType);
        char *cstr = new char[source.size() + 1];
        strcpy(cstr, source.c_str());
        return cstr;
    }

    IInputStream *create_input_stream(const char *data) TCPP_NOEXCEPT
    {
        std::string source(data);
        return new StringInputStream(source);
    }

    /// Template from preprocessor.h (with additional callbacks)
    char *process_with_specs(char *data, const void *callback, void (*error)(const void *, const TErrorInfo &), IInputStream *(*_include)(const void *, const char *, bool))
    {
        std::string sdata(data);
        Lexer lex(std::make_unique<StringInputStream>(sdata));
        Preprocessor::TPreprocessorConfigInfo config;
        if (error)
        {
            config.mOnErrorCallback = [callback, error](auto err)
            {
                return error(callback, err);
            };
        }
        if (_include)
        {
            config.mOnIncludeCallback = [callback, _include](auto name, auto boolean)
            {
                return std::unique_ptr<IInputStream>(_include(callback, name.c_str(), boolean));
            };
        }
        Preprocessor pre(lex,
                         config);
        auto x = pre.Process();
        char *cstr = new char[x.size() + 1];
        strcpy(cstr, x.c_str());
        return cstr;
    }

    char *process(char *data)
    {
        return process_with_specs(data, nullptr, nullptr, nullptr);
    }
}
