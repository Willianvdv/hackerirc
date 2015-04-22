extern crate rustc_serialize;
use rustc_serialize::json::Json;

struct Report {
    url: String,
    title: String
}

impl Report {}

fn create_report_from_json(report_json: Json) -> Report {
    return Report { title: report_json["title"].to_string(),
                    url: report_json["url"].to_string() };
}

fn get_public_reports() -> Vec<Report> {
    let data = Json::from_str("{\"title\": \"Some report\", \"url\": \"http://example.com\"}").unwrap();
    let report: Report = create_report_from_json(data);
    let reports: Vec<Report> = vec![report];

    return reports;
}

fn main() {
    get_public_reports();
}
