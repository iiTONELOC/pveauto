use pveauto::auto_installer::{
    commands::command_parser::parse_command_from,
    dispatcher::{dispatch, execute::execute},
};

pub async fn run() -> Result<(), ()> {
    run_from(std::env::args().skip(1)).await
}
pub async fn run_from<I>(args: I) -> Result<(), ()>
where
    I: Iterator<Item = String>,
{
    execute(dispatch(parse_command_from(args).map_err(|_| ())?)).await;
    Ok(())
}
#[tokio::main]
async fn main() {
    run().await.unwrap_or_else(|_| std::process::exit(1));
}
