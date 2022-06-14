std::io gives us, unsurprisingly, input/output
variables are immutable by default
expect() gives us the ability to do error handling
println!() is a macro -- more on those later
cargo works like npm but I'm sure it's better
cargo creates something similar to package.json / package_lock.json
Shadowing is super cool. No need to do intervening variables as you perform operations, specifically as it relates to changing types.
Match seems like switch/case
loop keyword creates an infinite loop

**Data Types**
blah blah numbers, chars, etc
*Tuples*
Tuples! These are awesome. Cannot shrink or grow in size.
rust has destructuring aw yiss. same as JS syntax-wise
access values in a tuple using tuple name then a dot then the index, ex: "x.1"
*Arrays*
Must all be the same type
array goes on stack rather than heap
array cannot grow or shrink in size, e.g. it has a fixed size after being declared
cf. to a vector (essentially a slice in go or an array in JS) which can change in size
access array elements the same way you would in other languages
rust will panic if you try to access a non-existant array index
**Functions**
main is the entry-point of a program
declare with "fn"
use snake_case
you must declare the types of each parameter in a function

*Statements and Expressions*
rust is an expression based language
statements = perform an action and do not return a value
expressions = evaluate to a resulting value
NOTE: an interesting tidbit about the difference between statements and expressions is that in other languages a statement, for example a variable declaration returns the value of the assignment, e.g. x = y = 6. Rust does not do this.
expressions don't need semi-colons,  but statements do

*functions with return values*
must declare type of return value from a function with "-> i32" for example
Rust does include implicit returns. The return value is the value of the final expression in a function block. You can use return though.

**Control Flow**
no parentheses on if statements in rust 
Rust does not convert non-bool types to booleans like JS so no if(number) xyz
if, else if, and else work the same as JS
yooo we can use if else in variable declarations but note possibilites cannot be of different types
*Loops*
loop gives us an infinite loop
break works like JS, but, super cool, we can declare a variable with a loop. the return value and the value that initializes the variable is the result of an expression that follows the break keyword
while loops exist
for/in works like JS
we can use ranges like so: for number in (1..4) {

   number //1, 2, 3, 4
  }
note ranges _include_ the first number, but _exclude_ the second number

