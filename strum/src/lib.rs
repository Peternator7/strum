pub enum ParseError {
    VariantNotFound,
}

pub trait IntoEnumIterator {
    type Iterator;

    fn iter() -> Self::Iterator;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
