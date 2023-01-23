#[derive(Debug)]
pub struct Memo<Function, Return>
where
    Return: Clone,
{
    get_value: Function,
    pub value: Option<Return>,
}

macro_rules! create_memo_for {
    ($trait_name:ident -> $($lowercase:ident => $uppercase:ident);*) => {
        trait $trait_name<$($uppercase),*, Return> {
            fn new(f: fn($($uppercase),*) -> Return) -> Self;
            fn run(&mut self, $($lowercase: $uppercase),*) -> Option<Return>;
        }

        impl <$($uppercase),*, Return> $trait_name<$($uppercase),*, Return> for Memo<fn($($uppercase),*) -> Return, Return>
        where
            $($uppercase: Eq),*,
            Return: Clone,
        {
            fn new(f: fn($($uppercase),*) -> Return) -> Self {
                Self {
                    get_value: f,
                    value: None,
                }
            }

            fn run(&mut self, $($lowercase: $uppercase),*) -> Option<Return> {
                match self.value {
                    Some(_) => self.value.to_owned(),
                    None => {
                        let value = Some((self.get_value)($($lowercase),*));
                        self.value = value;
                        self.value.to_owned()
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
        // let mut f0 = <Memo<_, _> as Memo0<_>>::new(|| 10);
        // assert_eq!(f0.run(10), Some(10));

        let mut f1 = <Memo<_, _> as Memo1<_, _>>::new(|a| a);
        assert_eq!(f1.run(10), Some(10));

        let mut f2 = <Memo<_, _> as Memo2<_, _, _>>::new(|a, b| a * b);
        assert_eq!(f2.run(10, 10), Some(100));

        let mut f3 = <Memo<_, _> as Memo3<_, _, _, _>>::new(|a, b, c| a * b * c);
        assert_eq!(f3.run(10, 10, 10), Some(1_000));

        let mut f4 = <Memo<_, _> as Memo4<_, _, _, _, _>>::new(|a, b, c, d| a * b * c * d);
        assert_eq!(f4.run(10, 10, 10, 10), Some(10_000));

        let mut f5 =
            <Memo<_, _> as Memo5<_, _, _, _, _, _>>::new(|a, b, c, d, e| a * b * c * d * e);
        assert_eq!(f5.run(10, 10, 10, 10, 10), Some(100_000));
    }
}
