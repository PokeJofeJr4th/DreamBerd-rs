# Dreamberd.rs

Rust-based interpreter for the Dreamberd language.

The full specification for Dreamberd is available at https://github.com/TodePond/DreamBerd. This file only contains the segments of the specification that are implemented in dreamberd-rs.

## Statements

Every statement ends with an exclamation mark! If you're feeling extra, you can even use multiple!!!

If you're unsure, that's okay too! You can also use question marks? This will print debug information to the console? The more question marks, the more detailed the information?

## Declarations

There are four types of declarations. Constant constants can't be changed at all.

```
const const name = "Ava"!
```

Constant variables can be edited but not reassigned.

```
const var age = 1!
// implementation under construction
```

Variable constants can be reassigned but not edited.

```
var const id = "main"!
// implementation under construction
```

Variable variables can be reassigned and edited.

```
var var count = 0!
// implementation under construction
```

## Naming

Both variables and constants can be named with any Unicode character or string that isn't interpreted as another feature.

```
const const firstAlphabetLetter = 'A'!
var const 👍 = true!
var var 1️⃣ = 1!
```

This includes numbers, and other language constructs.

```
const const 5 = 4!
(2 + 2 === 5)? //true
```

## Booleans

Booleans can be `true`, `false`, or `maybe`.

## Strings

Strings can be declared with zero quotes, single quotes, or double quotes.

```
const const name = L!
const const name = 'Lu'!
const const name = "Luke"!
```

## Arithmetic

DreamBerd has significant whitespace. Use spacing to specify the order of arithmetic operations.

```
(1 + 2*3)? //7
(1+2 * 3)? //9
```

You can add strings together and multiply them by floats.

```
("he" + "l"*2 "o" + " " + "world")? // "hello world"
("johnny"*1.5)? // "johnnyjoh"
```

### Dividing by Zero

Dividing by zero returns undefined.

```
(3 / 0)? // undefined
```

## Equality

JavaScript lets you do different levels of comparison. `==` for loose comparison, and `===` for a more precise check. DreamBerd takes this to another level.

You can use `==` to do a loose check.

```
(3.14 == "3.14")? //true
```

You can use `===` to do a more precise check.

```
(3.14 === "3.14")! //false
```

You can use `====` to be EVEN MORE precise!

```
const const pi = 3.14!
(pi ==== pi)? //true
(3.14 ==== 3.14)? //false (this differs from the official DreamBerd specification)
(3.14 ==== pi)? //false
```

Finally, if you want to be much less precise, you can use `=`.

```
(3 = 3.14)? //true
```

## Functions

To declare a function, you can use any letters from the word function (as long as they're in order):

```
function(add, (a, b),  (a + b))!
func(multiply, (a, b), (a * b))!
fun(subtract, (a, b), (a - b))!
fn(divide, (a, b), (a / b))!
functi(power, (a, b), (a ** b))!
union(inverse, (a), (1/a))!
```

## Delete

To avoid confusion, the delete statement only works with identifiers like variables, numbers, strings, and booleans.

```
delete(3)!
(2+1 == 3) // false
```

DreamBerd is a multi-paradigm programming language, which means that you can delete the keywords and paradigms you don't like.

```
delete(maybe)!!!
const const is_raining = maybe!
is_raining? // undefined
```

When perfection is achieved and there is nothing left to delete, you can do this:

```
delete(delete)!
```
