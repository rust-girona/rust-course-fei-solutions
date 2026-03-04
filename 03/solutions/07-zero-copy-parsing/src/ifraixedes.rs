//! Run this file with `cargo test --test 06_zerocopy_parsing`.

// TODO: Write a BLAZINGLY FAST, high-performance zero-copy parser of... *checks notes* cinema tickets?
//
// Implement a function `parse_ticket` that receives a string slice and returns either
// a parsed ticket (if the parsing went fine) or `None` (if the ticket could not have been parsed).
//
// The parser should be "zero copy", i.e. it should not copy data out of the string, but rather
// represent the parsed items with references into the original input string.
// Apart from that, the parser can perform allocations when it is executed :)
//
// A ticket contains the following three things:
// - The name of a movie.
//   - The name has to consist only of lower/uppercase letters (a-zA-Z), digits (0-9) and spaces.
//   - The name is required.
// - The day of the week when the movie was broadcasted in the cinema.
//   - The day is represented by a literal string, "monday", "tuesday", "wednesday", "thursday", "friday",
//   "satuday" or "sunday". The lower/upper case of the individual characters does not matter.
//   - The day is required.
// - The name of the visitor.
//   - The name has the same character requirements as the movie (a-zA-Z0-9 ).
//   - The name is optional, the ticket can be anonymous.
//
// The format of the ticket is `<movie-name>;<day>;<visitor-name>`. The second semicolon is optional
// when the visitor name is missing. There must not be any trailing data in the input string.

// Below you can find a set of unit tests.

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl std::convert::TryFrom<&str> for Day {
    type Error = String;

    fn try_from(s: &str) -> Result<Day, Self::Error> {
        match s.to_ascii_lowercase().as_str() {
            "monday" => Ok(Day::Monday),
            "tuesday" => Ok(Day::Tuesday),
            "wednesday" => Ok(Day::Wednesday),
            "thursday" => Ok(Day::Thursday),
            "friday" => Ok(Day::Friday),
            "saturday" => Ok(Day::Saturday),
            "sunday" => Ok(Day::Sunday),
            _ => Err(String::from("invalid week day name")),
        }
    }
}

struct Ticket<'a> {
    movie: &'a str,
    day: Day,
    visitor: Option<&'a str>,
}

// The following signature also compiles, however, there is a clippy warning about "hiding a
// lifetime that's elided elsewhere is confusing".
// fn parse_ticket(raw_ticket: &str) -> Option<Ticket>
//
// Then I changed the signature function to
// fn parse_ticket<'a>(raw_ticket: &'a str) -> Option<Ticket<'a>>
//
// But it's better to elide the lifetime and use the anonymous lifetime for the Ticket to avoid the
// hiding warning.
fn parse_ticket(raw_ticket: &str) -> Option<Ticket<'_>> {
    // REVIEW: here it's better to use splitn to 3, so we don't compute more parsing after the
    // first 2 semicolons.
    let parts: Vec<&str> = raw_ticket.split(';').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return None;
    }

    if parts.len() == 2 {
        Some(Ticket {
            movie: check_name(parts[0]).ok()?,
            day: Day::try_from(parts[1]).ok()?,
            visitor: None,
        })
    } else {
        let visitor = if parts[2].is_empty() {
            None
        } else {
            Some(check_name(parts[2]).ok()?)
        };

        Some(Ticket {
            movie: check_name(parts[0]).ok()?,
            day: Day::try_from(parts[1]).ok()?,
            visitor,
        })
    }
}

fn check_name(n: &str) -> Result<&str, ()> {
    for c in n.chars() {
        if !c.is_ascii_alphanumeric() && c != ' ' {
            return Err(());
        }
    }

    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::{parse_ticket, Day};

    #[test]
    fn empty() {
        assert!(parse_ticket("").is_none());
    }

    #[test]
    fn valid_ticket() {
        let ticket = parse_ticket("Titanic;Monday;Mark Rousskov").expect("ticket expected");
        assert!(matches!(ticket.movie, "Titanic"));
        assert!(matches!(ticket.day, Day::Monday));
        assert!(matches!(ticket.visitor, Some("Mark Rousskov")));
    }

    #[test]
    fn anonymous_visitor() {
        let ticket = parse_ticket("Armageddon;Tuesday;").expect("ticket expected");
        assert!(matches!(ticket.movie, "Armageddon"));
        assert!(matches!(ticket.day, Day::Tuesday));
        assert!(ticket.visitor.is_none());
    }

    #[test]
    fn anonymous_visitor_no_semicolon() {
        let ticket = parse_ticket("The Dark Knight;Saturday").expect("ticket expected");
        assert!(matches!(ticket.movie, "The Dark Knight"));
        assert!(matches!(ticket.day, Day::Saturday));
        assert!(ticket.visitor.is_none());
    }

    #[test]
    fn invalid_movie_name() {
        assert!(parse_ticket("Scott Pilgrim Vs. The World;Monday;Theresa June").is_none());
    }

    #[test]
    fn invalid_visitor_name() {
        assert!(parse_ticket("Groundhog Day;Friday;Adéla Nováková").is_none());
    }

    #[test]
    fn empty_day() {
        assert!(parse_ticket("The Terminator;;Jules Verne").is_none());
    }

    #[test]
    fn missing_day() {
        assert!(parse_ticket("Black Panther;Martina Novakova").is_none());
    }

    #[test]
    fn invalid_day() {
        assert!(parse_ticket("Shaun Of The Dead;yesterday;Martin Scorcese").is_none());
    }

    #[test]
    fn weird_day_case() {
        let ticket = parse_ticket("Psycho;wedNEsDAy;Gareth Bail").expect("ticket expected");
        assert!(matches!(ticket.movie, "Psycho"));
        assert!(matches!(ticket.day, Day::Wednesday));
        assert!(matches!(ticket.visitor, Some("Gareth Bail")));
    }

    #[test]
    fn trailing_data() {
        assert!(parse_ticket(
            "Eternal Sunshine Of The Spotless Mind;sunday;Arnold Schwarzenegger ;00"
        )
        .is_none());
    }

    #[test]
    fn nonstatic_lifetime() {
        // Just to make sure that `parse_ticket` doesn't cheat by using a 'static lifetime :)
        let input = String::from("Batman;wednesDAY;Julia Roberts");
        let ticket = parse_ticket(&input).expect("ticket expected");
        assert!(matches!(ticket.movie, "Batman"));
        assert!(matches!(ticket.day, Day::Wednesday));
        assert!(matches!(ticket.visitor, Some("Julia Roberts")));
    }
}
