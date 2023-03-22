# `pop` built-in function
`pop` is a built-in function to remove the last element from an array. It takes one argument and returns the array without the last element.

## Examples
```ocypode
~main<argc><argv>{<
    array = [1,2,3];
    array = pop<array>;
    println<array>;
>}
```
Output:
```
[1, 2]
```

```ocypode
~main<argc><argv>{<
    array = [1,2,3];
    array = pop<array>;
    array = pop<array>;
    array = pop<array>;
    println<array>;
>}
```
Output:
```
[]
```

```ocypode
~main<argc><argv>{<
    array = [1,2,3];
    array = pop<array>;
    array = pop<array>;
    array = pop<array>;
    array = pop<array>;
    println<array>;
>}
```
Output:
```
[]
```
