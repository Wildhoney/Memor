// #[derive(Debug)]
// pub struct Memo<Function, Return>
// where
//     Return: Clone,
// {
//     get_value: Function,
//     pub value: Option<Return>,
// }

// macro_rules! create_memo_impl {
//     ($($lowercase:ident => $uppercase:ident);*) => {
//         impl<$($uppercase),*, Return> Memo<fn($($uppercase),*) -> Return, Return>
//         where
//             $($uppercase: Eq),*,
//             Return: Clone,
//         {
//             pub fn new(f: fn($($uppercase),*) -> Return) -> Self {
//                 Self {
//                     get_value: f,
//                     value: None,
//                 }
//             }

//             pub fn run(&mut self, $($lowercase: $uppercase),*) -> Option<Return> {
//                 match self.value {
//                     Some(_) => self.value.to_owned(),
//                     None => {
//                         let value = Some((self.get_value)($($lowercase),*));
//                         self.value = value;
//                         self.value.to_owned()
//                     }
//                 }
//             }
//         }
//     };
// }

// create_memo_impl!(arg1 => Arg1; arg2 => Arg2);
// create_memo_impl!(arg1 => Arg1);

struct Memo {}

trait Memo1<A> {
    fn new(a: A) -> i32;
}

trait Memo2<A, B> {
    fn new(a: A, b: B) -> i32;
}

impl<A> Memo1<A> for Memo {
    fn new(a: A) -> i32 {
        5
    }
}

impl<A, B> Memo2<A, B> for Memo {
    fn new(a: A, b: B) -> i32 {
        5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_caches_functions() {
        let mut one = <Memo as Memo1<i32>>::new(1);
        // assert_eq!(one.run(10), Some(10));

        let mut two = <Memo as Memo2<i32, i32>>::new(1, 2);
        // assert_eq!(two.run(10, 10), Some(100));
    }
}
