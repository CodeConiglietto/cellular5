use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::Write as FmtWrite,
    fs,
    io::Write as IoWrite,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use failure::{ensure, format_err, Fallible};
use lazy_static::lazy_static;
use mutagen::{Event, EventKind};
use serde::{Deserialize, Serialize};

use crate::util;

type EventCount = HashMap<Cow<'static, str>, usize>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MutagenProfiler {
    generated: EventCount,
    mutated: EventCount,
    updated: EventCount,
}

impl MutagenProfiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Fallible<Self> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Fallible<()> {
        fs::write(path, &serde_json::to_string(&self)?)?;
        Ok(())
    }

    pub fn save_graphs<P: AsRef<Path>>(&self, path: P) -> Fallible<()> {
        let path = path.as_ref();

        fs::create_dir_all(path)?;
        save_graph(&self.generated, "Generated", path.join("generated"))?;
        save_graph(&self.mutated, "Mutated", path.join("mutated"))?;
        save_graph(&self.updated, "Updated", path.join("updated"))?;

        Ok(())
    }

    pub fn default_path() -> PathBuf {
        util::local_path("profile.json")
    }

    pub fn default_graphs_path() -> PathBuf {
        util::local_path("profile_graphs")
    }

    pub fn handle_event(&mut self, event: Event) {
        lazy_static! {
            pub static ref KEY_BLACKLIST: HashSet<&'static str> =
                ["NodeSet", "NodeTree"].iter().copied().collect();
        }

        if !KEY_BLACKLIST.contains(event.key.as_ref()) {
            let data = match event.kind {
                EventKind::Generate => &mut self.generated,
                EventKind::Mutate => &mut self.mutated,
                EventKind::Update => &mut self.updated,
            };

            *data.entry(event.key).or_insert(0) += 1;
        }
    }
}

fn save_graph<P: AsRef<Path>>(data: &EventCount, title: &str, base_path: P) -> Fallible<()> {
    let base_path = base_path.as_ref();
    let output_path = base_path.with_extension("png");

    let mut buf = String::new();

    let mut entries: Vec<_> = data.iter().map(|(k, v)| (k.as_ref(), *v)).collect();
    entries.sort_by_key(|(_, v)| *v);

    writeln!(buf, "reset session")?;

    writeln!(buf, "$Data << EOD")?;
    for (key, value) in entries.iter() {
        writeln!(buf, "\"{}\" {}", key, value)?;
    }
    writeln!(buf, "EOD")?;

    let height = 100 + 20 * data.len();

    writeln!(
        buf,
        "set terminal pngcairo size 1920,{} enhanced font 'Verdana,10'",
        height
    )?;
    writeln!(buf, "set output \"{}\"", output_path.to_string_lossy())?;

    writeln!(buf, "set yrange [0:*] reverse")?;
    writeln!(buf, "set ytics scale 0")?;
    writeln!(buf, "set grid noxtics noytics noztics front")?;
    writeln!(buf, "set style fill solid")?;
    writeln!(buf, "set title \"{}\"", title)?;
    writeln!(buf, "unset key")?;
    writeln!(buf, "myBoxWidth = 0.8")?;
    writeln!(buf, "set offsets 0,0,0.5-myBoxWidth/2.,0.5")?;

    const COLORS: &[&str] = &[
        "#ff0000", // Red
        "#ff7f00", // Orange
        "#ffff00", // Yellow
        "#7fff00", // Chartreuse green
        "#00ff00", // Green
        "#00ff7f", // Spring green
        "#00ffff", // Cyan
        "#007fff", // Azure
        "#0000ff", // Blue
        "#7f00ff", // Violet
        "#ff00ff", // Magenta
        "#ff007f", // Rose
    ];

    for (i, color) in COLORS.iter().enumerate() {
        writeln!(buf, "set linetype {} linecolor rgb \"{}\"", i + 1, color)?;
    }
    writeln!(buf, "set linetype cycle {}", COLORS.len())?;

    // gnuplot black magic to make a horizontal histogram
    writeln!(buf, "plot $Data using 2:0:(0):2:($0-myBoxWidth/2.):($0+myBoxWidth/2.):($0+1):ytic(1) with boxxyerror linecolor variable, $Data using (0):0:2 with labels left")?;

    let gnuplot_check = Command::new("gnuplot").arg("--version").output();
    let gnuplot_version = match gnuplot_check {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).into_owned())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).into_owned())
            }
        }

        Err(e) => Err(e.to_string()),
    };

    match gnuplot_version {
        Ok(version) => {
            println!(
                "Rendering {} with {}",
                output_path.to_string_lossy(),
                version.trim_end(),
            );

            let mut gnuplot = Command::new("gnuplot")
                .current_dir(base_path.parent().unwrap())
                .stdin(Stdio::piped())
                .spawn()?;

            {
                let mut stdin = gnuplot
                    .stdin
                    .take()
                    .ok_or_else(|| format_err!("Failed to get stdin of gnuplot process"))?;

                write!(stdin, "{}", buf)?;
            }

            ensure!(gnuplot.wait()?.success());
        }

        Err(e) => {
            let plt_path = base_path.with_extension("plt");

            println!(
                "Couldn't render with gnuplot: {}, saving to {} instead",
                e,
                plt_path.to_string_lossy(),
            );

            fs::write(&plt_path, buf)?;
        }
    }

    Ok(())
}
