use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Memo<Key, Function, Return>
where
    Return: Copy,
{
    map: HashMap<Key, Box<Return>>,
    get_value: Function,
}

macro_rules! create_memo_for {
    ($trait_name:ident -> $($let:ident => $type:ident);*) => {
        trait $trait_name<'a, $($type),*, Return> {
            fn new(f: fn($($type),*) -> Return) -> Self;
            fn run(&mut self, $($let: &'a $type),*) -> Return;
        }

        impl <'a, $($type),*, Return> $trait_name<'a, $($type),*, Return> for Memo<($(&'a $type),*), fn($($type),*) -> Return, Return>
        where
            $($type: 'a + Eq + Hash + Copy),*,
            Return: 'a + Copy,
        {
            fn new(f: fn($($type),*) -> Return) -> Self {
                Self {
                    get_value: f,
                    map: HashMap::new()
                }
            }

            fn run(&mut self, $($let: &'a $type),*) -> Return {
                let key = ($($let),*);

                match self.map.get(&key) {
                    Some(value) => **value,
                    None => {
                        let value = (self.get_value)($(*$let),*);
                        self.map.insert(key, Box::new(value));
                        value
                    }
                }
            }
        }
    };
}

create_memo_for!(Memo1 -> arg1 => Arg1);
create_memo_for!(Memo2 -> arg1 => Arg1; arg2 => Arg2);
create_memo_for!(Memo3 -> arg1 => Arg1; arg2 => Arg2; arg3 => Arg3);
create_memo_for!(Memo4 -> arg1 => Arg1; arg2 => Arg2; arg3 => Arg3; arg4 => Arg4);
create_memo_for!(Memo5 -> arg1 => Arg1; arg2 => Arg2; arg3 => Arg3; arg4 => Arg4; arg5 => Arg5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_memoizes_functions_for_i32() {
        // let mut f0 = <Memo<_, _,_> as Memo0<_>>::new(|| 10);
        // assert_eq!(f0.run(10), Some(10));

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
}
