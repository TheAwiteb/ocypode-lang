# `len` built-in function
`len` is a built-in function to get the length of a object. It takes one argument and returns the length of the object.

## What can it get the length of?
`len` can get the length of [strings] and [arrays].

## Errors
`len` will throw an error if its argument is not a string or array.

## Examples
```ocypode
~main<argc><argv>{<
    len_of_string = len<"Hello, World!">;
    println<len_of_string>;
>}
```
Output:
```
13
```

```ocypode
~main<argc><argv>{<
    len_of_array = len<[1,2,3,[4,5,6]]>;
    println<len_of_array>;
>}
```
Output:
```
4
```

[strings]: ../types/string.md
[arrays]: ../types/array.md