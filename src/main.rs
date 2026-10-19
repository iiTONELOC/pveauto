use pveauto::auto_installer::{
    commands::{command_parser::parse_command_from, constants::USAGE},
    dispatcher::{dispatch, execute::execute},
};

const EXIT_OK: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const EXIT_USAGE: i32 = 2;

pub async fn run_from<I>(args: I) -> i32
where
    I: Iterator<Item = String>,
{
    let command = match parse_command_from(args) {
        Ok(command) => command,
        Err(e) => {
            eprintln!("{e}\n{USAGE}");
            return EXIT_USAGE;
        }
    };
    match execute(dispatch(command)).await {
        Ok(()) => EXIT_OK,
        Err(e) => {
            eprintln!("error: {e}");
            EXIT_FAILURE
        }
    }
}

#[tokio::main]
async fn main() {
    std::process::exit(run_from(std::env::args().skip(1)).await);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> impl Iterator<Item = String> {
        list.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }

    #[tokio::test]
    async fn exit_code_follows_the_outcome() {
        let cases: [(&[&str], i32); 6] = [
            (&["bogus"], EXIT_USAGE),
            (&[], EXIT_USAGE),
            (&["serve-answers"], EXIT_FAILURE),
            (&["offline-installer"], EXIT_FAILURE),
            (&["--help"], EXIT_OK),
            (&["exit"], EXIT_OK),
        ];
        for (argv, expected) in cases {
            assert_eq!(run_from(args(argv)).await, expected, "{argv:?}");
        }
    }
}
