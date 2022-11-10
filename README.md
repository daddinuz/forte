# forte

A stack-based, forth-like, esoteric programming language, read about [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)) on Wikipedia.  
Inspired by [BrainFart](https://github.com/daddinuz/brainfart), a BrainFuck interpreter.

### Instructions

| Opcode         | Stack effect                                                                          |
|----------------|---------------------------------------------------------------------------------------|
| number literal | push the number into the stack
| \+             | `( i j -- o   )` where `o := i + j`  add
| \-             | `( i j -- o   )` where `o := i - j`  subtract
| \*             | `( i j -- o   )` where `o := i * j`  multiply
| \/             | `( i j -- o   )` where `o := i / j`  divide
| \%             | `( i j -- o   )` where `o := i % j`  reminder
| \=             | `( i j -- o   )` where `o := i = j`  equal
| \>             | `( i j -- o   )` where `o := i > j`  greater
| \<             | `( i j -- o   )` where `o := i < j`  less
| \~             | `(   i -- o   )` where `o := ~i`     bitwise not
| \&             | `( i j -- o   )` where `o := i & j`  bitwise and
| \^             | `( i j -- o   )` where `o := i ^ j`  bitwise xor
| \|             | `( i j -- o   )` where `o := i \| j` bitwise or
| «              | `( i j -- o   )` where `o := i << j` shift left
| »              | `( i j -- o   )` where `o := i >> j` shift right *(signed)*
| \.             | `(   i --     )`                     pop
| \_             | `(   i -- i i )`                     duplicate
| \,             | `( i j -- j i )`                     swap
| \?             | `(     -- i   )`                     ask *(wait for incoming byte from stdin)*
| \!             | `(   i --     )`                     say *(output value as char)*
| ¡              | `(   i --     )`                     print *(output value as number)*
| \[             | `(   i --     )`                     move `i` into the loop stack. Loop while `i != 0` **see implementation details**
| \]             |                                      ends the matching loop **see implementation details**
| \{             | `(   i --     )`                     start function definition **see implementation details**
| \}             |                                      end function definition **see implementation details**
| @              | `(   i --     )`                     call function **see implementation details**
| $              |                                      return
| §              |                                      halt

### Implementation details

Only ASCII-encoding is supported, every character not listed in opcodes is ignored and will be treated as a comment.

- number literal
  
  ```
  42 42-
  ```
  
  Pushes 42 and -42 onto the stack
  
  **Note:** in this context spaces are important!
  
  ```
  42 42 -
  ```
  
  The code above pushes 2 times the number 42 into the stack,
  then subtracts the latters and pushes the result back.

- loops
  
  Forte has a special stack called `loop stack` in which loop control values
  are pushed after being removed from the `data stack`.
  
  At each iteration the current loop control value will be updated to next value toward 0.
  
  ```
   10 97  2 [ ! ]
   10 97 -2 [ ! ]
  ```
  
  both lines above produce the same output: "a\n"

- functions definitions/calls
  
  In forte a function is identified by a number.
  
  `{` will pop the value on top of the stack for identifing the function later
  
  In order to call a function a similar operation is done:
  `@` will pop the top of the stack to determine function to call.
  
  ```
  0{
    21 21 +
  }
  
  0@
  ```
  
  Note that functions can be redefined, and there are no undefined-functions: by default every identifier points to a no-op function.
  
  ```
  42@    \\ noop
  
  42{    \\ define function 42
    21 21 +
  }
  
  42{    \\ redefine function 42
    20 20 2 + +
  }
  ```
