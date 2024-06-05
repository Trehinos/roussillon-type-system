# Roussillon : Type System

> **This crate is an incomplete library** with a clear goal of creating a new language.  
> *Feel free to use it if needed. It is usable as is.*
>
> In future versions, this crate may provide new types or new features on values.
>
> When applicable, it will stay as uncorrelated as possible to the language.

This crate provides some structs and traits to manage types and typed values.

This crate goal **IS NOT** to manage memory allocation.

## Type system

### Typing module

This module offers a way to mark some data with a type.

#### Primitive types

These are the necessary types to construct any other meaningful type.
Usually, these are "machine types" :

- Boolean
- Byte
- Bytes
- Float
- Integer
- Reference
- List

#### The "tuple" type

- SequenceType

#### Algebraic Data Types

- SumType
- ProductType

#### Custom types (ADT with an Identifier)

- Enumeration (values are called unions).
- Structure

### Value module

The **value** module provides ways to create values from all types in the **typing** module.

The provided constructible values are :
- Boolean
- Bytes (arbitrarily size)
  - Byte (8 bits)
  - Word (16)
  - Quad (32)
  - Long (64)
  - Wide (128)
- Integer
- Float
- Reference
- List
- Sequence
- Sum
- Product
- Union
- Record

## License

(c) 2024 Sébastien Geldreich

This work is published under the MIT License.