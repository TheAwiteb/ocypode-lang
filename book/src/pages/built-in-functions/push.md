# `push` built-in function
`push` is a built-in function to add an element to the end of an array. It takes two arguments, the array and the element to add. It returns the array with the element added to the end.

## Examples
```ocypode
~main<argc><argv>{<
    array = [1,2,3];
    array = push<array, 4>;
    println<array>;
>}
```
Output:
```
[1, 2, 3, 4]
```

```ocypode
~main<argc><argv>{<
    array = [1,2,3];
    array = push<array, 4>;
    array = push<array, 5>;
    array = push<array, 6>;
    println<array>;
>}
```
Output:
```
[1, 2, 3, 4, 5, 6]
```
