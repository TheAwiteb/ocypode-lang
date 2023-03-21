# Anonymous functions
Anonymous functions are functions that don't have a name, and they are defined inside another function. Anonymous functions are useful when you want to pass a function as an argument to another function, or when you want to return a function from another function.

## Syntax
Anonymous functions have parameters and a body. The parameters are the same as [local functions], and the body is the same as [local functions]. The only difference is that the function name is missing.

```ocypode
~main<argc><argv>{<
    <param1><param2><param3>{</* The block */>};
>}
```

## Take no arguments
There is a difference between [local functions] and anonymous functions if you want to make it take no arguments, you need to add `<>` before the block.

## Return values
Anonymous functions can return values, just like [local functions], you can return a value by using the `return` keyword.

```ocypode
~main<argc><argv>{<
    <>{<return 1;>};
>}
```

## Call anonymous functions
To call an anonymous function, you just need to call it like a [local function]. Add `<>` to call it without arguments, or add `<arg1><arg2><arg3>` to call it with arguments.

```ocypode
~main<argc><argv>{<
    hello = <>{<return "Hello world!";>}<>;
    println<hello>;
>}
```
Output:
```text
Hello world!
```

## Assign anonymous functions to variables
You can assign anonymous functions to variables, like the example below.

```ocypode
~main<argc><argv>{<
    say_hello = <name>{<println<format<"Hello {}"><name>;>;>};
    say_hello<"Ahmed">;
>}
```
Output:
```text
Hello Ahmed
```