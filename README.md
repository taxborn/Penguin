# Penguin Language
This is a language I want to build. Most of what is in the syntax examples below are not implemented yet.

## TODO:
- [ ] Create a lexer
- [ ] Create a parser
    - [ ] Create AST
- [ ] Compile.
Not sure what I want to do for compiling yet, as in if I want to compile to x86_64 assembly (then use [NASM](https://nasm.us/) or [FASM](http://flatassembler.net/)), or transpile to another language like C.

## Syntax Examples
### Comments:
There are two types of comments, single line and multiline comments
```
// Single line comments
```
```
/*
Multiline
Comments
*/
```
### Variable assignments:
Variables start with `var` and are assigned using the walrus operator, `:=`,  which is inspired by [Jai](https://inductive.no/jai/). You can either have implicit or static typing in your variables (however implicit typing  won't come for a while). If you are including a type, you can put it between the colon and equal sign like so:
```
// instead of:
var a := 5;
// you can statically type like so:
var a : u32 = 5;
// and white spaces are optional:
var a:u32=5;
```

Untyped declerations:
```
var a := 5; // u32
var b := 5.0001; // f32
```

Typed decleration:
```
var a : u16 = 5;
var b : f64 = 5.001;
```

### Functions:
Functions start with `func`, followed by the name of the function, and then followed by the parameters. It is then assigned (notice the walrus `:=` operator here, it's also used by variables) to a body. Functions **must** be typed.

`func main()` is special since it is the entrypoint of your program.

**Main:**
```
func main() := {
    // ...
}:
```

**User defined function (typed):**
```
func add_two(a : u32, b : u32) : u32 = {
    // ...
}:
```

### Imports
```
import std;

func main() := {
    // ... 
	std.print(c);
};
```
Or
```
import std.print;

func main() := {
    // ...
	print(c);
};
```

### Small example program
Putting all of those syntax exampels together, this is an example program put 
together to see how it all looks together. This will change as feedback is given
about the syntax.
```
import std.print;

func add_two(a : u32, b: u32) : u32 = {
    // Add the two inputs, a and b.
	return a + b;	
};

/*
This is main, the entrypoint of the program
*/
func main() := {
	var a := 16; // u32
	var b := 32; // u32

	var c := add_two(a, b); // 48, 

    // Print the result
    print(c);
};
```
