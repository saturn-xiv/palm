use clap::Clap;

use super::Result;

#[derive(Clap)]
pub enum Gpio {
    #[clap(about = "Led")]
    Led(Led),
    #[clap(about = "Button")]
    Button(Button),
}

impl Gpio {
    pub fn launch(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clap)]
pub struct Led {
    #[clap(short, long)]
    pub pin: u8,
}

#[derive(Clap)]
pub struct Button {
    #[clap(long)]
    pub pin: u8,
    #[clap(long)]
    pub port: u16,
}
