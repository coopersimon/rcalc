# Rust calc
Calculator used for simple maths and base conversion.

Like my previous calc project, only written in Rust. I use it as a tool for my other projects / at work.

This is cleaner, maybe faster, and offers new functionality.

### Usage
Right now, single expressions are possible in a one-shot command:
```
rcalc "5 / 3"
rcalc "0b4 * 0x2d"
```

Also, the interpreter can be entered by simply running `rcalc`, then multiple expressions can be evaluated.

In interpreted mode, variables can be used to save values:
```
v = 5 / 3
v * 20
```

Valid variable names are a single character, followed by 0 or more digits. `v10` is a valid variable, `var` is not.

The special variable `ans` can be used to get the result of the previous expression.

Running an expression with just `q` or `quit` exits the program. Therefore, `q` is not recommended to be used as a variable name.

### Types
Integer and floating-point (both 64-bit). There are no explicit conversions between types (yet), however integer precision is kept whenever possible. If a fraction is formed from a division of two integers, then the result is converted to a floating-point.

### Flags
`--help` for help.

Set `-b` for binary output, `-o` for octal and `-h` for hexadecimal. These can be combined with `pN` where `N` is the number of places of precision.

`-pN` can also be used by itself with decimal (floating-point) numbers to set the number of decimal places.

### TODO
- Multi-expression statements, separated by `;`.
- Better parse error messages.
- Setting of output type.
- Explicit rounding/casting.