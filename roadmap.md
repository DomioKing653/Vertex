# This is roadmap for Vertex PL
If you find something missing or finished just add or fix etc.
## Lexer
- Make working lexer [x]
  1. Keywords [x]
  2. Numbers [x]
  3. Braces [x]
- Make better formatted error handeling [_]
  1. Unknown char [_]
  2. Cannot parse empty file [_]
  3. Underminated string [_]
  4. More dots in a number [_]
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
  1. All errors in (compiler errors file)[src/backend/errors/compiler] [_]
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
