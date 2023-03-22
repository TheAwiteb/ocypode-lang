# `format` built-in function
`format` is a built-in function to format a string. It takes first argument as format string and the rest of the arguments as values to format. It returns a formatted string.

## What can it format?
`format` can format any [data type](../types/intro.md)

## Format string
The format string is a string that contains a placeholders. and the placeholders are replaced with the values passed as arguments.

### Placeholders
Placeholders are curly brackets like `{}` and this curly bracket will be replaced with the values passed as arguments. Also the placeholders can contain a number to specify the index of the argument to use. The index starts from 0.

#### Examples
In this example, the first placeholder `{}` will be replaced with the first argument passed to the `format` function, and the second placeholder `{0}` contains a number `0` which is the index of the first argument passed to the `format` function, so the second placeholder will be replaced with the first argument passed to the `format` function.
```ocypode
~main<argc><argv>{<
    formatted_string = format<"{} {0}"><"Hello, World!">;
    println<formatted_string>;
>}
```
Output:
```
Hello, World! Hello, World!
```

In this example, the first placeholder `{}` will be replaced with the first argument passed to the `format` function, and the second placeholder `{}` will be replaced with the second argument passed to the `format` function.
```ocypode
~main<argc><argv>{<
    formatted_string = format<"Hi {} you are from {}"><"Jhon"><"USA">;
    println<formatted_string>;
>}
```
Output:
```
Hi Jhon you are from USA
```
