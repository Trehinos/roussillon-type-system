
pub mod identify;
pub mod typing;
pub mod value;

#[cfg(test)]
mod tests {
    use crate::typing::algebraic::{ProductType, SumType};
    use crate::typing::primitive::Primitive;
    use crate::typing::typedef::{Enumeration, Structure};
    use crate::value::number::{Float, Integer};
    use crate::value::record::Record;
    use crate::value::union::Union;

    #[test]
    fn test() {
        let class = Structure::new("MyClass", ProductType::new(&[
            Primitive::Integer.to_rc(),
            Primitive::Integer.to_rc(),
            Primitive::Float.to_rc(),
        ])).to_rc();

        let object = Record::new(class.clone(), &[
            Integer::new(40).to_cell(),
            Integer::new(96).to_cell(),
            Float::new(40.0).to_cell()
        ]).unwrap();
        println!("\n{:?}", object.clone().to_cell().borrow());
        for i in 0..3 {
            let field = object.get_field(i).unwrap();
            println!("\n{:?}", field.borrow());
        }

        let some = Structure::new("Some", ProductType::new(&[class])).to_rc();
        let none = Structure::new("None", ProductType::unit_type()).to_rc();
        let option_class = Enumeration::new("Option", SumType::new(&[
            none.clone(),
            some.clone()
        ])).to_rc();

        let mut union_object = Union::new(option_class, 1, Record::new(
            some.clone(),
            &[object.clone().to_cell()],
        ).unwrap().to_cell()).unwrap();
        println!("\n{:?}", union_object);
        println!("\n{:?}", union_object.current_value().borrow());

        union_object.set_cell(0, Record::new(none, &[]).unwrap().to_cell()).unwrap();
        println!("\n{:?}", union_object);
        println!("\n{:?}", union_object.current_value().borrow());

        union_object.set_cell(1, Record::new(some, &[object.to_cell()]).unwrap().to_cell()).unwrap();
        println!("\n{:?}", union_object);
        println!("\n{:?}", union_object.current_value().borrow());
    }
}
