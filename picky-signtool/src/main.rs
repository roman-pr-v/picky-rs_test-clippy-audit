use std::path::{Path, PathBuf};

use anyhow::bail;
use walkdir::{DirEntry, WalkDir};

use picky_signtool::config::*;
use picky_signtool::lief_logging;
use picky_signtool::sign::sign;
use picky_signtool::verify::verify;

fn main() -> anyhow::Result<()> {
    let matches = config();

    let files_to_process: Vec<PathBuf> = match (matches.is_present(ARG_BINARY), matches.is_present(ARG_PS_SCRIPT)) {
        (true, false) => {
            let binary_path = matches
                .value_of(ARG_INPUT)
                .expect("Path to a Windows executable is required");

            lief_logging(matches.value_of(ARG_LOGGING));

            vec![PathBuf::from(binary_path)]
        }
        (false, true) => {
            let folder = matches
                .value_of(ARG_SCRIPTS_PATH)
                .expect("The PowerShell file path was not specified");

            if Path::new(folder).is_file() {
                vec![PathBuf::from(folder)]
            } else {
                let is_ps_file = |entry: &DirEntry| -> bool {
                    entry
                        .path()
                        .extension()
                        .map(|ext| {
                            ext.to_str()
                                .map(|ext| matches!(ext, "ps1" | "psm1" | "psd1"))
                                .unwrap_or(false)
                        })
                        .unwrap_or(false)
                };

                WalkDir::new(PathBuf::from(folder))
                    .contents_first(true)
                    .into_iter()
                    .filter_map(|entry| {
                        entry.ok().and_then(|entry| {
                            if is_ps_file(&entry) {
                                Some(entry.into_path())
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<Vec<PathBuf>>()
            }
        }
        (true, true) => bail!("Do not know what to process exactly(`binary` and `script` both are specified)"),
        (false, false) => bail!("Do not know what to process(`binary` or `script` is not specified)"),
    };

    if matches.is_present(ARG_SIGN) && !files_to_process.is_empty() {
        if let (Some(certfile), Some(private_key)) = (matches.value_of(ARG_CERTFILE), matches.value_of(ARG_PRIVATE_KEY))
        {
            return sign(
                &matches,
                PathBuf::from(certfile),
                PathBuf::from(private_key),
                &files_to_process,
            );
        }
    }

    if files_to_process.is_empty() {
        bail!("Nothing to do")
    }

    if matches.is_present(ARG_VERIFY) {
        verify(&matches, &files_to_process)?;
    }

    Ok(())
}
