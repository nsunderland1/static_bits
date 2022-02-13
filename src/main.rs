#![no_std]

use core::{
    marker::PhantomData,
    ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr, Sub},
};

use typenum::{
    IsGreaterOrEqual, IsLessOrEqual, Max, Min, Unsigned, U1, U128, U15, U16, U32, U47, U48, U49,
    U6, U74, U80,
};

// TODO: better Debug impl?
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct MaxBits<Width: Unsigned> {
    data: u128,
    _marker: PhantomData<*const Width>,
}

type MaximumWidth = U128;

impl<Width> MaxBits<Width>
where
    Width: Unsigned,
{
    fn fits(data: u128) -> bool
    where
        Width: Unsigned,
    {
        data.leading_zeros() >= (MaximumWidth::to_u32() - Width::to_u32())
    }

    pub fn new(data: u128) -> Option<Self> {
        if Self::fits(data) {
            Some(Self {
                data,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn into_inner(self) -> u128 {
        self.data
    }

    pub fn widen<Widened>(self) -> MaxBits<Widened>
    where
        Widened: Unsigned + IsGreaterOrEqual<Width>,
    {
        MaxBits {
            data: self.data,
            _marker: PhantomData,
        }
    }

    pub fn narrow<Narrowed>(self) -> Option<MaxBits<Narrowed>>
    where
        Narrowed: Unsigned + IsLessOrEqual<Width>,
    {
        MaxBits::<Narrowed>::new(self.data)
    }
}

impl<Width, Shift> Shl<Shift> for MaxBits<Width>
where
    Width: Add<Shift> + Unsigned,
    <Width as Add<Shift>>::Output: Unsigned,
    Shift: Unsigned,
    <Width as Add<Shift>>::Output: IsLessOrEqual<MaximumWidth>,
{
    type Output = MaxBits<<Width as Add<Shift>>::Output>;

    fn shl(self, _: Shift) -> Self::Output {
        Self::Output {
            data: self.data << Shift::to_u32(),
            _marker: PhantomData,
        }
    }
}

impl<Width, Shift> Shr<Shift> for MaxBits<Width>
where
    Width: Sub<Shift> + Unsigned,
    <Width as Sub<Shift>>::Output: Unsigned,
    Shift: Unsigned,
{
    type Output = MaxBits<<Width as Sub<Shift>>::Output>;

    fn shr(self, _: Shift) -> Self::Output {
        Self::Output {
            data: self.data >> Shift::to_u32(),
            _marker: PhantomData,
        }
    }
}

impl<Width, RhsWidth> BitOr<MaxBits<RhsWidth>> for MaxBits<Width>
where
    Width: Unsigned + Max<RhsWidth>,
    RhsWidth: Unsigned,
    <Width as Max<RhsWidth>>::Output: Unsigned,
{
    type Output = MaxBits<<Width as Max<RhsWidth>>::Output>;

    fn bitor(self, rhs: MaxBits<RhsWidth>) -> Self::Output {
        Self::Output {
            data: self.data | rhs.data,
            _marker: PhantomData,
        }
    }
}

impl<Width, RhsWidth> BitAnd<MaxBits<RhsWidth>> for MaxBits<Width>
where
    Width: Unsigned + Min<RhsWidth>,
    RhsWidth: Unsigned,
    <Width as Min<RhsWidth>>::Output: Unsigned,
{
    type Output = MaxBits<<Width as Min<RhsWidth>>::Output>;

    fn bitand(self, rhs: MaxBits<RhsWidth>) -> Self::Output {
        Self::Output {
            data: self.data & rhs.data,
            _marker: PhantomData,
        }
    }
}

impl<Width, RhsWidth> BitXor<MaxBits<RhsWidth>> for MaxBits<Width>
where
    Width: Unsigned + Max<RhsWidth>,
    RhsWidth: Unsigned,
    <Width as Max<RhsWidth>>::Output: Unsigned,
{
    type Output = MaxBits<<Width as Max<RhsWidth>>::Output>;

    fn bitxor(self, rhs: MaxBits<RhsWidth>) -> Self::Output {
        Self::Output {
            data: self.data ^ rhs.data,
            _marker: PhantomData,
        }
    }
}

impl<Width> Not for MaxBits<Width>
where
    Width: Unsigned,
{
    // Not much we can do to improve this bound without tracking
    // much more about the value, which doesn't seem reasonable
    type Output = MaxBits<MaximumWidth>;

    fn not(self) -> Self::Output {
        Self::Output {
            data: !self.data,
            _marker: PhantomData,
        }
    }
}

impl<Width, RhsWidth> Add<MaxBits<RhsWidth>> for MaxBits<Width>
where
    Width: Unsigned + Max<RhsWidth>,
    RhsWidth: Unsigned,
    <Width as Max<RhsWidth>>::Output: Unsigned + Add<U1>,
    <<Width as Max<RhsWidth>>::Output as Add<U1>>::Output: Unsigned + IsLessOrEqual<MaximumWidth>,
{
    type Output = MaxBits<<<Width as Max<RhsWidth>>::Output as Add<U1>>::Output>;

    fn add(self, rhs: MaxBits<RhsWidth>) -> Self::Output {
        Self::Output {
            data: self.data + rhs.data,
            _marker: PhantomData,
        }
    }
}

impl<Width, RhsWidth> Sub<MaxBits<RhsWidth>> for MaxBits<Width>
where
    Width: Unsigned + Sub<U1>,
    RhsWidth: Unsigned + IsLessOrEqual<Width>,
    <Width as Sub<U1>>::Output: Unsigned,
{
    type Output = MaxBits<<Width as Sub<U1>>::Output>;

    fn sub(self, rhs: MaxBits<RhsWidth>) -> Self::Output {
        Self::Output {
            data: self.data + rhs.data,
            _marker: PhantomData,
        }
    }
}

fn main() {
    let bits = MaxBits::<U16>::new(0xffff).unwrap();
    let shifted: MaxBits<U32> = (bits << U15::new()).widen();
    let bigger = MaxBits::<U48>::new(0xcdbaef123456).unwrap();
    let summed: MaxBits<U49> = shifted + bigger;

    let bits = MaxBits::<U128>::new(u128::MAX).unwrap();
    let one = MaxBits::<U1>::new(1).unwrap();
    let eighty_mask: MaxBits<U80> = (one << U80::new()) - one;
    let bits: MaxBits<U80> = bits & eighty_mask;
    let six_mask: MaxBits<U6> = (one << U6::new()) - one;
    let six_bits: MaxBits<U6> = (eighty_mask & (six_mask << U74::new())) >> U74::new();
}
