mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod repeat;
use crate::repeat::repchar;
mod specific;
use crate::specific::specific;
use vornix_banner::{Banner, BuiltinFont, Style, rgb};
mod misa;
use crate::misa::cagrep;
mod regex;
use crate::regex::analyze;

/*
Gaurav Sablok,
gsablok@proton.me
*/

fn main() {
    let style = Style::new().fg(rgb(255, 100, 20)).bold();

    let mut banner = Banner::new("repchar")
        .with_builtin_font(BuiltinFont::Block)
        .with_style(style)
        .centered(true);

    banner.display().unwrap();
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::Window {
            filepath,
            repeat,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = repchar(filepath, repeat).unwrap();
                println!("The command has been finished:{}", command);
            });
        }
        Commands::SpecificWindow {
            filepath,
            repeat,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = specific(filepath, repeat).unwrap();
                println!("The command has been finished:{}", command);
            });
        }
        Commands::Table {
            filepath,
            repeat,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .expect("threads failed");
            pool.install(|| {
                let command = cagrep(filepath, repeat).unwrap();
                println!("The command has finished:{}", command);
            })
        }
        Commands::Count {
            filepath,
            nucleotide,
            count,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(4usize)
                .build()
                .expect("threads failed");
            pool.install(|| {
                let command = analyze(filepath, nucleotide, count).unwrap();
                println!("The command has finished: {}", command);
            })
        }
    }
}
