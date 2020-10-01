# Functions and Anonymous Functions

## Table of Content
* [Introduction](#Introduction)
* [Anonymous Functions](#Anonymous-Functions)
* [Default value in Arguments](#Default-value-in-Arguments)
* [Typing functions](#Typing-function)

## Introduction
Functions are used to run actions when are called, and to declare them the `func` keyword is used followed by the name, the arguments and the execution block.

The default return data type of the function is `void` and it does not return anything.

```sf
func printName() {
  print("Sflynlang");
}

printName();
// Output: Sflynlang

func getName(): string {
  return "Sflynlang";
}

print(getName());
// Output: Sflynlang
```

## Anonymous Functions
An anonymous function is the same of a normal function, but it is declared in a variable.

```sf
let printName = () => {
  print("Sflynlang");
};

printName();
// Output: Sflynlang

const getName = func (): string {
  return "Sflynlang";
};

print(getName());
// Output: Sflynlang
```

Besides, can use the anonymous functions in function arguments.

```sf
func getName(callback: (name: string) => void): void {
  callback('Sflynlang');
}

getName((name: string): void => {
  print(name);
  // Output: Sflynlang
});
```

## Default value in Arguments
The function arguments can have default values. If you set a value to one, the following arguments must have a default value as well.

```sf
func concat(str: string, str2: string = 'lang'): string {
  return str + str2;
}

print(concat("Sflyn"));
// Output: Sflynlang

print(concat("Sflyn", "-lang"));
// Output: Sflyn-lang
```

## Typing functions
```sf
const addTwo: (n: number) => number = func (n: number): number {
  return num + 2;
}
```
