use std::ops::Deref;

use bitcoin::{Amount as BdkAmount, Denomination as BdkDenomination};
use wasm_bindgen::prelude::wasm_bindgen;

/// Amount
///
/// The [Amount] type can be used to express Bitcoin amounts that support
/// arithmetic and conversion to various denominations.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Amount {
    amount: BdkAmount,
}

#[wasm_bindgen]
impl Amount {
    /// Gets the number of satoshis in this [`Amount`].
    pub fn to_sat(&self) -> u64 {
        self.amount.to_sat()
    }

    /// Express this [`Amount`] as a floating-point value in Bitcoin.
    ///
    /// Please be aware of the risk of using floating-point numbers.
    pub fn to_btc(&self) -> f64 {
        self.amount.to_btc()
    }

    /// Express this [Amount] as a floating-point value in the given denomination.
    ///
    /// Please be aware of the risk of using floating-point numbers.
    pub fn to_float_in(&self, denom: Denomination) -> f64 {
        self.amount.to_float_in(denom.into())
    }
}

impl Deref for Amount {
    type Target = BdkAmount;

    fn deref(&self) -> &Self::Target {
        &self.amount
    }
}

impl From<BdkAmount> for Amount {
    fn from(amount: BdkAmount) -> Self {
        Amount { amount }
    }
}

/// A set of denominations in which amounts can be expressed.
#[wasm_bindgen]
#[derive(Debug)]
pub enum Denomination {
    /// BTC
    Bitcoin,
    /// cBTC
    CentiBitcoin,
    /// mBTC
    MilliBitcoin,
    /// uBTC
    MicroBitcoin,
    /// nBTC
    NanoBitcoin,
    /// pBTC
    PicoBitcoin,
    /// bits
    Bit,
    /// satoshi
    Satoshi,
    /// msat
    MilliSatoshi,
}

impl From<BdkDenomination> for Denomination {
    fn from(denom: BdkDenomination) -> Self {
        match denom {
            BdkDenomination::Bitcoin => Denomination::Bitcoin,
            BdkDenomination::CentiBitcoin => Denomination::CentiBitcoin,
            BdkDenomination::MilliBitcoin => Denomination::MilliBitcoin,
            BdkDenomination::MicroBitcoin => Denomination::MicroBitcoin,
            BdkDenomination::NanoBitcoin => Denomination::NanoBitcoin,
            BdkDenomination::PicoBitcoin => Denomination::PicoBitcoin,
            BdkDenomination::Bit => Denomination::Bit,
            BdkDenomination::Satoshi => Denomination::Satoshi,
            BdkDenomination::MilliSatoshi => Denomination::MilliSatoshi,
            _ => panic!("Unsupported denomination"),
        }
    }
}

impl From<Denomination> for BdkDenomination {
    fn from(denom: Denomination) -> Self {
        match denom {
            Denomination::Bitcoin => BdkDenomination::Bitcoin,
            Denomination::CentiBitcoin => BdkDenomination::CentiBitcoin,
            Denomination::MilliBitcoin => BdkDenomination::MilliBitcoin,
            Denomination::MicroBitcoin => BdkDenomination::MicroBitcoin,
            Denomination::NanoBitcoin => BdkDenomination::NanoBitcoin,
            Denomination::PicoBitcoin => BdkDenomination::PicoBitcoin,
            Denomination::Bit => BdkDenomination::Bit,
            Denomination::Satoshi => BdkDenomination::Satoshi,
            Denomination::MilliSatoshi => BdkDenomination::MilliSatoshi,
        }
    }
}
