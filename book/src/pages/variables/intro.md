# Variables
A variable is a name that is used to store a value. The value can't be changed later, but you can change the value of the variable after is owned by another variable/parameter. see [ownership](#ownership).

## Ownership
A variable is owned by the function that it is defined in, and it is destroyed when the function ends. After using a variable, you can't use it again because the value has been moved to another variable/parameter.

## Syntax
A variable is defined by its name and its value. The name of the variable can contain letters, numbers, and underscores, and it can't start with a number, also the variable name is case-sensitive. And the variable name must be [snake case].

### Examples
```ocypode
~main<argc><argv>{<
    name = "Ahmed";
    age = 20;
    is_student = true;
    height = 1.75;
    grades = [100, 90, 80, 70, 60];
    println<
        format<"{} is {} years old, and he is {} a student, and his height is {}. His grades are {}">
        <name><age><is_student><height><grades>
    >;
>}
```
Output:
```text
Ahmed is 20 years old, and he is true a student, and his height is 1.75. His grades are [100, 90, 80, 70, 60]
```