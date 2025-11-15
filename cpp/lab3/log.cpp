#include "log.h"

void log_to_out(LogLevel level, const std::string &msg) {
    const char *color;
    const char *prefix;

    switch (level) {
    case LogLevel::INFO:
        color = INFO_COLOR;
        prefix = "[INFO]";
        break;
    case LogLevel::WARN:
        color = WARN_COLOR;
        prefix = "[WARN]";
        break;
    case LogLevel::ERROR:
        color = ERROR_COLOR;
        prefix = "[ERROR]";
        break;
    default:
        color = RESET_COLOR;
        prefix = "[LOG]";
        break;
    }

    std::cout << color << prefix << RESET_COLOR << " " << msg << std::endl;
}
