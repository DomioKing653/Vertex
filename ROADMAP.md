
# This is roadmap for Vertex PL
If you find something missing or finished just add or fix etc.
## MAIN GOALS
1. More robust variable and type system [_]
2. Better error rendering for parser and compiler [_]
3. Better docs [_]
4. Better linker [_]
## Lexer
- Make working lexer [_]
  1. Keywords [x]
  2. Numbers [x]
  3. Braces [x]
  4. Operators:[_]
    * +/- [x]
    * * / [x]
    * && / || [_]
- Make better formatted error handeling [x]
  1. Unknown char [x]
  2. Cannot parse empty file [x]
  3. Underminated string [x]
  4. More dots in a number [x]
## Parser
- Make better erros [_]
  1. Expected type [_]
  2. Expected token: foo [_]
- Working AST builder [x]:
  1. Statemnts
    * If [x]
    * Else [x]
    * While [x]
    * Functions [x]
    * Variables [x]
    * Returns [x]
    * Import [x]
  2. Expressions [x]
    * Plus [x]
    * Minus [x]
    * Times [x]
    * Divide [x]
    * Modulo [x]
## Bytecode
- Working bytecode emitter [_]
  1. If [x]
  2. Else [x]
  2. While [x]
  3. Return [_]
  4. Functions [x]
  5. Variables [x]
  6. Typedef [_]
  7. Structs [_]
  8. Imports [x]
- Working errors [_]:
  1. All errors in [compiler errors file](src/backend/errors/compiler) [_]
- Optimization [_]:
  1. Constant folding [x]
  2. Jmp [_]
## Linker
- Detect cyclic imports [x]
- Dependency sorter [x]
- Optimization [_]
## Compiler
- Working compiler with ```rustc ...``` [x]
- Make it simplier [_]
- Cross compilation [_]
