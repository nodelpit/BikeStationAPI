use serde::Serialize;

#[derive(Serialize)]
pub struct Station {
    pub id: StationId,
    pub name: String,
    pub total_docks: u32,
    pub available_bikes: u32,
    pub free_docks: u32,
}

#[derive(Serialize)]
pub struct StationId(pub u32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_station() {
        let rust_data = Station {
            id: StationId(0),
            name: "docks-1".to_string(),
            total_docks: 100,
            available_bikes: 800,
            free_docks: 20,
        };

        let to_json = serde_json::to_string(&rust_data).unwrap();

        let json_value: serde_json::Value = serde_json::from_str(&to_json).unwrap();

        assert_eq!(
            json_value,
            serde_json::json!({
                "id": 0,
                "name": "docks-1",
                "total_docks": 100,
                "available_bikes": 800,
                "free_docks": 20,
            })
        );
    }
}
