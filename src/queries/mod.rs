///
pub fn show_databases(host: &str) -> String {
    host.to_string() + "/query?q=show databases"
}

///
pub fn show_tags(host: &str, db: &str) -> String {
    host.to_string() + "/query?db=" + db + "&q=show tag keys"
}

///
pub fn show_tags_from_measurement(host: &str, db: &str, measurement: &str) -> String {
    host.to_string() + "/query?db=" + db + "&q=show tag keys from " + measurement
}

///
pub fn show_measurements(host: &str, db: &str) -> String {
    host.to_string() + "/query?db=" + db + "&q=show measurements"
}

///
pub fn measurement(host: &str, db: &str, measurement: &str) -> String {
    host.to_string() + "/query?db=" + db + "&q=select * from " + measurement
}

pub fn write(host: &str, db: &str) -> String {
    host.to_string() + "/write?db=" + db
}
