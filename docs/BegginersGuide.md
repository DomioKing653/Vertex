## Getting Started
First to install the compiler you need to install the vertexC and libvm_runtime.a
Then if you are on Linux just:
```shell
chmod +x vertexC
chmod +x libvm_runtime.a
```

and done you have just installed Vertex.

## Hello World
In every programming language the first program you usually make is the hello world.
For the Hello World program you need 1 thing some form of print.

In vertex we can write to the terminal using `writeLn!();` or `write!();`. The difference between the two `writeLn!();` adds the new line character while `write!();` does not.

As for the hello world it can look something like this:
```
writeLn!("Hello World");
```
or 
```
write!("Hello World");
```

## Data Types
Vertex is statically typed which means types are known at runtime.
In vertex we can currently use 4 types:
1. `bool` the classic true false 
2. `int` a normal integer like 1 or 2
3. `flt` a floating point integer like 1.5 or 3.14
4. `string` a string like "hello"
Types can be inferred in cases like:
```
var x = 10; # inferrs to an if 
```
## Variables
To declare a variable we use the `var` keyword or `const` for a constant.
For example:
```
var number: int = 10;
var float: flt = 3.14;
var name: string = "vertex";
var boolien: bool = true;
```

Types can be separated to two categories:
1. Printable
	1. `int`
	2. `flt`
	3. `string`
2. Non Printable
	1. `bool`
The difference between the two categories is in the name you can only print, printable characters.

## User input
We can take user input by using the `readInput!()` macro.
For example:
```
const userName:string = readInput!("Whats your name?");

writeLn!("Your name is",userName)
```
## Conditions
The `if` and `else` statements can be used to evaluate a condition to `bool` simple yet elegant.
```
if (condition) {
# Do stuff
}else{
# Do other stuff
}
```
There are only 2 supported conditions `>` and `<`.

## Loops
We have the good old `while` as our main loop here which works similar to an if but it repeats the body of it as long as the condition evaluates to `true`.

Example:
```
while(condition) {
# Loop body
}
```

## Functions
A function is a fundamental part of code and also extremely simple. We use the `fnc` keyword for declaring a function and functions follow a simple structure of:
```
fnc name(arg: type, arg2: type): return_type {
# Function body
}
```

The available return types are the Data Types and `void`. `void` is used in functions that return nothing.

Example function:
```
fnc add(a: int, b: int): int {
	var sum: int = a + b;
	return sum;
}

var x: int = add(1, 2);
writeLn!(x);
```

Example void function:
```
fnc hello() {
	writeLn!("hello");
}

hello();
```

## Exit process
We can use the `processExit!()` macro to exit the program with an `int` argument.
```
const x: int = 5;
if(x > 7){
  processExit!(195);
}
else{
  processExit!(x);
}
