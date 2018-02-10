mod http_client;

///
pub fn tag_keys(url: String, db: StringString) -> Vec<String> {
  String::from(url + "/query?db=" + db + "&q=show tag keys" )
}

pub fn tag_keys_measurement(url: String, db: String, measurement: String) -> Vec<String> {
  String::from(url + "/query?db=" + db + "&q=show tag keys from " + measurement)
}