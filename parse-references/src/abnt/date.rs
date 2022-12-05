use std::fmt::Display;

pub struct Date<'d>(pub &'d str);

impl<'d> Display for Date<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn lookup_month(n: u8) -> Option<&'static str> {
            const MONTHS: [&str; 12] = [
                "jan", "fev", "mar", "abr", "mai", "jun", "jul", "ago", "set", "out", "nov", "dez",
            ];

            if n == 0 || n > 12 {
                None
            } else {
                Some(MONTHS[(n - 1) as usize])
            }
        }

        let parts: Vec<_> = self.0.split("-").collect();

        let year = parts.get(0);
        let month: Option<&str> = parts
            .get(1)
            .map(|s| s.parse::<u8>().ok())
            .flatten()
            .map(lookup_month)
            .flatten();
        let day: Option<u8> = parts.get(2).map(|s| s.parse().ok()).flatten();

        if let (Some(year), Some(month), Some(day)) = (year, month, day) {
            write!(f, "{} {}. {}", day, month, year)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
