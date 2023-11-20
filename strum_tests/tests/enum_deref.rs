use std::ops::{Deref, DerefMut};
use strum::EnumDeref;

mod core {} // ensure macros call `::core`

#[test]
fn homogeneous_enum() {
    #[derive(EnumDeref)]
    #[strum_deref_target(str)]
    enum Name {
        FirstName(String),
        NameAndSurname(String),
    }

    let first_name = Name::FirstName("Mario".to_owned());
    let name_n_surname = Name::NameAndSurname("Gordon Freeman".to_owned());

    {
        // Assert that the compiler accepts the derefs
        let _: &str = &first_name;
        let _: &str = &name_n_surname;
    }

    assert_eq!(first_name.deref(), "Mario");
    assert_eq!(name_n_surname.deref(), "Gordon Freeman");
}

#[test]
fn heterogeneous_enum() {
    #[derive(Debug, EnumDeref)]
    #[strum_deref_target([u32])]
    enum Collection {
        Fixed([u32; 8]),
        Dynamic(Vec<u32>),
    }

    let mut fixed = Collection::Fixed([0, 1, 2, 3, 4, 5, 6, 7]);
    let mut dynamic = Collection::Dynamic(Vec::from([10, 21, 32]));

    assert_eq!(&*fixed, &[0, 1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(&*dynamic, &[10, 21, 32]);

    {
        // Assert that the compiler accepts the derefs
        let _: &[u32] = &fixed;
        let _: &[u32] = &dynamic;

        let _: &mut [u32] = &mut fixed;
        let _: &mut [u32] = &mut dynamic;
    }

    fixed[4] = 400;
    dynamic[0] = 140;

    assert!(matches!(fixed, Collection::Fixed(a) if a[4] == 400));
    assert!(matches!(dynamic, Collection::Dynamic(v) if v.first().is_some_and(|i| *i == 140)));
}

#[test]
fn traits() {
    trait Walker {
        fn distance_walked(&self) -> f32;
        fn walk(&mut self);
    }

    struct Person {
        distance_walked: u32,
    }

    impl Walker for Person {
        fn distance_walked(&self) -> f32 {
            self.distance_walked as f32
        }

        fn walk(&mut self) {
            self.distance_walked += 500;
        }
    }

    struct Ant {
        feets_moved: u32,
    }

    impl Walker for Ant {
        fn distance_walked(&self) -> f32 {
            self.feets_moved as f32 * 0.125
        }

        fn walk(&mut self) {
            self.feets_moved += 1
        }
    }

    #[derive(EnumDeref)]
    #[strum_deref_target(dyn Walker)]
    enum AntiBox {
        Person(Person),
        Ant(Ant),
    }

    let mut person = AntiBox::Person(Person {
        distance_walked: 1000,
    });
    let mut ant = AntiBox::Ant(Ant { feets_moved: 0 });

    {
        // Assert that the compiler accepts the derefs
        // Because we're dealing with `dyn Trait`s, implicit deref-ing won't work
        // in variable assignments
        let _: &dyn Walker = person.deref();
        let _: &dyn Walker = ant.deref();

        let _: &mut dyn Walker = person.deref_mut();
        let _: &mut dyn Walker = ant.deref_mut();
    }

    assert_eq!(person.distance_walked(), 1000_f32);
    assert_eq!(ant.distance_walked(), 0_f32);

    person.walk();
    ant.walk();

    assert_eq!(person.distance_walked(), 1500_f32);
    assert_eq!(ant.distance_walked(), 0.125_f32);
}

#[test]
fn multi_level_deref() {
    #[derive(EnumDeref)]
    #[strum_deref_target(i32)]
    enum Storage {
        Stack(i32),
        Heap(Box<i32>),
    }

    let mut stack = Storage::Stack(5);
    let mut heap = Storage::Heap(Box::new(3));

    {
        // Assert that the compiler accepts the derefs
        let _: &i32 = &stack;
        let _: &i32 = &heap;

        let _: &mut i32 = &mut stack;
        let _: &mut i32 = &mut heap;
    }

    assert_eq!(*stack, 5);
    assert_eq!(*heap, 3);

    *stack += 10;
    *heap *= 2;

    assert_eq!(*stack, 15);
    assert_eq!(*heap, 6);
}
