# Penguin Language
This is an experimental language that I'm using to teach myself more about compilers.

**Currently working on:** The lexer.

## TODO
Moved to [TODO.md](TODO.md)

## Known issues:
- No floating point numbers

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
Variables start with `let` and are assigned using the walrus operator, `:=`,  which is inspired by [Jai](https://inductive.no/jai/). You can either have implicit or static typing in your variables (however implicit typing  won't come for a while). If you are including a type, you can put it between the colon and equal sign like so:
```
// instead of:
let a := 5;
// you can statically type like so:
let a : u32 = 5;
// and white spaces are optional:
let a:u32=5;
```

Untyped declerations:
```
let a := 5; // u32
let b := 5.0001; // f32
```

Typed decleration:
```
let a : u16 = 5;
let b : f64 = 5.001;
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
**TODO**: Determine if I want these imports in string literals or not.
E.g. `import std.print` vs. `import "std.print"`
```
import "std";

func main() := {
    // ... 
	std.print(c);
};
```
Or
```
import "std.print";

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
import "std.print";

func add_two(a : u32, b: u32) : u32 = {
    // Add the two inputs, a and b.
	return a + b;	
};

/*
This is main, the entrypoint of the program
*/
func main() := {
	let a := 16; // u32
	let b := 32; // u32

	let c := add_two(a, b); // 48, 

    // Print the result
    print(c);
};
```
