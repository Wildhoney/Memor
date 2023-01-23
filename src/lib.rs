#[derive(Debug)]
pub struct Memo<Function, Return>
where
    Return: Clone,
{
    get_value: Function,
    pub value: Option<Return>,
}

macro_rules! create_memo_impl {
    (<$ident_arg1:ident, $ident_arg2:ident>) => {
        impl<$ident_arg1, $ident_arg2, Return> Memo<fn($ident_arg1, $ident_arg2) -> Return, Return>
        where
            $ident_arg1: Eq,
            $ident_arg2: Eq,
            Return: Clone,
        {
            pub fn new(f: fn($ident_arg1, $ident_arg2) -> Return) -> Self {
                Self {
                    get_value: f,
                    value: None,
                }
            }

            pub fn run(&mut self, arg1: $ident_arg1, arg2: $ident_arg2) -> Option<Return> {
                match self.value {
                    Some(_) => self.value.to_owned(),
                    None => {
                        let value = Some((self.get_value)(arg1, arg2));
                        self.value = value;
                        self.value.to_owned()
                    }
                }
            }
        }
    };
}

create_memo_impl!(<Arg1, Arg2>);

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
