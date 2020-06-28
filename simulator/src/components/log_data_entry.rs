///used by simumo derive to write custom log
/// todo :: it should be placed in simumo derive
///  it is simpler for quick fix to put it there but its an abomination of design
#[derive(Serialize)]
pub struct LogDataEntry<T> {
    #[serde(rename = "type")]
    typename: String,
    resolution: Option<String>,
    value: T,
}

impl<T> LogDataEntry<T> {
    pub fn new(typename: String, resolution: Option<String>, value: T) -> Self {
        Self {
            typename,
            resolution,
            value,
        }
    }
}
