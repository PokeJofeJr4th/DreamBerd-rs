# DreamBerd rs

Rust-based interpreter for the DreamBerd language.

The full specification for DreamBerd is available at https://github.com/TodePond/DreamBerd. This file only contains the segments of the specification that are implemented in DreamBerd-rs, along with some of my own additions.

## Rapidly-Evaporating Program Logic (REPL)

DreamBerd provides a convenient mode to execute code within the terminal. The preservation of state allows you to play with features of DreamBerd without writing your code into a file. You can also include code from a file by providing the filename as an argument, which will run it and allow you to play around in what remains.

## Statements

Every statement ends with an exclamation mark! If you're feeling extra, you can even use multiple!!!

```c
print("Hello World!")!

print("Hi!!")!!!!
```

If you're unsure, that's okay too! You can also use question marks? This will print debug information to the console? The more question marks, the more detailed the information???

```c
print("uh... hi??")???
```

## Negation

You might be wondering what DreamBerd uses for its negation operator, since most languages use `!`. Don't worry! `;` and `-` both negate the value in front of them.

```c
;"hello there"? // "ereht olleh"
-true? // false
;1 // -1
-1 // -1
```

## Declarations

There are four types of declarations. Constant constants can't be changed at all.

```c
const const name = "Ava"!
name += "?"! // does nothing
name = "John"! // does nothing
```

Constant variables can be edited but not reassigned.

```c
const var age = 1!
age += 1!
age? // 2
```

Variable constants can be reassigned but not edited.

```c
var const id = "main"!
id = "no thank you"!
id? // "no thank you"
```

Variable variables can be reassigned and edited.

```c
var var count = 0!
count += 1!
count = 2!
```

### Types

DreamBerd is a weakly-typed language. However, type annotations can be added to declarations and functions.

```c
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

```c
const const firstAlphabetLetter = 'A'!
var const 👍 = true!
var var 1️⃣ = 1!
```

This includes numbers, and other language constructs.

```c
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

## Control Flow

DreamBerd has a simple `if` statement:

```c
if(true ===== false,
    print("true is false"),
    print("true is not false"),
    print("true might be false")
)!
```

Notice, the if statement includes a section for if the discriminating variable is `maybe`.

This is actually a function, so it can be assigned:

```c
const const the_if_statement = if!

the_if_statement(true ==== false, print("true is false"))!
```

## Strings

Strings can be declared with backticks, single quotes, double quotes, zero quotes, or even french, austrian, or german quotes!

```c
const const name: String = `Jeremy`!
const const name: String = 'Lu'!
const const name: String = "Luke"!
const const name: String = L!
const const nom: Chaîne = «Antoine»!
const const név: Húr = »Lorenz«!
const const name: Zeichenfolge = „Karl“!
```

### String Interpolation

Please remember to use your regional currency when interpolating strings.

```c
const const name: String = "world"!
print("Hello ${name}!")!
print("Hello £{name}!")!
print("Hello ¥{name}!")!
```

And make sure to follow your local typographical norms

```c
print("Hello {name}€")!
```

## Arithmetic

DreamBerd has significant whitespace. Use spacing to specify the order of arithmetic operations.

```c
1 + 2*3? // 7
1+2 * 3? // 9
```

For operations with the same amount of whitespace, grouping is poorly defined.

```c
1+1*1+1? // 4
```

You can add strings together and multiply them by numbers. Negating a string reverses it.

```c
"he" + "l"*2 "o" + " " + "world"? // "hello world"
"johnny"*1.5? // "johnnyjoh"
"no lemon " + -"no lemon"? // "no lemon nomel on"
```

### Dividing by Zero

Dividing by zero returns undefined.

```c
3 / 0? // undefined
```

## Equality

JavaScript lets you do different levels of comparison. `==` for loose comparison, and `===` for a more precise check. DreamBerd takes this to another level.

You can use `===` to do a loose check.

```c
3.14 === "3.14"? // true
```

You can use `====` to do a more precise check.

```c
3.14 ==== "3.14"? // false
```

You can use `=====` to be EVEN MORE precise!

```c
const const pi = 3.14!
pi ===== pi? // true
3.14 ===== 3.14? // false (this differs from the official DreamBerd specification)
3.14 ===== pi? // false
```

Finally, if you want to be much less precise, you can use `==`.

```c
3 == 3.14? // true
🥧 == 22/7? // true
```

## Functions

