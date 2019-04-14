use num_traits::Float;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use serde_derive::Serialize;

use std::convert::TryFrom;
use std::num::ParseFloatError;
use std::str::FromStr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(transparent)]
pub(crate) struct PositiveFinite<F: Float> {
    // Constraint:
    // - `inner.is_sign_positive() && inner.is_finite()`
    inner: F,
}

impl<F: Float> PositiveFinite<F> {
    fn try_new(value: F) -> std::result::Result<Self, &'static str> {
        if value.is_sign_positive() && value.is_finite() {
            Ok(Self { inner: value })
        } else {
            Err("the value must be positive and finite")
        }
    }
}

impl<F: Float + FromStr<Err = ParseFloatError>> FromStr for PositiveFinite<F> {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, String> {
        let value = s.parse::<F>().map_err(|e| e.to_string())?;
        Self::try_new(value).map_err(Into::into)
    }
}

impl<'de, F: Float + DeserializeOwned> Deserialize<'de> for PositiveFinite<F> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let value = F::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<f64> for PositiveFinite<f64> {
    type Error = &'static str;

    fn try_from(value: f64) -> std::result::Result<Self, &'static str> {
        Self::try_new(value)
    }
}

impl From<PositiveFinite<f64>> for f64 {
    fn from(from: PositiveFinite<f64>) -> Self {
        from.inner
    }
}

#[cfg(test)]
mod tests {
    use crate::util::num::PositiveFinite;

    use std::convert::TryFrom as _;
    use std::f64;

    #[test]
    fn test_positive_finite() -> std::result::Result<(), String> {
        fn assert_f64_eq(a: f64, b: f64) {
            assert!((a - b).abs() < f64::EPSILON);
        }

        PositiveFinite::try_from(0.0).map_err(ToOwned::to_owned)?;
        PositiveFinite::try_from(1.0).map_err(ToOwned::to_owned)?;
        PositiveFinite::try_from(1.175_494_2e-38).map_err(ToOwned::to_owned)?;
        PositiveFinite::try_from(-0.0).unwrap_err();
        PositiveFinite::try_from(f64::INFINITY).unwrap_err();
        PositiveFinite::try_from(f64::NAN).unwrap_err();
        "0.0".parse::<PositiveFinite<f64>>()?;
        assert_f64_eq(
            f64::from(PositiveFinite::try_from(42.0).map_err(ToOwned::to_owned)?),
            42.0,
        );
        Ok(())
    }
}
