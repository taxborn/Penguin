" Vim syntax file
" Language: Penguin

" Usage Instructions
" Put this file in .vim/syntax/penguin.vim
" and add in your .vimrc file the next line:
" autocmd BufRead,BufNewFile *.pg set filetype=penguin

if exists("b:current_syntax")
  finish
endif

set iskeyword=a-z,A-Z,-,*,_,!,@

syntax keyword penguinTodos TODO FIXME NOTE

" Language keywords
syntax keyword penguinKeywords let func import

" Comments
syntax region penguinCommentLine start="//" end="$"          contains=penguinTodos
syntax region penguinMultiCommentLine start="/\*" end="\*/"  contains=penguinTodos

" String literals
syntax region penguinString start=/\v"/ skip=/\v\\./ end=/\v"/ contains=penguinEscapes

" Char literals
syntax region penguinChar start=/\v'/ skip=/\v\\./ end=/\v'/ contains=penguinEscapes

" Escape literals \n, \r, ....
syntax match penguinEscapes display contained "\\[nrt0\\\"']"

" Function definitions, matches the word '@func' followed by a word
syntax region penguinFuncDef start="@func" end=/\v\w+/ contains=penguinFuncName

" Function name
syntax match penguinFuncName display contained /\v\w+/

" Number literals
syntax match penguinNumber display contained /\v[0-9]+/

" Type names the compiler recognizes
syntax keyword penguinTypeNames u32 u16 i32 i16 f32 f16 bool char string

" Set highlights
highlight default link penguinTodos Todo
highlight default link penguinKeywords Keyword
highlight default link penguinCommentLine Comment
highlight default link penguinMultiCommentLine Comment
highlight default link penguinString String
highlight default link penguinNumber Statement
highlight default link penguinTypeNames Comment
highlight default link penguinChar Character
highlight default link penguinEscapes SpecialChar
highlight default link penguinFuncDef Function

let b:current_syntax = "penguin"

