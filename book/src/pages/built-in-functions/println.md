# `println` built-in function
`println` is a built-in function to print a value to the stdout. It takes one argument and prints it to the stdout. It adds a newline character at the end of the output.

## What can it print?
`println` can print any [data type](../types/intro.md)

## Differences between `print` and `println`
`print` and `println` are very similar. The only difference is that `println` adds a newline character at the end of the output.

### Examples
```ocypode
~main<argc><argv>{<
    println<"Hello, World!">;
>}
```
Output:
```
Hello, World!
```

```ocypode
~main<argc><argv>{<
    println<[1,2,3,[4,5,6]]>;
>}
```
Output:
```
[1, 2, 3, [4, 5, 6]]
```