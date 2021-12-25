import sys

bin_ops = {
    "add": "+",
    "mul": "*",
    "mod": "%",
    "div": "/",
}

c_lines = []
inputs = 0
for line in sys.stdin:
    line = line.strip()
    if line:
        args = line.split(" ")
        if args[0] == "inp":
            c_lines.append("  %s = input%d;" % (args[1], inputs))
            inputs += 1;
        elif args[0] in bin_ops:
            c_lines.append("  %s %s= %s;" % (args[1], bin_ops[args[0]], args[2]))
        elif args[0] == "eql":
            c_lines.append("  %s = (%s == %s);" % (args[1], args[1], args[2]))
        else:
            assert False;
c_lines = [
    "#include <stdint.h>",
    "",
    "int run_program(const int64_t* inputs) {",
    "  int64_t w, x, y, z;",
    "  w = x = y = z = 0;",
    *[
        "  const int64_t input%d = inputs[%d];" % (i, i)
        for i in range(inputs)
    ]
] + c_lines + [
    "  return z == 0;",
    "}"
]
print("\n".join(c_lines))
