# Variables

## Table of Content
* [Introduction](#Introduction)
* [Data Types](#Data-Types)
  * [String](#String)
  * [Number](#Number)
  * [Boolean](#Boolean)
  * [Null](#Null)
  * [Arrays](#Arrays)

## Introduction
Declare variables is easy, only must start with `let` or `const` followed by the name and the value, but what is the difference between use `let` and `const`?

When uses `let` to declare variables, you could set a new value for it after declare it. Otherwise, when uses `const` it is not possible.

If you don't specify the data type of the variable, Sflynlang will figure it for you:

```sf
// String.
let name = "Sflyn";
const name1 = "Sflyn";

// Number.
let year = 2020;
const year1 = 2020;

// Boolean.
let isExample = true;
const isExample1 = true;

// Array.
let numbers = [0, 1];
const numbers1 = [0, 1];
```

Or you can specify the data type like this:

```sf
let name: string = "Sflyn";
let year: number = 2020;
let isExample: boolean = true;
let numbers: number[] = [0, 1];
```

## Data Types
Sflynlang has five principal data types: `string`, `number`, `boolean`, `null` and arrays.

### String
Used to store texts. With strings you can concat two or more strings (If the value is not a string you need convert it to a string using `toString()` property).

You can declare strings using single quotes (`'`) or double quotes (`"`).

```sf
let name = "Sflyn";

name += 'lang';

print(name);
// Output: Sflynlang

print("Sflyn" + 'lang');
// Output: Sflynlang
```

### Number
Used to store decimal and floating numbers. With numbers you can use the following math operators: plus (`+`), minus (`-`), multiply (`*`), exponentiation (`**` or `^`), modulo (`%`) and divide (`/`).

```sf
let one = 1;
print(one->toString());
// Output: 1

let two = one + one;
print(two->toString());
// Output: 2

one -= two;
print(one->toString());
// Output: -1
```

### Boolean
Booleans are `true` or `false`, this is the most basic data type in Sflynlang.

```sf
let isExample = true;
print(isExample->toString());
// Output: true
```

### Null
Null is a value and a data type, you can use this when you don't want to set an empty string or a `0`.

```sf
let emptyString: Option<string> = null;
let emptyNumber: Option<number> = null;

// Then you can:
emptyString = "Sflynlang";
emptyNumber = 1;
```

### Arrays
Sflynlang only support one data type for every array.

```sf
let numbers = [0, 1, 2, 3];
print(numbers->toString());
// Output: [0, 1, 2, 3]

let letters = ['A', 'B', 'C', 'D'];
print(letters->toString());
// Output: [A, B, C, D]
```

You can get an array element like this:

```sf
let numbers = [0, 1, 2, 3];

print(numbers[0]->toString());
// Output: 0

numbers[0] = 10;
print(numbers[0]->toString());
// Output: 10

print(numbers[10]->toString());
// Output: null
```
