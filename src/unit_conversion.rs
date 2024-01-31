use std::fmt;
#[derive(Debug)]
struct ConversionError(String);

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ConversionError {}

#[derive(Debug)]
enum Distance {
    Meters,
    Feet,
    Inches,
}
#[derive(Debug)]
enum Time {
    Hours,
    Minutes,
}

#[derive(Debug)]
enum ConversionUnit {
    Distance(Distance),
    Time(Time),
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Distance::Meters => write!(f, "meters",),
            Distance::Feet => write!(f, "feet",),
            Distance::Inches => write!(f, "inches",),
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Time::Hours => write!(f, "hours",),
            Time::Minutes => write!(f, "minutes",),
        }
    }
}

impl fmt::Display for ConversionUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionUnit::Distance(distance) => write!(f, "{}", distance),
            ConversionUnit::Time(time) => write!(f, "{}", time),
        }
    }
}
#[derive(Debug)]
struct ConversionQuery {
    from: ConversionUnit,
    to: ConversionUnit,
    value: f32,
}

impl ConversionQuery {
    fn new(n: f32, from: &str, to: &str) -> Result<Self, ConversionError> {
        let from_unit = match from {
            "m" => ConversionUnit::Distance(Distance::Meters),
            "in" => ConversionUnit::Distance(Distance::Inches),
            "ft" => ConversionUnit::Distance(Distance::Feet),
            "hr" => ConversionUnit::Time(Time::Hours),
            "min" => ConversionUnit::Time(Time::Minutes),
            _ => return Err(ConversionError(format!("Unknown from type: {}", from))),
        };

        let to_unit = match to {
            "m" => ConversionUnit::Distance(Distance::Meters),
            "in" => ConversionUnit::Distance(Distance::Inches),
            "ft" => ConversionUnit::Distance(Distance::Feet),
            "hr" => ConversionUnit::Time(Time::Hours),
            "min" => ConversionUnit::Time(Time::Minutes),
            _ => return Err(ConversionError(format!("Unknown to type: {}", to))),
        };

        match (&from_unit, &to_unit) {
            (ConversionUnit::Time(_), ConversionUnit::Distance(_)) => {
                return Err(ConversionError(format!(
                    "Cannot convert time to distance: unit: {}, from: {}, to: {}",
                    n, from, to
                )));
            }
            (ConversionUnit::Distance(_), ConversionUnit::Time(_)) => {
                return Err(ConversionError(format!(
                    "Cannot convert distance to time: unit: {}, from: {}, to: {}",
                    n, from, to
                )));
            }
            _ => (),
        }

        Ok(Self {
            from: from_unit,
            to: to_unit,
            value: n,
        })
    }

    fn convert_units(&self) -> Result<f32, ConversionError> {
        let res: Result<f32, ConversionError> = match (&self.from, &self.to) {
            (ConversionUnit::Time(Time::Hours), ConversionUnit::Time(Time::Minutes)) => {
                Ok(self.value * 60.)
            }

            (ConversionUnit::Time(Time::Minutes), ConversionUnit::Time(Time::Hours)) => {
                Ok(self.value / 60.)
            }
            (
                ConversionUnit::Distance(Distance::Meters),
                ConversionUnit::Distance(Distance::Feet),
            ) => Ok(self.value * 3.28),
            (
                ConversionUnit::Distance(Distance::Meters),
                ConversionUnit::Distance(Distance::Inches),
            ) => Ok(self.value * 3.28 * 12.),
            (
                ConversionUnit::Distance(Distance::Feet),
                ConversionUnit::Distance(Distance::Inches),
            ) => Ok(self.value * 12.),
            (
                ConversionUnit::Distance(Distance::Feet),
                ConversionUnit::Distance(Distance::Meters),
            ) => Ok(self.value / 3.28),
            (
                ConversionUnit::Distance(Distance::Inches),
                ConversionUnit::Distance(Distance::Feet),
            ) => Ok(self.value / 12.),
            (
                ConversionUnit::Distance(Distance::Inches),
                ConversionUnit::Distance(Distance::Meters),
            ) => Ok(self.value / 12. / 3.28),
            _ => Err(self.get_conversion_error()),
        };
        Ok(res)?
    }

    fn get_conversion_error(&self) -> ConversionError {
        match (&self.from, &self.to) {
            (ConversionUnit::Time(time), ConversionUnit::Distance(_)) => {
                return ConversionError(format!(
                    "Cannot convert time to distance: unit: {}, from: {}, to: {}",
                    self.value, self.from, self.to
                ));
            }
            (ConversionUnit::Distance(distance), ConversionUnit::Time(_)) => {
                return ConversionError(format!(
                    "Cannot convert distance to time: unit: {}, from: {}, to: {}",
                    self.value, self.from, self.to
                ));
            }
            _ => {
                return ConversionError(format!(
                    "Cannot convert for Unkonwn Conversion type: from: {:?}, to: {:?}",
                    self.from, self.to
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success1() {
        let input =
            ConversionQuery::new(2.0, "m", "in").expect("Expected creation of Conversion Query");
        let res: f32 = input
            .convert_units()
            .expect("Expected Unit Conversion 2 meters to inches");
        let output: f32 = 78.72;
        assert_eq!(res, output);
    }
    #[test]
    fn success2() {
        let input =
            ConversionQuery::new(13.0, "in", "m").expect("Expected creation of Conversion Query");
        let res: f32 = input
            .convert_units()
            .expect("Expected Unit Conversion 13.0 inches to meters");
        let output: f32 = 0.330; // roughly
        let precision: f32 = 1000.0; // For 3 decimal places
                                     // Round both values to the same number of decimal places
        let res_rounded = (res * precision).round() / precision;
        let output_rounded = (output * precision).round() / precision;

        assert_eq!(res_rounded, output_rounded);
    }
    #[test]
    fn conversion_query_input_error() {
        let result = ConversionQuery::new(13., "in", "hr");
        match result {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(e) => {
                let expected_error = ConversionError(format!(
                    "Cannot convert distance to time: unit: 13, from: in, to: hr"
                ));
                assert_eq!(e.to_string(), expected_error.to_string());
            }
        }
    }
}
