use syn::visit::Visit;

#[derive(Default)]
struct LifetimeVisitor {
    contains_lifetime: bool,
}

impl<'ast> Visit<'ast> for LifetimeVisitor {
    fn visit_lifetime(&mut self, i: &'ast syn::Lifetime) {
        self.contains_lifetime = true;
        syn::visit::visit_lifetime(self, i);
    }
}

pub fn contains_lifetime(ty: &syn::Type) -> bool {
    let mut visitor = LifetimeVisitor::default();
    visitor.visit_type(ty);

    visitor.contains_lifetime
}
