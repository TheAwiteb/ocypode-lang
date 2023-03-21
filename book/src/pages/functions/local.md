# Local Functions
Local functions are functions that are defined inside another function. Local functions are defined as global functions [see here](./global.md), but they are defined inside another function. Local functions are useful when you want to define a function that is only used inside another function.

## Ownership
Local functions are owned by the function that they are defined in, and they are destroyed when the function ends.

## Syntax
Same as global functions [see here](./global.md). The only difference is that local functions need to add a semi-colon `;` after the function body.

## Examples
```ocypode
~main<argc><argv>{<
    ~foo<name>{<
        println<format<"Hello {}"><name>>;
    >};
    foo<"Ahmed">;
>}
```
Output:
```text
Hello Ahmed
```