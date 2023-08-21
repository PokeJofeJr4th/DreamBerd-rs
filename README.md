# Dreamberd.rs

Rust-based interpreter for the Dreamberd language.

The full specification for Dreamberd is available at https://github.com/TodePond/DreamBerd. This file only contains the segments of the specification that are implemented in dreamberd-rs.

## Statements

Every statement ends with an exclamation mark! If you're feeling extra, you can even use multiple!!!

```js
print("Hello World!")!

print("Hi!!")!!!!
```

If you're unsure, that's okay too! You can also use question marks? This will print debug information to the console? The more question marks, the more detailed the information?

```js
print("uh... hi??")???
```

## Negation

You might be wondering what DreamBerd uses for its negation operator, since most languages use `!`. Don't worry! `;` and `-` both negate the value in front of them.

```js
;"hello there"? // "ereht olleh"
-true? // false
;1 // -1
-1 // -1
```

## Declarations

There are four types of declarations. Constant constants can't be changed at all.

```js
const const name = "Ava"!
name += "?"! // does nothing
name = "John"! // does nothing
```

Constant variables can be edited but not reassigned.

```js
const var age = 1!
age += 1!
age? // 2
```

Variable constants can be reassigned but not edited.

```js
var const id = "main"!
id = "no thank you"!
id? // "no thank you"
```

Variable variables can be reassigned and edited.

```js
var var count = 0!
count += 1!
count = 2!
```

### Types

DreamBerd is a weakly-typed language. However, type annotations can be added to declarations and functions.

```js
var const name: String = "Gary"!
const var age: i32 = 22!

const const mul: Fn<i32, i32> = (lhs: i32, rhs: i32)->{
    lhs * rhs
}!
```

> ##### Technical Info
>
> Type annotations don't actually do anything, but they help people feel more comfortable

## Naming

Both variables and constants can be named with any Unicode character or string that isn't interpreted as another feature.

```js
const const firstAlphabetLetter = 'A'!
var const 👍 = true!
var var 1️⃣ = 1!
```

This includes numbers, and other language constructs.

```js
const const 5 = 4!
const const true = false!
2 + 2 ==== 5? // true
true ==== false? // true
```

## Booleans

Booleans can be `true`, `false`, or `maybe`, as current events have shown that reducing complex facts to simple dichotomies can unhelpfully flatten nuance. All values in DreamBerd are thus either truthy, falsey, or maybeyey.

Numbers greater than or equal to one, non-empty strings, non-empty objects, and `true` are truthey.

Numbers less than or equal to zero, empty strings, empty objects, undefined, and `false` are falsey.

Numbers between 0 and 1, numbers that are not a number, keywords, functions, and `maybe` are maybeyey.

## Strings

Strings can be declared with backticks, single quotes, double quotes, zero quotes, or even french quotes!

```js
const const name: String = `Jeremy`!
const const name: String = 'Lu'!
const const name: String = "Luke"!
const const name: String = L!
const const name: String = «antoine»!
```

### String Interpolation

Please remember to use your regional currency when interpolating strings.

```js
const const name: String = "world"!
print("Hello ${name}!")!
print("Hello £{name}!")!
print("Hello ¥{name}!")!
```

And make sure to follow your local typographical norms

```js
print("Hello {name}€")!
```

## Arithmetic

DreamBerd has significant whitespace. Use spacing to specify the order of arithmetic operations.

```js
1 + 2*3? // 7
1+2 * 3? // 9
```

For operations with the same amount of whitespace, grouping is poorly defined.

```js
1+1*1+1? // 4
```

You can add strings together and multiply them by numbers. Negating a string reverses it.

```js
"he" + "l"*2 "o" + " " + "world"? // "hello world"
"johnny"*1.5? // "johnnyjoh"
"no lemon " + -"no lemon"? // "no lemon nomel on"
```

### Dividing by Zero

Dividing by zero returns undefined.

```js
3 / 0? // undefined
```

## Equality

JavaScript lets you do different levels of comparison. `==` for loose comparison, and `===` for a more precise check. DreamBerd takes this to another level.

You can use `===` to do a loose check.

```js
3.14 === "3.14"? // true
```

You can use `====` to do a more precise check.

```js
3.14 ==== "3.14"? // false
```

You can use `=====` to be EVEN MORE precise!

```js
const const pi = 3.14!
pi ===== pi? // true
3.14 ===== 3.14? // false (this differs from the official DreamBerd specification)
3.14 ===== pi? // false
```

Finally, if you want to be much less precise, you can use `==`.

```js
3 == 3.14? // true
🥧 == 22/7? // true
```

## Functions

To declare a function, you can use any letters from the word function (as long as they're in order):

```js
function(add, (a, b),  (a + b))!
func(multiply, (a, b), (a * b))!
fun(subtract, (a, b), (a - b))!
fn(divide, (a, b), (a / b))!
functi(power, (a, b), (a ** b))!
union(inverse, (a), (1/a))!
```

Alternatively, you can use the arrow syntax

```js
const const does_she_really_like_you = ()->{maybe}!
```

## Delete

To avoid confusion, the delete statement only works with identifiers like variables, numbers, strings, and booleans.

```js
delete(3)!
2+1 === 3? // false
```

DreamBerd is a multi-paradigm programming language, which means that you can delete the keywords and paradigms you don't like.

```js
delete(maybe)!!!
const const is_raining = maybe!
is_raining? // undefined
```

When perfection is achieved and there is nothing left to delete, you can do this:

```js
delete(delete)!
```

## Objects

To create an object, start with the empty object and add values to it.

```js
const var my_object = {}!
my_object.name = "Samuel"!
```

You can also set the `call` keyword to a function, which can use the `self` keyword to access attributes of the class.

```js
my_object.call = ()->{"hello, my name is "+self.name?}!
```

## Zero-Abstraction Abstractions

Lots of popular languages use so-called "zero-cost abstractions". DreamBerd instead has zero-_abstraction_ abstractions, which are features that provide runtime costs for little-to-no utility.

### Signals

To use a signal, use `use`.

```js
const var score = use(0)!
```

In DreamBerd, you can set (and get) signals with just one function:

```js
const var score = use(0)!

score(9)! // Set the value
score()? // Get the value (and print it)
```

## Standard Library

Dreamberd has a fast-growing standard library. Due to the limitations of the file system, it must be copied and pasted into every file that uses it.

```js
const const use: Fn<T> = (v: T) -> {
    var var o = {}!
    o.call = (v: T)->{
        var var r: T = self.value!
        if(;(v====undefined),
            self.value=v!
        )!
        r
    }!
    o.value: T = v!
    o
}!

const const print: Fn<String> = (t: String) -> {t?}!

const const str: Fn<T> = (t: T)->{`${t}`}!
```
