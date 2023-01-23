#[derive(Debug)]
pub struct Memo<Function, Return>
where
    Return: Clone,
{
    get_value: Function,
    pub value: Option<Return>,
}

macro_rules! create_memo_impl {
    ($earg1:ident => $iarg1:ident; $earg2:ident => $iarg2:ident) => {
        impl<$iarg1, $iarg2, Return> Memo<fn($iarg1, $iarg2) -> Return, Return>
        where
            $iarg1: Eq,
            $iarg2: Eq,
            Return: Clone,
        {
            pub fn new(f: fn($iarg1, $iarg2) -> Return) -> Self {
                Self {
                    get_value: f,
                    value: None,
                }
            }

            pub fn run(&mut self, $earg1: $iarg1, $earg2: $iarg2) -> Option<Return> {
                match self.value {
                    Some(_) => self.value.to_owned(),
                    None => {
                        let value = Some((self.get_value)($earg1, $earg2));
                        self.value = value;
                        self.value.to_owned()
                    }
                }
            }
        }
    };
}

create_memo_impl!(arg1 => Arg1; arg2 => Arg2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_caches_functions() {
        fn multiply(a: i32, b: i32) -> isize {
            println!("Called!");
            return (a * b) as isize;
        }

        let mut memoized_compute = Memo::new(multiply);
        assert_eq!(memoized_compute.run(1, 10), Some(10));
    }
}
