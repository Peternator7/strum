use strum::{StaticVariantsArray, EnumDiscriminants};

mod core {} // ensure macros call `::core`

#[test]
fn simple() {
    #[derive(StaticVariantsArray, PartialEq, Eq, Debug)]
    enum Operation {
        Add,
        Sub,
        Mul,
        Div,
    }

    assert_eq!(
        Operation::ALL_VARIANTS,
        &[
            Operation::Add,
            Operation::Sub,
            Operation::Mul,
            Operation::Div,
        ]
    )
}

#[test]
fn in_enum_discriminants() {
    #[allow(dead_code)]
    #[derive(EnumDiscriminants)]
    #[strum_discriminants(derive(StaticVariantsArray))]
    #[strum_discriminants(name(GeometricShapeDiscriminants))]
    enum GeometricShape {
        Point,
        Circle(i32),
        Rectangle {
            width: i32,
            height: i32,
        }
    }

    assert_eq!(
        GeometricShapeDiscriminants::ALL_VARIANTS,
        &[
            GeometricShapeDiscriminants::Point,
            GeometricShapeDiscriminants::Circle,
            GeometricShapeDiscriminants::Rectangle,
        ]
    )
}

#[test]
fn empty_enum() {
    #[derive(StaticVariantsArray, PartialEq, Eq, Debug)]
    enum Empty {}

    assert_eq!(
        Empty::ALL_VARIANTS,
        &[],
    )
}
