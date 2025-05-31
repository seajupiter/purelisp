#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long i64;
typedef double f64;

enum PLType {
    PL_NIL,
    PL_BOOL,
    PL_INT,
    PL_FLOAT,
    PL_STR,
    // PL_LIST,
    PL_FUNCPTR,
    PL_CLOS,
};

typedef enum PLType PLType;

struct PLV; // Forward declaration

// Function pointer type
typedef struct PLV (*PLFuncptr)(struct PLV *args);
// Closure pointer type
typedef struct PLV (*PLClosptr)(struct PLV *freevars, struct PLV *args);

union PLVal {
    char b;
    i64 n;
    f64 x;
    char *s;
    PLFuncptr funcptr; // Changed to use the function pointer type
    struct {
        PLClosptr closptr;
        struct PLV *freevars;
    } clos;
};
typedef union PLVal PLVal;

struct PLV {
    PLType type;
    PLVal val;
};
typedef struct PLV PLV;

// Function prototypes to create a new PLV
PLV __new_NIL();
PLV __new_BOOL(char b);
PLV __new_INT(i64 n);
PLV __new_FLOAT(f64 x);
PLV __new_STR(const char *s);
PLV __new_LIST(PLV *elements, int len);
PLV __new_FUNCPTR(PLFuncptr funcptr);
PLV __new_CLOS(PLClosptr closptr, PLV *freevars);

// Function prototype to delete a PLV
void __delete_PLV(PLV *v);

// Function prototype for printing a PLV
void __PLV_print(PLV *v);

// Function prototype for funcall
PLV __PL_funcall(PLV *args);

// Built-in function prototypes
PLV global_func_add(PLV *args);
PLV global_func_sub(PLV *args);
PLV global_func_mul(PLV *args);
PLV global_func_div(PLV *args);
PLV global_func_eq(PLV *args);
PLV global_func_lt(PLV *args);
PLV global_func_leq(PLV *args);
PLV global_func_gt(PLV *args);
PLV global_func_geq(PLV *args);

// Implementation of PLV creattion functions
PLV __new_NIL() {
    PLV v;
    v.type = PL_NIL;
    return v;
}

PLV __new_BOOL(char b) {
    PLV v;
    v.type = PL_BOOL;
    v.val.b = b;
    return v;
}

PLV __new_INT(i64 n) {
    PLV v;
    v.type = PL_INT;
    v.val.n = n;
    return v;
}

PLV __new_FLOAT(f64 x) {
    PLV v;
    v.type = PL_FLOAT;
    v.val.x = x;
    return v;
}

PLV __new_STR(const char *s) {
    PLV v;
    v.type = PL_STR;
    v.val.s = strdup(s); // Duplicate the string
    if (v.val.s == NULL) {
        fprintf(stderr, "Error: Memory allocation failed for string\n");
        exit(1);
    }
    return v;
}

PLV __new_FUNCPTR(PLFuncptr funcptr) {
    PLV v;
    v.type = PL_FUNCPTR;
    v.val.funcptr = funcptr;
    return v;
}

PLV __new_CLOS(PLClosptr closptr, PLV *freevars) {
    PLV v;
    v.type = PL_CLOS;
    v.val.clos.closptr = closptr;
    v.val.clos.freevars = freevars; // Assume freevars is already allocated
    return v;
}

// Implementation of PLV deletion function
void __delete_PLV(PLV *v) {
    if (v->type == PL_STR) {
        free(v->val.s);
    } else if (v->type == PL_CLOS) {
        // Assume freevars is already allocated and needs to be freed
        free(v->val.clos.freevars);
    }
    // No need to free function pointers or other types
}

// Implementation of printing function
void __PLV_print(PLV *v) {
    switch (v->type) {
    case PL_NIL:
        printf("nil");
        break;
    case PL_BOOL:
        printf(v->val.b ? "true" : "false");
        break;
    case PL_INT:
        printf("%lld", v->val.n);
        break;
    case PL_FLOAT:
        printf("%lf", v->val.x);
        break;
    case PL_STR:
        printf("\"%s\"", v->val.s);
        break;
    case PL_FUNCPTR:
        printf("<function at %p>", (void *)v->val.funcptr);
        break;
    case PL_CLOS:
        printf("<closure %p>", v->val.clos.closptr);
        break;
    }
}

// Implementation of funcall
PLV __PL_funcall(PLV *args) {
    if (args[0].type != PL_FUNCPTR && args[0].type != PL_CLOS) {
        fprintf(stderr,
                "Error: First argument must be a function or closure\n");
        exit(1);
    }

    PLV result;

    if (args[0].type == PL_FUNCPTR) {
        result = args[0].val.funcptr(args + 1);
    } else {
        result = args[0].val.clos.closptr(args[0].val.clos.freevars, args + 1);
    }

    return result;
}

// Implementation of Built-in functions
PLV global_func_add(PLV *args) {
    PLV result;

    if (args[0].type == PL_INT && args[1].type == PL_INT) {
        result.type = PL_INT;
        result.val.n = args[0].val.n + args[1].val.n;
    } else if (args[0].type == PL_FLOAT && args[1].type == PL_FLOAT) {
        result.type = PL_FLOAT;
        result.val.x = args[0].val.x + args[1].val.x;
    } else {
        fprintf(stderr, "Error: Type error in addition\n");
        exit(1);
    }

    return result;
}

