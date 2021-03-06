
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::basic;
use crate::constants::{
    PI
};

//##########################################################################################################################

// Constants
const D0 = dec!(0);
const D1 = dec!(1);
const D2 = dec!(2);
const D1NEG = dec!(-1);
const PI2 = PI * D2;
const PIHALF = PI / D2;
const PINEG = PI * D1NEG;
const PIHNEG = PINEG / D2;

//##########################################################################################################################

fn trig_prepare(
    value: Decimal
) -> Decimal {
    let mut rem = value;
    rem = match true {
        (rem > PI) => rem - (
            (rem / PI2).floor() * PI2
        ),
        (rem < PINEG) => rem - (
            (rem / PI2).floor() * PI2
        ),
        _ => rem,
    };
    match true {
        (rem > PI) => rem - PI2,
        (rem < PINEG) => rem + PI2,
        _ => rem,
    }
}

//##########################################################################################################################

fn cos_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (0..terms).par_iter()
            .map(|n| [
                || basic::pow(value, 2 * n).unwrap(),
                || basic::fac(2 * n).unwrap(),
                || basic::pow(D1NEG, n).unwrap()
            ].par_iter()))
            .map(|t| t.map(|f| f()).collect())
            .map(|t| (t[0] / t[1]) * t[2])
            .reduce(|| D0, |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn cos(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    let rem = trig_prepare(value);
    Ok(
        match rem {
            PI => D1NEG,
            PIHALF => D0,
            D0 => D1,
            PIHNEG => D0,
            PINEG => D1NEG,
            _ => cos_series(terms, rem)?,
        }
    )
}

//##########################################################################################################################

fn sin_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (0..terms).par_iter()
            .map(|n| [
                || basic::pow(value, (2 * n) + 1).unwrap(),
                || basic::fac((2 * n) + 1).unwrap(),
                || basic::pow(D1NEG, n).unwrap()
            ].par_iter()))
            .map(|t| t.map(|f| f()).collect())
            .map(|t| (t[0] / t[1]) * t[2])
            .reduce(|| D0, |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn sin(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    let rem = trig_prepare(value);
    Ok(
        match rem {
            PI => D0,
            PIHALF => D1,
            D0 => D0,
            PIHNEG => D1NEG,
            PINEG => D0,
            _ => sin_series(terms, rem)?,
        }
    )
}

//##########################################################################################################################
