#pragma once

#include <iostream>

constexpr const char *DEBUG_COLOR = "\033[36m";
constexpr const char *INFO_COLOR = "\033[32m";
constexpr const char *WARN_COLOR = "\033[33m";
constexpr const char *ERROR_COLOR = "\033[31m";
constexpr const char *RESET_COLOR = "\033[0m";

enum class LogLevel {
    INFO,
    WARN,
    ERROR,
};

extern "C" void log_to_out(LogLevel level, const std::string &msg);
