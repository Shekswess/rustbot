//! Current residential transaction tax bands from HMRC, Revenue Scotland and WRA.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use super::{comma, price};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(MILLIONS, r"\b(\d+(?:\.\d+)?)\s*(?:million|m)\b");
pattern!(
    FIRST,
    r"\b(?:first[- ]time|first home|first house|first flat|first property|first place|our first)\b"
);
pattern!(
    SECOND,
    r"\b(?:second home|buy[- ]to[- ]let|second property|additional home|additional dwelling|investment property|second house|rental property|landlord)\b"
);
pattern!(
    SCOTLAND,
    r"\b(?:scotland|scottish|lbtt|land and buildings|edinburgh|glasgow|aberdeen|dundee|inverness)\b"
);
pattern!(
    WALES,
    r"\b(?:wales|welsh|ltt|land transaction tax|cardiff|swansea|newport)\b"
);
pattern!(
    NON_RESIDENT,
    r"\b(?:non[- ]resident|overseas buyer|live abroad)\b"
);
pattern!(
    SPECIAL,
    r"\b(?:commercial|non[- ]residential|mixed[- ]use|corporate|company buy|(?:in|from|before) 20(?:1[0-9]|2[0-4]))\b"
);

// Each rate is in basis points and applies above the band floor.
type Bands = &'static [(u64, u64)];
const SDLT: Bands = &[
    (0, 0),
    (125_000, 200),
    (250_000, 500),
    (925_000, 1_000),
    (1_500_000, 1_200),
];
const FIRST_SDLT: Bands = &[(0, 0), (300_000, 500)];
const LBTT: Bands = &[
    (0, 0),
    (145_000, 200),
    (250_000, 500),
    (325_000, 1_000),
    (750_000, 1_200),
];
const LTT: Bands = &[
    (0, 0),
    (225_000, 600),
    (400_000, 750),
    (750_000, 1_000),
    (1_500_000, 1_200),
];
const HIGH_LTT: Bands = &[
    (0, 500),
    (180_000, 850),
    (250_000, 1_000),
    (400_000, 1_250),
    (750_000, 1_500),
    (1_500_000, 1_700),
];

fn duty(price: u64, bands: Bands, surcharge: u64) -> u64 {
    bands
        .iter()
        .zip(
            bands
                .iter()
                .skip(1)
                .map(|band| band.0)
                .chain(std::iter::once(price)),
        )
        .map(|(&(floor, rate), top)| {
            u128::from(price.min(top).saturating_sub(floor)) * u128::from(rate + surcharge)
        })
        .sum::<u128>()
        .div_euclid(10_000) as u64
}

pub(super) fn reply(text: &str) -> Cow<'static, str> {
    let price = MILLIONS
        .captures_iter(text)
        .filter_map(|captures| captures[1].parse::<f64>().ok())
        .filter(|value| value.is_finite() && *value < 1e9)
        .map(|value| (value * 1e6) as u64)
        .chain(std::iter::once(price(text)))
        .max()
        .unwrap_or(0);
    let first = FIRST.is_match(text);
    let second = SECOND.is_match(text);
    let special = SPECIAL.is_match(text);
    let (name, details, calculator, amount) = if SCOTLAND.is_match(text) {
        let extra = if second && price >= 40_000 {
            price * 8 / 100
        } else {
            0
        };
        let relief = if first && !second {
            price.saturating_sub(145_000).min(30_000) * 2 / 100
        } else {
            0
        };
        (
            "Scotland's LBTT",
            "The standard nil band ends at £145,000. Eligible first-time buyers get a nil band up to £175,000, saving up to £600. The Additional Dwelling Supplement is generally 8% of the price on qualifying purchases of £40,000 or more.",
            "Revenue Scotland's property transaction calculator",
            duty(price, LBTT, 0) + extra - relief,
        )
    } else if WALES.is_match(text) {
        (
            "Wales's LTT",
            "The main residential nil band ends at £225,000. Wales has no separate first-time buyer relief. Qualifying additional properties use a separate set of rates.",
            "the Welsh Revenue Authority's Land Transaction Tax calculator",
            duty(
                price,
                if second && price >= 40_000 {
                    HIGH_LTT
                } else {
                    LTT
                },
                0,
            ),
        )
    } else {
        let mut surcharge = u64::from(NON_RESIDENT.is_match(text)) * 200;
        if second && price >= 40_000 {
            surcharge += 500;
        }
        (
            "SDLT in England and Northern Ireland",
            "The standard nil band ends at £125,000, with 2% on the next £125,000 and 5% on the next band up to £925,000. Eligible first-time buyers pay 0% up to £300,000 and 5% on the rest up to £500,000. Qualifying additional properties generally add 5 percentage points to the standard rates.",
            "HMRC's Stamp Duty Land Tax calculator",
            duty(
                price,
                if first && !second && price <= 500_000 {
                    FIRST_SDLT
                } else {
                    SDLT
                },
                surcharge,
            ),
        )
    };
    if special || price < 20_000 {
        return Cow::Owned(format!(
            "{name}. {details} Use {calculator} for the price, completion date and any reliefs or surcharges."
        ));
    }
    Cow::Owned(format!(
        "Using current residential rates for {name}, the estimate is £{} on £{}. {details} Confirm the completion date and eligibility for any reliefs or surcharges in {calculator}.",
        comma(amount),
        comma(price)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_the_current_residential_bands_for_each_jurisdiction() {
        assert!(reply("stamp duty on a £250,000 house").contains("£2,500"));
        assert!(reply("stamp duty on a £300,000 first home").contains("estimate is £0"));
        assert!(reply("first-time buyer on £200,000 in scotland").contains("£500"));
        assert!(reply("second home for £260,000 in wales").contains("£15,950"));
    }
}
