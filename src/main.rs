struct Report {
    url: String,
    title: String
}

impl Report {}

fn get_public_reports() -> Vec<Report> {
    let report: Report = Report { url: "example.com".to_string(), 
                                  title: "Some report".to_string() };

    let reports: Vec<Report> = vec![report];
    return reports;
}

fn main() {
    get_public_reports();
}
