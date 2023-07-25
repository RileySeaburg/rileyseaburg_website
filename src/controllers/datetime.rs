pub mod datetime {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = match *date {
            Some(ref d) => d.format(FORMAT).to_string(),
            None => "".to_string(),
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<NaiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map(Some).map_err(serde::de::Error::custom)
    }
}
