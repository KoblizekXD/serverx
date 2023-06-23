// https://maven.minecraftforge.net/net/minecraftforge/forge/<version>/forge-<version>-installer.jar
// https://maven.fabricmc.net/net/fabricmc/fabric-installer/0.11.2/fabric-installer-0.11.2.jar

extern crate clap;

use clap::Parser;

enum ModLoader {
    None,
    Forge,
    Fabric
}

#[derive(Parser, Debug)]
#[command(author = "KoblizekXD", about = "Minecraft Server creation tool", long_about = None)]
struct Args {
    #[arg(short = 'v', long = "version")]
    version: String,
    #[arg(short = 'l', long = "modloader", default_value = "none")]
    loader: String
}

fn main() {
    let _args = Args::parse();
}