PLV global_func_sub(PLV *args) {
    PLV result;

    if (args[0].type == PL_INT && args[1].type == PL_INT) {
        result.type = PL_INT;
        result.val.n = args[0].val.n - args[1].val.n;
    } else if (args[0].type == PL_FLOAT && args[1].type == PL_FLOAT) {
        result.type = PL_FLOAT;
        result.val.x = args[0].val.x - args[1].val.x;
    } else {
        fprintf(stderr, "Error: Type error in subtraction\n");
        exit(1);
    }

    return result;
}

PLV global_func_mul(PLV *args) {
    PLV result;

    if (args[0].type == PL_INT && args[1].type == PL_INT) {
        result.type = PL_INT;
        result.val.n = args[0].val.n * args[1].val.n;
    } else if (args[0].type == PL_FLOAT && args[1].type == PL_FLOAT) {
        result.type = PL_FLOAT;
        result.val.x = args[0].val.x * args[1].val.x;
    } else {
        fprintf(stderr, "Error: Type error in multiplication\n");
        exit(1);
    }

    return result;
}

PLV global_func_div(PLV *args) {
    PLV result;

    if (args[0].type == PL_INT && args[1].type == PL_INT) {
        if (args[1].val.n == 0) {
            fprintf(stderr, "Error: Division by zero\n");
            exit(1);
        }
        result.type = PL_INT;
        result.val.n = args[0].val.n / args[1].val.n;
    } else if (args[0].type == PL_FLOAT && args[1].type == PL_FLOAT) {
        if (args[1].val.x == 0.0) {
            fprintf(stderr, "Error: Division by zero\n");
            exit(1);
        }
        result.type = PL_FLOAT;
        result.val.x = args[0].val.x / args[1].val.x;
    } else {
        fprintf(stderr, "Error: Type error in division\n");
        exit(1);
    }

    return result;
}

PLV global_func_eq(PLV *args) {
    PLV result;
    result.type = PL_BOOL;

    if (args[0].type != args[1].type) {
        result.val.b = 0; // false
        return result;
    }

    switch (args[0].type) {
    case PL_NIL:
        result.val.b = 1; // Both nil
        break;
    case PL_BOOL:
        result.val.b = (args[0].val.b == args[1].val.b);
        break;
    case PL_INT:
        result.val.b = (args[0].val.n == args[1].val.n);
        break;
    case PL_FLOAT:
        result.val.b = (args[0].val.x == args[1].val.x);
        break;
    case PL_STR:
        result.val.b = (strcmp(args[0].val.s, args[1].val.s) == 0);
        break;
    default:
        result.val.b = 0; // Other types not comparable
        break;
    }

    return result;
}

PLV global_func_lt(PLV *args) {
    PLV result;
    result.type = PL_BOOL;

    if (args[0].type == PL_INT && args[1].type == PL_INT) {
        result.val.b = (args[0].val.n < args[1].val.n);
    } else if (args[0].type == PL_FLOAT && args[1].type == PL_FLOAT) {
        result.val.b = (args[0].val.x < args[1].val.x);
    } else {
        fprintf(stderr, "Error: Type error in less than comparison\n");
        exit(1);
    }

    return result;
}

PLV global_func_leq(PLV *args) {
    PLV result;
    result.type = PL_BOOL;

    if (args[0].type == PL_INT && args[1].type == PL_INT) {
        result.val.b = (args[0].val.n <= args[1].val.n);
    } else if (args[0].type == PL_FLOAT && args[1].type == PL_FLOAT) {
        result.val.b = (args[0].val.x <= args[1].val.x);
    } else {
        fprintf(stderr, "Error: Type error in less than or equal comparison\n");
        exit(1);
    }

    return result;
}

PLV global_func_gt(PLV *args) {
    PLV result;
    result.type = PL_BOOL;

    if (args[0].type == PL_INT && args[1].type == PL_INT) {
        result.val.b = (args[0].val.n > args[1].val.n);
    } else if (args[0].type == PL_FLOAT && args[1].type == PL_FLOAT) {
        result.val.b = (args[0].val.x > args[1].val.x);
    } else {
        fprintf(stderr, "Error: Type error in greater than comparison\n");
        exit(1);
    }

    return result;
}

PLV global_func_geq(PLV *args) {
    PLV result;
    result.type = PL_BOOL;

    if (args[0].type == PL_INT && args[1].type == PL_INT) {
        result.val.b = (args[0].val.n >= args[1].val.n);
    } else if (args[0].type == PL_FLOAT && args[1].type == PL_FLOAT) {
        result.val.b = (args[0].val.x >= args[1].val.x);
    } else {
        fprintf(stderr,
                "Error: Type error in greater than or equal comparison\n");
        exit(1);
    }

    return result;
}

void print_PLV(PLV *v) {
    switch (v->type) {
    case PL_NIL:
        printf("nil");
        break;
    case PL_BOOL:
        printf(v->val.b ? "true" : "false");
        break;
    case PL_INT:
        printf("%lld", v->val.n);
        break;
    case PL_FLOAT:
        printf("%lf", v->val.x);
        break;
    case PL_STR:
        printf("\"%s\"", v->val.s);
        break;
    case PL_FUNCPTR:
        printf("<function at %p>", (void *)v->val.funcptr);
        break;
    case PL_CLOS:
        printf("<closure %p>", v->val.clos.closptr);
        break;
    }
}
