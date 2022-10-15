//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Ongoings:
//  {{{
//  Ongoing: 2022-10-15T23:06:48AEDT floating-point add assert_eq assumes no <rounding> error(?) [...] (note: '    assert_eq!(0.1 + 0.2, 0.3)' fails)
//  Ongoing: 2022-10-15T23:10:36AEDT how to get the defintion of a given imported trait(?)
//  Ongoing: 2022-10-15T23:18:05AEDT implementing Add for ExampleComplex doesn't require it to be a copy type(?) [...] (implementing operators for copy vs non-copy types?)
//  Ongoing: 2022-10-15T23:58:04AEDT Eq vs PartialEq -> when each is used
//  Ongoing: 2022-10-16T00:35:23AEDT Array reference &[T] works with Vec<T> / other containers
//  }}}
#![allow(unused)]
#![allow(non_snake_case)]

use std::ops::Add;
use std::ops::Neg;
use std::cmp::PartialEq;
use std::ops::AddAssign;
use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Copy,Clone,Debug)]
struct ExampleComplex<T> { 
    re: T, im: T, 
}

impl<T> ExampleComplex<T> {
    fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

//  Rust supports operator overloading through implementing the relevent trait
fn example_operators_list()
{
    //      std::ops::Neg               -x
    //      std::ops::Not               !x
    //      std::ops::Add               x + y
    //      std::ops::Sub               x - y
    //      std::ops::Mul               x * y
    //      std::ops::Div               x / y
    //      std::ops::Rem               x % y
    //      std::ops::BitAnd            x & y
    //      std::ops::BitOr             x | y
    //      std::ops::BitXor            x ^ y
    //      std::ops::Shl               x << y
    //      std::ops::Shr               x >> y
    //      std::ops::AddAssign         x += y
    //      std::ops::SubAssign         x -= y
    //      std::ops::MulAssign         x *= y
    //      std::ops::DivAssign         x /= y
    //      std::ops::RemAssign         x %= y
    //      std::ops::BitAndAssign      x &= y
    //      std::ops::BitOrAssign       x |= y
    //      std::ops::BitXorAssign      x ^= y
    //      std::ops::ShlAssign         x <<= y
    //      std::ops::ShrAssign         x >>= y
    //      std::cmp::PartialEq         x == y, x != y
    //      std::cmp::PartialOrd        x < y, x <= y, x > y, x >= y
    //      <>Eq                        <>
    //      <>Ord                       <>
    //      std::ops::Index             x[y], &x[y]
    //      std::ops::IndexMut          x[y] = z, &mut x[y]
    //      <>Deref                     <> 
    //      <>DerefMut                  <>
    println!("example_operators_list, DONE");
}


fn example_arithmeticAndBitwise()
{
    //  Equivalent:
    //      a + b
    //      a.add(b)
    //  (using the later requires bringing 'std::ops::Add' into scope)
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);

    //  Definition of binary operator trait 'std::ops::Add'
    trait ExampleAdd<RHS=Self> {
        type Output;
        fn add(self, rhs: RHS) -> Self::Output;
    }

    //  Definition of unary operator trait 'std::ops::Neg'
    trait ExampleNeg {
        type Output;
        fn neg(self) -> Self::Output;
    }

    //  Definition of equality operator trait 'std::cmp::PartialEq'
    trait ExamplePartialEq<Rhs: ?Sized = Self> {
        fn eq(&self, other: &Rhs) -> bool;
        fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
    }

    //  To overload '+' for a given type, implement std::ops::Add for that type.
    impl<T> Add for ExampleComplex<T>
        where T: Add<Output=T>
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self { re: self.re + rhs.re, im: self.im + rhs.im }
        }
    }
    //  To overload '==' / '!=' for a given type, implement std::cmp::PartialEq for that type.
    impl<T> PartialEq for ExampleComplex<T> 
        where T: PartialEq
    {
        fn eq(&self, rhs: &ExampleComplex<T>) -> bool {
            self.re == rhs.re && self.im == rhs.im
        }
    }
    //  (alternatively, use '#[derive(PartialEq)]')

    //  PartialEq differs from Eq in that it does not require 'x == x' to be true
    //  (This is neccessary for floating point numbers, since NaN != Nan)
    //  (f32/f64 are the only standard library types that implement PartialEq but not Eq)

    //  Implement negation for ExampleComplex
    impl<T> Neg for ExampleComplex<T>
        where T: Neg<Output=T>
    {
        type Output = ExampleComplex<T>;
        fn neg(self) -> ExampleComplex<T> {
            ExampleComplex { re: -self.re, im: -self.im }
        }
    }

    assert!( ExampleComplex::new(5,7) + ExampleComplex::new(2,1) == ExampleComplex::new(7,8) );
    assert!( -ExampleComplex::new(5,-4) == ExampleComplex::new(-5,4) );

    //  Equivalent:
    //      a += b
    //      a.add_assign(b)
    //  (note that AddAssign is not implicitly implemented in terms of Add)
    
    //  Definition of 'std::ops::AddAssign'
    trait ExampleAddAssign<RHS=Self> {
        fn add_assign(&mut self, rhs: RHS);
    }

    //  Implement AddAssign for ExampleComplex
    impl<T> AddAssign for ExampleComplex<T> 
        where T: Add<Output=T> + Copy
    {
        fn add_assign(&mut self, rhs: Self) {
            self.re = self.re + rhs.re;
            self.im = self.im + rhs.im;
        }
    }

    //  Shl/Shr/ShlAssign/ShrAssign do not default RHS=Self

    println!("example_arithmeticAndBitwise, DONE");
}


