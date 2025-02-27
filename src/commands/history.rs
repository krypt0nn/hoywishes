use std::path::PathBuf;

use clap::Args;

use crate::url::parse_wishes_urls;

#[derive(Args)]
pub struct HistoryArgs {
    #[arg(short, long)]
    /// Path to the game installation
    pub game_path: PathBuf,

    #[arg(short, long, default_value_t = false)]
    /// Return URLs in reversed order (from oldest to recent)
    pub reverse_order: bool,

    #[arg(short, long, default_value_t = false)]
    /// Open first URL in the returning list
    /// 
    /// If reversed order enabled, then the oldest URL will be opened.
    /// Otherwise the most recent one
    pub open_first_url: bool,

    #[arg(short, long, default_value_t = 3)]
    /// Maximal number of URLs to return
    pub max_return_num: usize
}

impl HistoryArgs {
    pub fn execute(&self) -> anyhow::Result<()> {
        if !self.game_path.exists() {
            anyhow::bail!("Given game path doesn't exist");
        }

        // Iterate over game installation files and folders
        for entry in self.game_path.read_dir()?.flatten() {
            // GenshinImpact[_Data]
            // StarRail[_Data]
            // etc.
            if entry.file_name().to_string_lossy().ends_with("_Data") {
                let data_path = entry.path().join("webCaches/Cache/Cache_Data/data_2");

                if data_path.exists() {
                    println!("[#] Data file: {}", data_path.to_string_lossy());

                    match parse_wishes_urls(data_path) {
                        Ok(urls) if urls.is_empty() => {
                            eprintln!("No wishes URL found");
                        }

                        Ok(mut urls) => {
                            // Reverse found urls vector if needed
                            if self.reverse_order {
                                urls = urls.into_iter().rev().collect();
                            }

                            // Limit returning urls
                            urls = urls[..self.max_return_num].to_vec();

                            // Open the first found URL
                            if self.open_first_url {
                                open::that(&urls[0])?;
                            }

                            // And print found URLs
                            for url in urls {
                                // TODO: it's possible to print here banner type and some other metadata, and don't print links with outdated API keys as well
                                println!("    - {url}");

                                // Open found url if required
                                // if cli.open_url {
                                //     open::that(url)?;
                                // }
                            }
                        }

                        Err(err) => eprintln!("Failed to parse wishes URLs: {err}")
                    }

                    // One empty line to split series
                    println!();
                }
            }
        }

        Ok(())
    }
}
