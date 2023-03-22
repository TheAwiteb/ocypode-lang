# `input` built-in function
`input` is a built-in function to get input from the user. It takes one argument, the prompt to display to the user. It returns the input from the user as a string.

### Errors
`input` will throw an error if its cannot get a input from stdin.

## Examples
```ocypode
~main<argc><argv>{<
    user_name = input<"Enter your name: ">;
    user_age = input<"Enter your age: ">;
    println<
        format<"Hello {}! you are {} years old"><user_name><user_age>
    >;
>}
```
Output:
```
Enter your name: John
Enter your age: 20
Hello John! you are 20 years old
```
