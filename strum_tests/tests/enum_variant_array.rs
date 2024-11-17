use strum::{EnumDiscriminants, VariantArray};

mod core {} // ensure macros call `::core`

#[test]
fn simple() {
    #[derive(VariantArray, PartialEq, Eq, Debug)]
    enum Operation {
        Add,
        Sub,
        Mul,
        Div,
    }

    assert_eq!(
        Operation::VARIANTS,
        &[
            Operation::Add,
            Operation::Sub,
            Operation::Mul,
            Operation::Div,
        ]
    );
}

#[test]
fn in_enum_discriminants() {
    #[allow(dead_code)]
    #[derive(EnumDiscriminants)]
    #[strum_discriminants(derive(VariantArray))]
    #[strum_discriminants(name(GeometricShapeDiscriminants))]
    enum GeometricShape {
        Point,
        Circle(i32),
        Rectangle { width: i32, height: i32 },
    }

    assert_eq!(
        GeometricShapeDiscriminants::VARIANTS,
        &[
            GeometricShapeDiscriminants::Point,
            GeometricShapeDiscriminants::Circle,
            GeometricShapeDiscriminants::Rectangle,
        ]
    );
}

#[test]
fn empty_enum() {
    #[derive(VariantArray, PartialEq, Eq, Debug)]
    enum Empty {}

    assert_eq!(Empty::VARIANTS, &[],);
}

#[test]
fn variants_with_values() {
    #[derive(VariantArray, PartialEq, Eq, Debug)]
    enum WeekDay {
        Sunday = 0,
        Monday = 1,
        Tuesday = 2,
        Wednesday = 3,
        Thursday = 4,
        Friday = 5,
        Saturday = 6,
    }

    assert_eq!(
        WeekDay::VARIANTS,
        &[
            WeekDay::Sunday,
            WeekDay::Monday,
            WeekDay::Tuesday,
            WeekDay::Wednesday,
            WeekDay::Thursday,
            WeekDay::Friday,
            WeekDay::Saturday,
        ],
    );
}
