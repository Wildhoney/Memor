#[derive(Debug)]
pub struct Memo<Function, Return>
where
    Return: Clone,
{
    get_value: Function,
    pub value: Option<Return>,
}

impl<Arg1, Arg2, Return> Memo<fn(Arg1, Arg2) -> Return, Return>
where
    Arg1: Eq,
    Arg2: Eq,
    Return: Clone,
{
    pub fn new(f: fn(Arg1, Arg2) -> Return) -> Self {
        Self {
            get_value: f,
            value: None,
        }
    }

    pub fn run(&mut self, arg1: Arg1, arg2: Arg2) -> Option<Return> {
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
