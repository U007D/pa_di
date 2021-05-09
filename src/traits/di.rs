pub trait Di {
    // TODO
    // fn register_type_as_ownable<Typ>(self) -> Self;
    // fn register_type_as_sharable<Typ>(self) -> Self;
    // fn register_type_as_mutable<Typ>(self) -> Self;
    // fn register_type_as_cloneable<Typ: Clone>(self) -> Self;

    fn register_instance_as_ownable<Typ>(self, instance: Typ) -> Self;
    fn register_instance_as_sharable<Typ>(self, instance: Typ) -> Self;
    fn register_instance_as_mutable<Typ>(self, instance: Typ) -> Self;
    fn register_instance_as_cloneable<Typ: Clone>(self, instance: Typ) -> Self;

    fn resolve_ownable<Typ>(&mut self) -> Option<Typ>;
    fn resolve_sharable<Typ>(&mut self) -> &Typ;
    fn resolve_mutable<Typ>(&mut self) -> &mut Typ;
    fn resolve_cloneable<Typ: Clone>(&mut self) -> Typ;
}

// register  type/instance singleton/shared/distinct
