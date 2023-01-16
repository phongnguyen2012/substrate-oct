use crate::Config;
// use crate::balances;
pub fn sqrt<T: Config> (y: T::Balance) -> T::Balance {
    if y > T::Balance::from(3u32) {
        let mut z = y;
        let mut x: T::Balance = y / T::Balance::from(2u32);
        x += T::Balance::from(1u32);
        while x < z {
            z = x;
            x = (y / x + x) / T::Balance::from(2u32);
        }
        z
    } else if y != T::Balance::from(0u32) {
        let z = T::Balance::from(1u32);
        z
    } else {
        y
    }
}

pub fn min<T: Config>(
    x: T::Balance,
    y: T::Balance,
) -> T::Balance {
    let z = match x < y {
        true => x,
        _ => y,
    };
    z
}

pub fn absdiff<T: Config>(
    x: T::Balance,
    y: T::Balance,
) -> T::Balance {
    let z = match x < y {
        true => y-x,
        _ => x-y,
    };
    z
}