fn example_ordered_comparisons()
{
    //  The comparison operators are overloaded with the trait 'std::cmp::PartialOrd'
    //  (Ord is to PartialOrd as Eq is to PartialEq)

    #[derive(PartialEq)]
    enum ExampleOrdering { 
        Less, Equal, Greater,
    }
    trait ExamplePartialOrd<Rhs = Self>: PartialEq<Rhs>
        where Rhs: ?Sized
    {
        fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
        fn lt(&self, other: &Rhs) -> bool { self.partial_cmp(&other) == Some(Ordering::Less) }
        fn gt(&self, other: &Rhs) -> bool { self.partial_cmp(&other) == Some(Ordering::Greater) }
        fn le(&self, other: &Rhs) -> bool { 
            match self.partial_cmp(&other) { 
                Some(Ordering::Less) | Some(Ordering::Equal) => true,
                _ => false,
            }
        }
        fn ge(&self, other: &Rhs) -> bool { 
            match self.partial_cmp(&other) { 
                Some(Ordering::Greater) | Some(Ordering::Equal) => true,
                _ => false,
            }
        }
    }

    #[derive(Debug,PartialEq)]
    struct Interval<T> {
        lower: T, upper: T,
    }
    impl<T> Interval<T> {
        fn new(lower: T, upper: T) -> Self {
            Self { lower, upper }
        }
    }

    //  Implement ordering for 'Interval'
    impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self == other {
                Some(Ordering::Equal)
            } else if self.lower >= other.upper {
                Some(Ordering::Greater)
            } else if self.upper <= other.lower {
                Some(Ordering::Less)
            } else {
                None    //  Ordering is not meaningful for overlapping intervals
            }
        }
    }
    assert!( Interval::new(10,20) < Interval::new(20,40) );
    assert!( Interval::new(7,8) > Interval::new(0,1) );
    assert!( Interval::new(7,8) <= Interval::new(7,8) );
    assert!( !( Interval::new(10,30) <= Interval::new(20,40) ) );
    assert!( !( Interval::new(10,30) >= Interval::new(20,40) ) );

    println!("example_ordered_comparisons, DONE");
}


fn example_Index_and_IndexMut() 
{
    //  Equivalent (for types other than Array):
    //      a[i]                a[i] = z
    //      *a.index(i)         *a.index_mut(i) = z

    //  Correct behaviour for out-of-bounds indexing is to trigger a panic

    trait ExampleIndex<Idx> {
        type Output: ?Sized;
        fn index(&self, index: Idx) -> &Self::Output;
    }
    trait ExampleIndexMut<Idx>: Index<Idx> {
        fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
    }
    //  (where Idx can be usize or Range<usize>)
    //  (HashMap/BTreeMap allow any hashable/ordered type as index)

    use std::collections::HashMap; 
    use std::ops::Index; 
    let mut m = HashMap::new(); 
    m.insert("十", 10); 
    m.insert("百", 100); 
    m.insert("千", 1000);
    m.insert("万", 1_0000); 
    m.insert("億", 1_0000_0000);
    assert_eq!(m["十"], 10); 
    assert_eq!(m["千"], 1000);   
    assert_eq!(*m.index("十"), 10); 
    assert_eq!(*m.index("千"), 1000);
    //  (note that HashMap does not support IndexMut)

    #[derive(Debug)]
    struct Image<T> { width: usize, pixels: Vec<T>, }

    impl<T: Default + Copy> Image<T> {
        fn new(width: usize, height: usize) -> Self {
            Image { width, pixels: vec![T::default(); width * height ] }
        }
    }

    //  Implementing 2d indexing for 'Image'
    impl<T> std::ops::Index<usize> for Image<T> {
        type Output = [T];
        fn index(&self, row: usize) -> &[T] {
            let start = row * self.width;
            &self.pixels[start .. start + self.width]
        }
    }
    impl<T> std::ops::IndexMut<usize> for Image<T> {
        fn index_mut(&mut self, row: usize) -> &mut [T] {
            let start = row * self.width;
            &mut self.pixels[start .. start + self.width]
        }
    }

    let mut i = Image::<u32>::new(10, 10);
    for row in 0 .. 10 {
        for col in 0 .. 10 {
            i[row][col] = (row + col) as u32;
        }
    }
    println!("i=({:?}", i);

    println!("example_Index_and_IndexMut, DONE");
}


fn example_other_operators()
{
    //  Not all operators can be overloaded
    //  Error checking '?' only works with Result values
    //  Logical '&&' / '||' only work with boolean values
    //  Range operator '..' only creates ranges
    //  '&' always borrows references

    //  The function call operator 'f()' cannot be overloaded 
    //  (use closures instead (see ch14))
}


fn main() 
{
    example_operators_list();
    example_arithmeticAndBitwise();
    example_ordered_comparisons();
    example_Index_and_IndexMut();
}

