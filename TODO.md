## TODO:
- [ ] Create a lexer - **In progress**
- [X] Syntax highlighting - **In progress** The basics are there, but the language changes every day so this will change with it for the time being
- [X] Make the lexer consistent. **In progress** I removed _most_ of the checks from the '=' match, however I still need to check backwards for untyped assignmnets
- [ ] Add floats!
- [ ] Standardize tokens/keywords/
- [ ] Add better command line argument parsing
    - [ ] Also add REPL?
- [ ] Add timings for lexing to README?
- [ ] Create a parser
    - [ ] Create AST
    - [ ] Research optimizations?
- [ ] Type checking
- [ ] Compile!

Not sure what I want to do for compiling yet, as in if I want to compile to x86_64 assembly (then use [NASM](https://nasm.us/) or [FASM](http://flatassembler.net/)), or transpile to another language like C. I think for the early parts I will transpile to C and have my compiler execute gcc on that C file. Then from there, slowly replace with asm.
