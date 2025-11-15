#include <dlfcn.h>
#include <iostream>

typedef void (*module_main_t)();

int main(void) {
    void *lib_so = dlopen("./lib.so", RTLD_NOW);
    if (!lib_so) {
        std::cerr << "dlopen error: " << dlerror() << "\n";
        return 1;
    }

    dlerror();

    module_main_t module_main = (module_main_t)dlsym(lib_so, "module_main");
    const char *err = dlerror();
    if (err) {
        std::cerr << "dlsym error: " << err << "\n";
        return 1;
    }
    module_main();

    dlclose(lib_so);
    return 0;
}
