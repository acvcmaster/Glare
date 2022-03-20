# Glare programming language

Programming language with python-like syntax and rust-like features.

## Built-in types:
- Number
- str
- String
- None
- ()
- Never
- List
- Array
- Union types (typescript-line)

## Operators
- \+
- \-
- \*
- \/
- \/\/

## Sample code
```
def print_message(message: &str) -> ():
    print(f'The message was: {message}')

def add(a: Number, b: Number) -> Number:
    a + b

def sub(a: Number, b: Number) -> Number:
    return a - b

print_message('Hello, world!')

match sub(a, b), sub(c, d):
    1, _:
        print('Hello')
    2, _:
        mut z = y + 2
        z = z + 1
        print(f'{z}')
    _:
        print('Unexpected!')
 
```
 
## Grammar (BNF Notation)

### Literal
```
Literal:
	: NumberLiteral
	| StringLiteral
;
```

### Simple Type
```
SimpleType:
    : 'Number'
    | 'str'
    | 'String'
    | 'None'
    | '()'
    | 'Never'
    | 'List'
    | 'Array'
;
```

### Skip
```
Skip:
    : Whitespace
    | NewLine
;
```