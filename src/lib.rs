use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Memo<Key, Function, Return>
where
    Return: Clone,
{
    map: HashMap<Key, Return>,
    get_value: Function,
}

macro_rules! create_memo_for {
    ($trait_name:ident -> $($let:ident => $type:ident);*) => {
        trait $trait_name<$($type),*, Return> {
            fn new(f: fn($($type),*) -> Return) -> Self;
            fn run(&mut self, $($let: $type),*) -> Return;
        }

        impl <$($type),*, Return> $trait_name<$($type),*, Return> for Memo<($($type),*), fn($($type),*) -> Return, Return>
        where
            $($type: Eq + Hash),*,
            Return: Clone,
        {
            fn new(f: fn($($type),*) -> Return) -> Self {
                Self {
                    get_value: f,
                    map: HashMap::new()
                }
            }

            fn run(&mut self, $($let: $type),*) -> Return {
                let key = ($($let),*);

                match self.map.get(&key) {
                    Some(value) => value.to_owned(),
                    None => {
                        let value = (self.get_value)($($let),*);
                        self.map.insert(key, value.to_owned());
                        value.to_owned()
                    }
                }
            }
        }
    };
}

create_memo_for!(Memo1 -> arg1 => Arg1);
// create_memo_for!(Memo2 -> arg1 => Arg1; arg2 => Arg2);
// create_memo_for!(Memo3 -> arg1 => Arg1; arg2 => Arg2; arg3 => Arg3);
// create_memo_for!(Memo4 -> arg1 => Arg1; arg2 => Arg2; arg3 => Arg3; arg4 => Arg4);
// create_memo_for!(Memo5 -> arg1 => Arg1; arg2 => Arg2; arg3 => Arg3; arg4 => Arg4; arg5 => Arg5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_memoizes_functions_for_i32() {
        // let mut f0 = <Memo<_, _,_> as Memo0<_>>::new(|| 10);
        // assert_eq!(f0.run(10), Some(10));

        let mut f1 = <Memo<_, _, _> as Memo1<_, _>>::new(|a| a);
        assert_eq!(f1.run(10), 10);
        assert_eq!(f1.run(20), 20);

        // let mut f2 = <Memo<_, _, _> as Memo2<_, _, _>>::new(|a, b| a * b);
        // assert_eq!(f2.run(10, 10), 100);

        // let mut f3 = <Memo<_, _, _> as Memo3<_, _, _, _>>::new(|a, b, c| a * b * c);
        // assert_eq!(f3.run(10, 10, 10), 1_000);

        // let mut f4 = <Memo<_, _, _> as Memo4<_, _, _, _, _>>::new(|a, b, c, d| a * b * c * d);
        // assert_eq!(f4.run(10, 10, 10, 10), 10_000);

        // let mut f5 =
        //     <Memo<_, _, _> as Memo5<_, _, _, _, _, _>>::new(|a, b, c, d, e| a * b * c * d * e);
        // assert_eq!(f5.run(10, 10, 10, 10, 10), 100_000);
    }
}
