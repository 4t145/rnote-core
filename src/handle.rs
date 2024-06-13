/// Magic args handler trait
pub trait Handler<'c, T, C> {
    fn apply(&self, c: &'c mut C);
}

pub trait Extract<'c, C>: 'c {
    fn extract(c: &'c mut C) -> Self;
}


macro_rules! impl_handler {
    ($($t:ident),*) => {
        #[allow(unused_variables, unused_parens, non_snake_case)]
        impl<'c, F, C, $($t),* > Handler<'c, ($($t,)*), C> for F
        where
            F: (Fn($($t,)*) ) + Clone ,
            $($t: Extract<'c, C>),*
        {
            fn apply(&self, c: &'c mut C) {
                $(let $t = $t::extract(c);)*
                (self.clone())($($t,)*)
            }
        }
    };
}

impl_handler!();
impl_handler!(T0);
impl_handler!(T0, T1);
impl_handler!(T0, T1, T2);
impl_handler!(T0, T1, T2, T3);
impl_handler!(T0, T1, T2, T3, T4);
impl_handler!(T0, T1, T2, T3, T4, T5);
impl_handler!(T0, T1, T2, T3, T4, T5, T6);
impl_handler!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_handler!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_handler!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_handler!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_handler!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_handler!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_handler!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_handler!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_handler!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