To declare a function, you can use any letters from the word function (as long as they're in order):

```c
function(add, (a, b),  (a + b))!
func(multiply, (a, b), (a * b))!
fn(subtract, (a, b), (a - b))!
non(divide, (a, b), (a / b))!
union(inverse, (a), (1/a))!
```

Alternatively, you can use the arrow syntax

```c
const const does_she_really_like_you = ()->{maybe}!
```

## Built In Functions

Did you know yoit know you could print the Dreamberd Logo!
All you have to do is
 ```c
 dreamberd_logo()! // Dreamberd
 ```

## Delete

To avoid confusion, the delete statement only works with identifiers like variables, numbers, strings, and booleans.

```c
delete(3)!
2+1 === 3? // false
```

DreamBerd is a multi-paradigm programming language, which means that you can delete the keywords and paradigms you don't like.

```c
delete(maybe)!!!
const const is_raining = maybe!
is_raining? // undefined
```

When perfection is achieved and there is nothing left to delete, you can do this:

```c
delete(delete)!
```

## Objects

To create an object, start with the empty object and add values to it.

```c
const var my_object = {}!
my_object.name = "Samuel"!
```

You can also set the `call` keyword to a function, which can use the `self` keyword to access attributes of the class.

```c
my_object.call = ()->{"hello, my name is "+self.name?}!
```

## Evaluation

DreamBerd provides a built-in function to interpret DreamBerd code at runtime. This is most useful when combined with string interpolation.

```c
const const value: i32 = 9!
const const square = x->eval("${x} * ${x}")!
square("value")? // 9
```

It's important to note that this will propagate errors from parsing or interepreting this code up to the caller.

## Zero-Abstraction Abstractions

Lots of popular languages use so-called "zero-cost abstractions". DreamBerd instead has zero-_abstraction_ abstractions, which are features that provide runtime costs for little-to-no utility.

### Signals

To use a signal, use `use`.

```c
const var score = use(0)!
```

In DreamBerd, you can set (and get) signals with just one function:

```c
const var score = use(0)!

score(9)! // Set the value
score()? // Get the value (and print it)
```

## Standard Library

> #### New for October 2023!
>
> DreamBerd has recently migrated to the [Babel](https://libraryofbabel.info/) hosting service for its standard library. The library is encoded in ASCII using base-16, where the lowercase letters a-p represent 0-15 and each character is represented by two of those letters.
>
> Currently, the standard library is stored at the page shown directly below. The library code shown in this document is a decoded version.
> [<details><summary>page 246 of volume 16 on shelf 1 of wall 4 in hexagon</summary>1n5yxx56981q9fyqxm545f0z9uw0o27k11vo4tm468gx9o66tm08hy564ra3lxtjab8pc4rxsghubqz8lyfzlq6muajp65j3jn6aiyjiw4l5b9mixytxpii0g4pa21w2mwp6txwvoji0lxrvxy7cna2picb1gzc9ap7u1jkgi6vzy5juxmcez6h6cmgcf1jiglo8u2bt3nb9hso115vw6fil31wniyokse8cipl93kjctyfaiui1y5x0z15lamt5vimgmatdzzc5zmddvq8f7hck022y4lze7ixi9xivbdmz5z7oldygeeavarmctf9reywmmz1t2eq9jz0hh93ob0nd034ei4ztz6e4wxrbn1nbwe1jozhm9xrm41lnb591uzcbib64jgpxpl8b2sk19jueogd18zn6b7jj5tn5433kmt6dazifkhm5jpowgni31r4tzimnibkfyzpli3n5exapnfsc4yjk8gspk1usetw6qcc769fw7yxh0q1c531z4tb5pvszevd4odmv8jhxq5apz8ky5xmb45jfo2sn8aku6aai4t2xsrun47a5slkftot9fifb137g4cnkit5bp4zi9pgix459nalbgbsplz8lwnqd5bssundychxgqhhte7d3rlyf45xzwaamnxny5y4qb9h0d2xfsh1c9jkyoofpte8dmfkuliu6gpgh0keetcflck3mk8rgzin16mk4wy50yax2b9ljb1s1fyrp1auvr7pc7p3czxru25kpzmcgjwfbwzw8smvmpi6ibyjcdaw8bpikpxdfbe84wmfa8k2top7vzqc5ahqt9wv5ch6lcokfo5irsabst570utor3gpgs73eu6fvcikxxq8vf2onkxj4a1xomcmzyzrtag9i5yx5rdgmmbe8firsjhkicqn7qpux00th1spvkg2bim1svtpltmdjrdd27qijl3f4uaf3twan33ndmu6j62n4emkxv8ebpk3k9eb1ggzvun2diubarli9chs7rcjegfafc0x3cczt171amecc99l29fyt7cvy33du3a29xaboapuk45dayuei5e133rn6jb5r0bzt95t78uqsxehngyjle462e3u16x32u59xzetasc4nbi448uiww056n3hud3ll1b3lekibd5rywb2pa0d6m07vtiqq1hyiqgs0l8zyshp3zxtxt8jk2obn8xhavkoo5nrrx2927p698m7dwr8q0mt9djk4sea53cvzdwh7w1t6q07fh5h3wolsyvehalo33nhtafgmegge6oktw507gfbwbrw884dqvzh0q5cqygy0cgfgayx86wwuk6jy4zsw0yjmbxtkj7ylttp1w5x4c7znnsw0nwe39a2v51493ffdrb7blsc3rj6vbt0pqf96ourlicon395sh9l7q9m34oyu9mc21tjckqddtjczrvdrcbcavq0h6vi31q3ovaujm94y6wbwttcjjn67zdmsb40b1q7lvgygpyldv5i395hs79jpjvzerdhrf7rikc9edm4e22iay1g38bomp31uodm8vmkijtruuz50xdwxlgqc5p1skqrg0jkd0z4n3axdnbo1miq24ypmynmbll4syzsvcdzm3fxp6c1g69wumsfgxgiigero8lf1m8j8k7u9tgc4m1odehwfmhn73rrvdfjj09q4n2bdizmw5n4xrx8pom0nwtabwzsreuufyu7ob6odvj3kc1bf46t0pm9fgykf0yun2baws49wlo7h3rmgvajz7zaqf6co9275qa4x13ect2bsa0oh3yuyv74yixc7pmxcrs4e06sv0c35uo4vpoup1mwghx7bd5mp49gfqo6n7cp1u2yh4pqm0ywv2x3lxgiyeoeoqz2y2i0f2hh5b6j5cnylejpetlhldboqh8y1pnbkk0wk3fwwdpauepwbill6dqiisdxvmb5pu4hifrf3n7wy6lekydmcz4eeu1k0rh9w7xzy72x74othus7b5n370a8nmkxzfqxtgmlwlt081vwpn26ibxunyvvigi4qcfzicmcw2ponaez54zor7f2d18dyxdrjsa9a8k4yrjj4x5713gt2rsqbriqd3p88dvoovn0pb99hn9zz9mxgm2kctnaxr5g1iyb73xxnbo7m6tbgz9smg5w15yr6p21hmciqzs4ycbyjyolinou1j7w5zqtxh0o2l6lu8qrpwf9gfz2mg0olkcr9dt1bo0lqibq4u11xod2sjjbl4ajpd9kfyuz8otfgppc49bg0uyixeucumtrl5nnbz9c12guwzw4mxgk5dwo1ep668ndahdqj1dnbop7o181s10dnw3b1g40zze6cbefd5mwtqvch05wlb07orytpdqwwhhvm4cpqucussdo2x1sb8dgqe57zpcc88sc0ahs2kuvvqvblaz9gluvbkzxh38ntfure3yvy898s3l8pfdq7ap0o81bhjgl8hoq9jpl4023fagkamsnsf3avq0938tcsbsm28otljq3f2myg1tzjao9h5juwtabzro5m7gpxvcs34ibtvpe05yehtu6y2o73s1d931hb0qv0y7d8y29ymdyz75s5ynct1uqd2vq5bylp25i2f7gcqikvygg9yjlp3sabfjm8dbzq7bl2gn4lujt5yscdfxkcugpe9xx36m96u4hopo7zk4jz50xqi2xlfvyi5pe89wytha7lmzlklgfeu75tbdlh0946wp9s47xa4eyqpqyarz7qxbuw2th58yzlgnoc6n8twi3jqwikdjycjsmlplkpk55razgkd1im65e73snsu59dhy780o2mt5gyed5zknqnzy6mhwfmvfmm49ctdch2n594j8vtgtw0y8mxoklrnly11mu2bn8vxh3ofkod2aaqwioy08gjwp84cn5zxbe7wxvady4xki7tqjh235030osk1hzbhkgbqub5jev4crr1z16pee7eorngwzyxa13d8n7tbadt2b1va701zw9trwljz1qa4fouetudgno64s6pvm58v5iwta2x3gxer8lq25j8g0bf4svlzrpvq42sk8lb2k13u0elhm01lm3wdsx0d3yzrr1brjsa54m5xqfq191057vk83t6aeax5qev2i2srbuumnl49bg86j8espxpjlnptk8s8ucuc1hscqo8wh8exs84otyruqgamxh3hxv162990zl08ikxmedsokds0vct6twp0mzdb2c8majtn2</details>](https://libraryofbabel.info/bookmark.cgi?dreamberd_rs_standard_library_10.16.2023)

DreamBerd has a fast-growing standard library. Due to the limitations of the file system, it must be copied and pasted into every file that uses it.

```c
const const use:Fn<T> = v:T->{
    const var o={}!
    o.call:Fn<Option<T>> = v:Option<T>->{
        var var r:T = self.value!
        if(;(v====undefined),
            self.value=v
        )!
        r
    }!
    o.value:T=v!
    o
}!

const const print:Fn<String> = t:String->{t?}!

const const str:Fn<T> = t:T->`${t}`!

const const identity:Fn<T> = t:T->t!

const const bool:Fn<T> = o:T->if(o,true,false,maybe)!

const const db:Fn<_> = ()->{
    print("   ___                     ___             __  ___  ____")!
    print("  / _ \_______ ___ ___ _  / _ )___ _______/ / / _ \/ __/")!
    print(" / // / __/ -_) _ `/  ' \/ _  / -_) __/ _  / / , _/\ \ ")!
    print("/____/_/  \__/\_,_/_/_/_/____/\__/_/  \_,_/ /_/|_/___/")!
}!

```
