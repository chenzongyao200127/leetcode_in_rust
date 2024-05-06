use std::marker::PhantomData;

struct Bounded<'a, 'b: 'a, T: ?Sized>(&'a T, PhantomData<&'b ()>);

fn helper<'a, 'b, T: ?Sized, F>(input: &'a T, closure: F) -> &'b T
where
    'a: 'b,
    F: for<'c> FnOnce(&'c T) -> Bounded<'b, '_, T>,
{
    closure(input).0
}

fn extend<'a, 'b, T: ?Sized>(input: &'a T) -> &'b T
where
    'a: 'b,
{
    helper(input, |x| Bounded(x, PhantomData))
}

fn main() {
    let s = String::from("aaaa");
    // turn &'a str into 'static
    let s: &'static str = extend(&s);
    println!("{}", s);
}
