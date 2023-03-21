# Strings
A string is a data type that can be used to store text, and it is used to represent a sequence of characters.

## Syntax
A string is defined by a sequence of characters surrounded by double quotes `""`, and the characters can be any character except the double quote `"`, and the backslash `\`.

### Escape Sequences
The backslash `\` can be used to escape a character, and it can be used to escape the following characters:
- `"`: Escape the double quote.
- `\`: Escape the backslash.
- `n`: Escape the new line character.
- `t`: Escape the tab character.
- `r`: Escape the carriage return character.

### Examples
```ocypode
~main<argc><argv>{<
    name = "Ahmed";
    address = "Cairo, Egypt";
    println<format<"His name is {}\nHe resides in in \"{}\""><name><address>>;
>}
```
Output:
```text
His name is Ahmed
He resides in "Cairo, Egypt"
```

## Muilti-line Strings
A string can be defined in multiple lines, just like this:
```ocypode
~main<argc><argv>{<
    name = "Ahmed";
    address = "Cairo, Egypt";
    println<format<"
        His name is {}
        He resides in in \"{}\"
    "><name><address>>;
>}
```
Output:
```text
His name is Ahmed
He resides in "Cairo, Egypt"
```

Or you can use `\n` to define a new line, like the example above.