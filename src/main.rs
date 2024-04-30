use std::marker::PhantomData;

struct Bounded<'a, 'b: 'a, T: ?Sized>(&'a T, PhantomData<&'b ()>);

fn helper<'a, 'b, T: ?Sized, F>(input: &'a T, closure: F) -> &'b T
where
    F: for<'c> FnOnce(&'c T) -> Bounded<'b, '_, T>,
{
    closure(input).0
}

fn extend<'a, 'b, T: ?Sized>(input: &'a T) -> &'b T {
    helper(input, |x| Bounded(x, PhantomData))
}

fn main() {
    let s = String::from("aaaa");
    let a: &'static str = extend(s.as_str()); // turn &'a str into 'static
    drop(s);
    println!("{a}"); // <------------ Use after free!
}
