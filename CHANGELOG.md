# Changelog

## v0.2.2

- (todo) Manage functions
- Update cargo.toml

## v0.2.1

- Fix construct_from_raw in Structure

## v0.2.0

- Add raw-value based constructors to DataType implementation and all types except for FunctionType.
- Add method 'from' in List, Sequence, ProductValue, SumValue, Record and Union.
- Add method 'to_cell' in ProductValue and Record.

## v0.1.5

- Add parsing functionality for Boolean, Bytes, Integer and Float values.
- Rename 'typing' module to 'types'.
- Rename 'identify' module to 'identity'.

## v0.1.4

- Add for SumType.
- Mul for ProductType.

## v0.1.3

- Add method 'from' in Integer and Float values.
- Modify method 'from' in Reference : the referenced type must be provided.

## v0.1.2

- Add method 'from' in Boolean value.

## v0.1.1

- Add method 'from' in Reference value.

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