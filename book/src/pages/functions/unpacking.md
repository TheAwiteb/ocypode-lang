# Unpacking Arguments
A unpack argument is an argument that can be used to unpack a list into multiple arguments. The unpack argument must be an array, and it has a 3-dots `...` before the argument name.

## Examples
```ocypode
~foo<name><*childs>{<
    println<format<"{} has {} childs"><name><len<childs>>>;
>}

~main<argc><argv>{<
    childs = ["Mohammed", "Ali", "Khalid"];
    // The childs list will be unpacked into 3 arguments
    foo<"Ahmad"><...childs>;
>}
```
Output:
```
Ahmad has 3 childs
```

## With anonymous functions
Unpacking arguments can be used with anonymous functions, the same as other functions.

```ocypode
~main<argc><argv>{<
    <name><*childs>{<println<format<"{} has {} childs"><name><len<childs>>>;>}
        <"Ahmad"><...["Mohammed", "Ali", "Khalid"]>;
>}
```
Output:
```
Ahmad has 3 childs
```
