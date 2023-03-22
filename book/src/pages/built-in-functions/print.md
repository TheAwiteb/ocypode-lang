# `print` built-in function
`print` is a built-in function to print a value to the stdout. It takes one argument and prints it to the stdout. It does not add a newline character at the end of the output.

## What can it print?
`print` can print any [data type](../types/intro.md)

### Examples
```ocypode
~main<argc><argv>{<
    print<"Hello, World!">;
>}
```
Output:
```
Hello, World!⏎
```

```ocypode
~main<argc><argv>{<
    print<1>;
>}
```
Output:
```
1⏎
```