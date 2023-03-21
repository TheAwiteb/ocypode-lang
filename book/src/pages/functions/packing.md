# Packing Parameters
A pack parameter is a parameter that can be used to pack the rest of the parameters into a list. The packing parameter must be the last, and it has a star `*` before the parameter name.

## Examples
```ocypode
~foo<name><*childs>{<
    println<format<"{} has {} childs"><name><len<childs>>>;
>}

~main<argc><argv>{<
    foo<"Ahmad"><"Mohammed"><"Ali"><"Khalid">;
>}
```
Output:
```text
Ahmad has 3 childs
```

A function with a packing parameter can be called with any number of arguments, the packing parameter will pack all the arguments into a list.

## With anonymous functions
Packing parameters can be used with anonymous functions, the same as other functions.

```ocypode
~main<argc><argv>{<
    <name><*childs>{<println<format<"{} has {} childs"><name><len<childs>>>;>}
        <"Ahmad"><"Mohammed"><"Ali"><"Khalid">;
>}
```
Output:
```text
Ahmad has 3 childs
```
