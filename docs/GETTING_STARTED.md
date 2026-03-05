# Getting started with flare

# Hello world
First create new dir and hello world file:
```bash
$ mkdir HelloWorld
$ cd HelloWorld
$ mkdir Hello.flare
```
Then open it in any editor. And write your first flare program.
For printing to console were using build-in macros like ```writeLn!()``` and ```write!()``` more about macros at [macros](MACROS.md) but macros will be more important later.

```flare
//defining hello world const
const hello_world:string = "Hello world!";
//writing out the output 
writeLn!(hello_world); 
```
Then run code:
```bash
$ flarec exec Hello.flare HelloWorld.out
```
```
Hello world!
```

Congats you wrote your first flare program.

Flare is statically typed so next thing will be types.


## Types
There are only 3. primitive types in flare:
1. **bool**:```true```/```false```
2. **string**:text values
3. **numb**:floats and intigers in one

And 1. sub type:
1. **printable**: both **string** and **numb** are printable becouse they can be printed out to the console.

And only 2. primitive values:

1. ```true```/```false```:**bool** values

Flare is statically type so you can't multiply bool by string etc. So this would be invalid:


```flare
"hello" + 5
//or
true * "flare"
//you probably know how it works now
```

### String operations
'+' is valid operator for strings so this will be ok:

```"hello "+"world"```

### Bool operators
'<' / '>' are operators that evaluates to bool. They are comparing two number like this:

```flare
6>8//false
8<10//true
//etc.
```


## Variables

### Variable decleration
```flare
var foo = "Hello world";

// Or you can do
var bar:string = "Hello world";

// Or
var hello:string;

//But you can't do this becouse flarec cannot infer type
var this_wont_work;
```
### Constant decleration
**One important rule:**  constant doesn't need to  have compile time known value like in [Rust](https://rust-lang.org) or [C#](https://dotnet.microsoft.com/en-us/download)
```flare
const bar = "Hello";
//etc.
//but you cant do this
const x:string;//since it doesn't have value it's useless
```
### Assigning values to variables

```flare
var x:string;
x = "hello";
// but this is invalid
x = true //becouse x is of typed string but true is of type bool
//etc.
```


# Statements
## If Statement
```flare
if(conditions){
  //do something
}
else{
  //do something else
}
```

---

Few rules for **if**

- condition must be of type **bool**
- than branch evaluates if condition is true
- else branch evaluates when condition is false
- else branch is not needed; if condition is false it just skip the than branch

## While Statement
```flare
while(condition){
  //do something
}
```

---

Few rules for **while**:
- condition must be of type **bool**
- body of the **while** evaluates while the condition is true
- when the condition is false it continues the program
- condition is evaluated at the end of each loop
## Scopes
- scopes start with ```{``` and ends with ```}```
- variables are added to current scope
**Example**
```Flare
if(something){
  var x = 5; //<-x defined here
}
x + 5 // wont work
```


# Functions
## Decleratin
```flare
fnc name(arg1:type,arg2:type):returnType //return type must be declared
{
  //do some stuff with args etc.

}
```
## Calling function
```flare
var foo = name_of_the_function(arguments);

//or
name_of_the_function(arguments);
```













