## TODO:
- [ ] Create a lexer - **In progress**
- [X] Syntax highlighting - **In progress** The basics are there, but the language changes every day so this will change with it for the time being
- [X] Make the lexer consistent. **In progress** I removed _most_ of the checks from the '=' match, however I still need to check backwards for untyped assignmnets
- [ ] Add floats!
- [ ] Standardize tokens/keywords/
- [ ] Create a parser
    - [ ] Create AST
    - [ ] Research optimizations?
- [ ] Type checking
- [ ] Compile.
Not sure what I want to do for compiling yet, as in if I want to compile to x86_64 assembly (then use [NASM](https://nasm.us/) or [FASM](http://flatassembler.net/)), or transpile to another language like C.
