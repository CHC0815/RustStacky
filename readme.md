# Stacky - a stack based programming language inspired by forth

## Implemented

- [x] [Built-in functions](#basic-functions): DUB, SWAP, DROP, PUTS
- [x] [Words](#word)
- [x] [If-Else-Then](#if)
- [x] [Do-Loops](#loop)
- [x] [Variables](#variables)

## Explanation

### Basic functions

- DUB : duplicates the last item on the stack
- SWAP : swaps the last two items on the stack
- DROP : drops the last item on the stack
- PUTS : takes a length n from the stack and the next n elements and prints them
- EMIT (.) : takes and prints the last item on the stack

### Word

A word defines a list of expressions that will be executed if the word is called.
The basic syntax for a word is:

```: [IDENTIFIER] [EXPRESSIONS] ;```

Example: 

```forth
: Add + ;
1 2 Add . 
```

will output 3

### IF

The basic syntax for an if-statement is:

```[CONDITION] IF [EXPRESSIONS] ELSE [EXPRESSIONS] THEN```

The IF-Expression pops the last item from the stack and checks the condition.
If the condition is met the if if-expressions-block is executed. Elsewise the else-expressions-block is executed.

A simple condition is: ```1 1 =``` which evaluates to 1 (true).

### LOOP

The basic syntax of a do-loop-expression is:

```[LIMIT] [INDEX] DO [EXPRESSIONS] LOOP```

The limit and the index get popped from the stack and are pushed to the loop-stack.
Every loop-iteration the limit and the index get popped from the loop-stack to check if the looping condition (index < limit) is valid.
After the loop-iteration the limit and the updated index are pushed back to the loop-stack.

### Variables

Variables can hold data.

To assign data to a variable use the ```->``` operator:

```1 -> X```

The number 1 is assigned to X. This operation will pop the 1 from the stack.

To get the data from an variable use the ```@``` operator:

```@ X```

This puts the data in X on the stack.

For now variables are global so this:

```
: Test @ X . ;
2 -> X
Test
```

will print 2