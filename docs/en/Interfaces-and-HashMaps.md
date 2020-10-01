# Interfaces and HashMaps

## Table of Content
* [Introduction](#Introduction)
* [Optional Properties](#Optional-Properties)
* [Function Types](#Function-Types)
* [Using HashMaps](#Using-HashMaps)
* [HashMap vs Interface](#HashMap-vs-Interface)

## Introduction
A `interface` in Sflynlang is a types map. In other words, you can configure the structure of a group of keywords to keep everything organized.

```sf
interface Sflyn {
  name: string;
  year: number;
}

const sflyn: Sflyn = {
  name: 'Sflynlang',
  year: 2020,
};

print(sflyn->toString());
// Output: { name: Sflynlang, year: 2020 }
```

## Optional Properties
The interfaces have optional properties and can be omitted in the declaration.

```sf
interface Label {
  name: string;
  size: Option<number>;
}

cont label1: Label = {
  name: 'Label 1',
};

print(label1->size->toString());
// Output: null

const label2: Label = {
  name: 'Label 2',
  size: 2,
};

print(label2->size->toString());
// Output: 2
```

## Function Types
```sf
interface Object {
  setName: (name: string) => void;
}

const obj: Object = {
  setName: (name: string): void => {
    print(name);
  },
};

obj->setName('Sflylang');
// Output: Sflynlang
```

See [Functions](./functions.md) for more information.

## Using HashMaps
If you don't want declare an `interface` to keep the control of a variable, can use the HashMaps. A `HashMap` doesn't have a declared type, it will be generated based to the value of the variable.

```sf
const obj = {
  setName: (name: string): void => {
    print(name);
  },
};

obj->setName('Sflynlang');
// Output: Sflynlang
```

## HashMap vs Interface
A `HashMap` only can be used in variables; on the other hand, an `Interface` can be used to define the structure of a `class` or a variable and as data type.
