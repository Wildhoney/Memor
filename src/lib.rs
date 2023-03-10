use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Memo<Key, Function, Return>
where
    Return: Copy,
{
    map: Option<HashMap<Key, Box<Return>>>,
    value: Option<Return>,
    get_value: Function,
}

macro_rules! create {
    ($trait_name:ident) => {
        trait $trait_name<'a, Return> {
            fn new(f: fn() -> Return) -> Self;
            fn run(&mut self) -> Return;
        }

        impl <'a, Return> $trait_name<'a, Return> for Memo<(), fn() -> Return, Return>
        where
            Return: 'a + Copy,
        {
            fn new(f: fn() -> Return) -> Self {
                Self {
                    get_value: f,
                    value: None,
                    map: None
                }
            }

            fn run(&mut self) -> Return {
                match self.value {
                    Some(value) => value,
                    None => {
                        let value = (self.get_value)();
                        self.value = Some(value);
                        value
                    }
                }
            }
        }
    };

    ($trait_name:ident -> $($let:ident => $type:ident);*) => {
        trait $trait_name<'a, $($type),*, Return> {
            fn new(f: fn($($type),*) -> Return) -> Self;
            fn run(&mut self, $($let: &'a $type),*) -> Return;
        }

        impl <'a, $($type),*, Return> $trait_name<'a, $($type),*, Return> for Memo<($(&'a $type),*,), fn($($type),*) -> Return, Return>
        where
            $($type: 'a + Eq + Hash + Copy),*,
            Return: 'a + Copy,
        {
            fn new(f: fn($($type),*) -> Return) -> Self {
                Self {
                    get_value: f,
                    value: None,
                    map: Some(HashMap::new())
                }
            }

            fn run(&mut self, $($let: &'a $type),*) -> Return {
                let key = ($($let),*,);
                let map = self.map.as_mut().unwrap();

                match map.get(&key) {
                    Some(value) => **value,
                    None => {
                        let value = (self.get_value)($(*$let),*);
                        map.insert(key, Box::new(value));
                        value
                    }
                }
            }
        }
    };
}

create!(Memo0);
create!(Memo1 -> arg1 => Arg1);
create!(Memo2 -> arg1 => Arg1; arg2 => Arg2);
create!(Memo3 -> arg1 => Arg1; arg2 => Arg2; arg3 => Arg3);
create!(Memo4 -> arg1 => Arg1; arg2 => Arg2; arg3 => Arg3; arg4 => Arg4);
create!(Memo5 -> arg1 => Arg1; arg2 => Arg2; arg3 => Arg3; arg4 => Arg4; arg5 => Arg5);

#[macro_export]
macro_rules! memo (
    ($fn:expr) => { <Memo<_, _, _> as Memo0<_>>::new($fn) };
    ($fn:expr => $arg1:tt) => { <Memo<_, _, _> as Memo1<$arg1, _>>::new($fn) };
    ($fn:expr => $arg1:tt, $arg2:tt) => { <Memo<_, _, _> as Memo2<$arg1, $arg2, _>>::new($fn) };
    ($fn:expr => $arg1:tt, $arg2:tt, $arg3:tt) => { <Memo<_, _, _> as Memo3<$arg1, $arg2, $arg3, _>>::new($fn) };
    ($fn:expr => $arg1:tt, $arg2:tt, $arg3:tt, $arg4:tt) => { <Memo<_, _, _> as Memo4<$arg1, $arg2, $arg3, $arg4, _>>::new($fn) };
    ($fn:expr => $arg1:tt, $arg2:tt, $arg3:tt, $arg4:tt, $arg5:tt) => { <Memo<_, _, _> as Memo5<$arg1, $arg2, $arg3, $arg4, $arg5 ,_>>::new($fn) };
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_memoizes_functions_for_struct() {
        let f = || 10;
        let mut f0 = <Memo<_, _, _> as Memo0<_>>::new(f);
        assert_eq!(f0.run(), 10);

        let f = |a| a;
        let mut f1 = <Memo<_, _, _> as Memo1<_, _>>::new(f);
        assert_eq!(f1.run(&10), 10);
        assert_eq!(f1.run(&20), 20);

        let f = |a, b| a * b;
        let mut f2 = <Memo<_, _, _> as Memo2<_, _, _>>::new(f);
        assert_eq!(f2.run(&10, &10), 100);
        assert_eq!(f2.run(&20, &20), 400);

        let f = |a, b, c| a * b * c;
        let mut f3 = <Memo<_, _, _> as Memo3<_, _, _, _>>::new(f);
        assert_eq!(f3.run(&10, &10, &10), 1_000);
        assert_eq!(f3.run(&20, &20, &20), 8_000);

        let f = |a, b, c, d| a * b * c * d;
        let mut f4 = <Memo<_, _, _> as Memo4<_, _, _, _, _>>::new(f);
        assert_eq!(f4.run(&10, &10, &10, &10), 10_000);
        assert_eq!(f4.run(&20, &20, &20, &20), 160_000);

        let f = |a, b, c, d, e| a * b * c * d * e;
        let mut f5 = <Memo<_, _, _> as Memo5<_, _, _, _, _, _>>::new(f);
        assert_eq!(f5.run(&10, &10, &10, &10, &10), 100_000);
        assert_eq!(f5.run(&20, &20, &20, &20, &20), 3_200_000);
    }

    #[test]
    fn it_memoizes_functions_for_macro() {
        let f = || 10;
        let mut f0 = memo!(f);
        assert_eq!(f0.run(), 10);

        let f = |a| a;
        let mut f1 = memo!(f => i32);
        assert_eq!(f1.run(&10), 10);
        assert_eq!(f1.run(&20), 20);

        let f = |a, b| a * b;
        let mut f2 = memo!(f => i32, i32);
        assert_eq!(f2.run(&10, &10), 100);
        assert_eq!(f2.run(&20, &20), 400);

        let f = |a, b, c| a * b * c;
        let mut f3 = memo!(f => i32, i32, i32);
        assert_eq!(f3.run(&10, &10, &10), 1_000);
        assert_eq!(f3.run(&20, &20, &20), 8_000);

        let f = |a, b, c, d| a * b * c * d;
        let mut f4 = memo!(f => i32, i32, i32, i32);
        assert_eq!(f4.run(&10, &10, &10, &10), 10_000);
        assert_eq!(f4.run(&20, &20, &20, &20), 160_000);

        let f = |a, b, c, d, e| a * b * c * d * e;
        let mut f5 = memo!(f => i32, i32, i32, i32, i32);
        assert_eq!(f5.run(&10, &10, &10, &10, &10), 100_000);
        assert_eq!(f5.run(&20, &20, &20, &20, &20), 3_200_000);
    }
}
