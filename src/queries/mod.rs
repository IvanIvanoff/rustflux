pub fn show_databases(host: &str) -> String {
  host.to_string() + "/query?q=show databases"
}

///
pub fn tag_keys(host: &str, db: &str) -> String {
  host.to_string() + "/query?db=" + db + "&q=show tag keys"
}

pub fn tag_keys_measurement(host: &str, db: &str, measurement: &str) -> String {
  host.to_string() + "/query?db=" + db + "&q=show tag keys from " + measurement
}