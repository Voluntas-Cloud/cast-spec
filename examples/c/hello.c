/*
 * cast-spec example: a small C program that exercises every kind of
 * anchor the C handler knows how to find — function definitions
 * (`main`, `format_greeting`, `print_greeting`), a `#define`
 * constant (`MAX_GREETING`), and a `struct` declaration (`greeter`).
 *
 * Each anchor is referenced from a `cast::io::continues_in!` in
 * crates/cast-spec/src/examples.rs, so cast-watch's `walk` query
 * starting from concept `c_examples` returns this file's five
 * anchorable items as io_bridges. That is what "walking the
 * foreign language" looks like in cast: starting from a concept,
 * listing every bridge into foreign source the concept declares.
 */

#include <stdio.h>
#include <string.h>

#define MAX_GREETING 80

struct greeter {
    const char *language;
    const char *salutation;
};

static const struct greeter en_greeter = {
    .language   = "en",
    .salutation = "Hello",
};

void print_greeting(const char *msg) {
    printf("%s\n", msg);
}

void format_greeting(char *out, size_t cap,
                     const struct greeter *g, const char *name) {
    snprintf(out, cap, "%s, %s!", g->salutation, name);
}

int main(int argc, char **argv) {
    const char *name = argc > 1 ? argv[1] : "World";
    char msg[MAX_GREETING];
    format_greeting(msg, sizeof(msg), &en_greeter, name);
    print_greeting(msg);
    return 0;
}
