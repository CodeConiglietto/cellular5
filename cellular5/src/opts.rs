use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opts {
    /// A number to seed the rng with
    #[structopt(long)]
    pub seed: Option<u128>,
}
