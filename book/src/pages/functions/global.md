# Global Functions
Global functions are functions that are defined in the global scope. Global functions are defined with the `~` keyword and are useful when you want to use them in multiple places in your program.

## Ownership
Global functions are owned by the global scope, and they are destroyed when the program ends.

## Syntax
Global functions have a name, parameters, and body. before the name you need to add the `~` keyword, and after the name is optional to add the parameters, it will locks like this `<param1><param2><param3>`, and after the parameters are the body of the function, and the body is the statements that will be executed when the function is called.

## Function name
Each function has a name, and the is unique which means you can't have two functions with the same name. The function name can contain letters, numbers, and underscores, and it can't start with a number, also the function name is case-sensitive. And the function name must be [snake case].

### Examples
```ocypode
~foo{<
    /*
    function body
    */
>}

~foo<name>{<
    /*
    Anouther function body
    */
>}

~main<argc><argv>{< >}
```
<details>
  <summary>The Error</summary>

```text
Error(runtime::already_declared)

  💥 Identifier already declared
    ╭─[test.oy:1:1]
  1 │ ~foo{<
    ·  ─┬─
    ·   ╰── Identifier `foo` already declared here
  2 │     /*
  3 │     function body
  4 │     */
  5 │ >}
  6 │ 
  7 │ ~foo<name>{<
    ·  ─┬─
    ·   ╰── And you tried to declare it again here
  8 │     /*
  9 │     Anouther function body
 10 │     */
    ╰────
  help: Try renaming `foo` or removing the previous declaration.
```
</details>

## Parameters
Each function can have parameters, and the parameters are optional. The parameters are the variables that you can use inside the function body. The parameters are defined after the function name, and before the body. The parameters are defined with the `<param1><param2><param3>` syntax, each parameter is inside a pair of `<` and `>`.

## Body
The body of the function is the statements that will be executed when the function is called. The body is defined after the parameters, starts with the `{<` and ends with the `>}`.

## Return values
Global functions can return values, you can return a value using the `return` keyword, and the value will be returned to the caller.

### Examples
```ocypode
~foo{<
    return 1;
>}
```

## Call functions
To call a function, you need to add `<>` after the function name if the function takes no arguments, or you need to add `<arg1><arg2><arg3>` after the function name if the function takes arguments.

### Examples
```ocypode
~foo<name><age>{<
    println<format<"Hello {} you are {} years old"><name><age>>;
>}

~main<argc><argv>{<
    foo<"Ahmed"><20>;
>}
```
Output:
```text
Hello Ahmed you are 20 years old
```

[snake case]: https://en.wikipedia.org/wiki/Snake_case
