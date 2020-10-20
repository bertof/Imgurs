//! Custom Serde serialization and deserialization implementations

/// Unix epoch serialization and deserialization
pub(crate) mod unix_epoch {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%s";

    pub(crate) fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_i64(date.timestamp())
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where D: Deserializer<'de> {
        let v = i64::deserialize(deserializer)?;
        Utc.datetime_from_str(&format!("{}", v), FORMAT)
            .map_err(serde::de::Error::custom)
    }
}


#[cfg(test)]
mod test {
    use std::error::Error;

    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use crate::serialization::unix_epoch;

    #[test]
    fn test_serialization_deserialization_timestamp() -> Result<(), Box<dyn Error>> {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        struct TestDate(#[serde(with = "unix_epoch")] DateTime<Utc>);

        let now = TestDate(Utc::now());

        let serialized = serde_json::to_string_pretty(&now)?;
        println!("{:?} => {}", &now, &serialized);

        let deserialized = serde_json::from_str::<TestDate>(&serialized)?;
        println!("{} => {:?}", &serialized, &deserialized);

        assert_eq!(now.0.timestamp(), deserialized.0.timestamp());
        Ok(())
    }
}