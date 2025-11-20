#include "log.h"
#include "vec.h"

int main(void) {
    Vector<int> vec;
    vec.push(1);
    vec.push(2);
    vec.push(3);

    if (!vec[0]) {
        log_to_out(LogLevel::ERROR, "Value is NULL");
    } else {
        log_to_out(LogLevel::INFO, "Value is " + std::to_string(vec[0]));

        if (vec[0] != 1) { log_to_out(LogLevel::WARN, "Unexpected value"); }
    }

    return 0;
}
