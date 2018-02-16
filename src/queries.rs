/// Returns a query that shows all databases
pub fn show_databases(host: &str) -> String {
    host.to_string() + "/query?q=show databases"
}

/// Returns a query that shows all tag keys from a database
pub fn show_tags_from_measurement(host: &str, db: &str, measurement: &str) -> String {
    host.to_string() + "/query?db=" + db + "&q=show tag keys from " + measurement
}

/// Returns a query that shows all measurements from a database
pub fn show_measurements(host: &str, db: &str) -> String {
    host.to_string() + "/query?db=" + db + "&q=show measurements"
}

/// Returns a query that shows the whole measurement
pub fn measurement(host: &str, db: &str, measurement: &str) -> String {
    host.to_string() + "/query?db=" + db + "&q=select * from " + measurement
}

/// Returns a query which body is written as a measurement to the db
pub fn write(host: &str, db: &str) -> String {
    host.to_string() + "/write?db=" + db
}

/// Returns a query for creating a db
pub fn create_db(host: &str, db: &str) -> String {
    host.to_string() + "/query?q=CREATE DATABASE " + db
}

/// Returns a query for droppin a db
pub fn drop_db(host: &str, db: &str) -> String {
    host.to_string() + "/query?q=DROP DATABASE " + db
}

/// Returns a query for dropping a measurement
pub fn drop_measurement(host: &str, db: &str, measurement: &str) -> String {
    host.to_string() + "/query?db=" + db + "&q=DROP MEASUREMENT " + measurement
}
