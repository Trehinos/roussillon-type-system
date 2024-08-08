# Changelog

## v0.3.7

- Add `get` method to `Byte` and other numeric structs.

## v0.3.8

- Implement `GetDataValue` trait for byte components.

## v0.3.7

- Add `get` method in `Byte`, `Word`, `Quad`, `Long`, `Wide`, `Arch`, `Integer` and `Float`.

## v0.3.6

- Add `new` method in `Byte`, `Word`, `Quad`, `Long`, `Wide`, `Arch`, `Integer` and `Float`.

## v0.3.5

- Add module `experimental` and feature `experiments`.

## v0.3.4

- New error `CannotCreateArchWithGivenSize` returned in the function `Bytes::try_from_arch()`.
- Add `Arch::size_of()` as a constant function.
- Implements `Copy` and `Default` (derive) for `Byte`, `Word`, `Quad`, `Long`, `Wide`, `Arch`, `Integer` and `Float`.
- Some documentation update.
- (experimental) Add `top_type()` in the `dynamic` module.

## v0.3.3

- Individual DataValues `Byte`, `Word`, `Quad`, `Long`, `Wide` and `Arch`.

## v0.3.2

- Add `ToRc` trait.
- Add `merge` method to 
- Experimental :
  - Effects.
  - `ValueType`, `Dynamic` and  `UnknownType`.

## v0.3.1

- Add `Record::field_from_name()`.
- Modified : module "typing" renamed to "types".
- (experimental) Interface, functions, effects.

## v0.3.0

- Complete `Label` and `Labelled` features.
- Add `LabelBank`.
- Add `Label<Type>` for `Enumeration` and `Structure` types.
- Add `Label<ValueCell>` for `Record`.

## v0.2.3

- `create_struct()`.
- `copy()`.

## v0.2.2

- Update `cargo.toml`.
- (started) `Interface` and `Traits`.
- (todo) Manage functions.

## v0.2.1

- Fix `construct_from_raw` in `Structure`.

## v0.2.0

- Add raw-value based constructors to `DataType` implementation and all types except for `FunctionType`.
- Add method `from()` in `List`, `Sequence`, `ProductValue`, `SumValue`, `Record` and `Union`.
- Add method `to_cell()` in `ProductValue` and `Record`.

## v0.1.5

- Add parsing functionality for `Boolean`, `Bytes`, `Integer` and `Float` values.
- Rename 'typing' module to 'types'.
- Rename 'identify' module to 'identity'.

## v0.1.4

- `Add` for `SumType`.
- `Mul` for `ProductType`.

## v0.1.3

- Add method `from` in `Integer` and `Float` values.
- Modify method `from` in `Reference` : the referenced type must be provided.

## v0.1.2

- Add method `from` in `Boolean` value.

## v0.1.1

- Add method `from` in `Reference` value.

## v0.1.0

Initial release :
- Boolean
- Bytes (arbitrarily size)
    - Byte (8 bits)
    - Arch (usize)
    - Word (16)
    - Quad (32)
    - Long (64)
    - Wide (128)
- Integer
- Float
- Reference
- List
- Sequence (Tuple values)
- Sum (SumpType values)
- Product (ProductType values)
- Union (Enumeration values)
- Record (Structure values)
- FunctionBody