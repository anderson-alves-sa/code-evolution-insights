#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::serve::StaticFiles;

use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::fs::File;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {

    /// The path to the git repository
    #[structopt(short = "p", long = "path")]
    project_path: String,
    /// The date after the analisys should run against (YYYY-MM-DD)
    #[structopt(short = "a", long = "after")]
    after_date: String,
    /// The date before the analisys should run against (YYYY-MM-DD)
    #[structopt(short = "b", long = "before")]
    before_date: String,
}

fn main() {

    let args = Cli::from_args();

    let project_path = &args.project_path;
    let before_date = &args.before_date;
    let after_date = &args.after_date;

    //cloc ./ --by-file --csv --quiet --report-file=classi_lines.csv

    let lines_report_file = "static/csvs/lines.csv"; //&[project_prefix, "lines.csv"].concat();
    let lines_file = File::create(lines_report_file).expect("couldn't create evolution report file");
    
    let lines_report = Command::new("cloc")
        .current_dir(project_path)
        .arg("./")
        .arg("--by-file")
        .arg("--csv")
        .arg("--quiet")
        .stdout(unsafe { Stdio::from_raw_fd(lines_file.into_raw_fd()) })
        .output()
        .expect("failed to execute process");

    println!("code lines report status: {}", lines_report.status);

    //git log --pretty=format:'[%h] %an %ad %s' --date=short --numstat --after=2019-01-01 > classi_evo.log

    let evolution_report_file = "static/csvs/evolution.csv"; //&[project_prefix, "evolution.csv"].concat();
    let evolution_file = File::create(evolution_report_file).expect("couldn't create evolution report file");
    
    let repository_git_path = [project_path, "/.git"].concat();

    let git_dir_option = &["--git-dir=", &repository_git_path].concat();
    let after_option = &["--after=", after_date].concat();
    let before_option = &["--before=", before_date].concat();

    let evolution_report = Command::new("git")
        .arg(git_dir_option)
        .arg("log")
        .arg("--pretty=format:[%h] %an %ad %s")
        .arg("--date=short")
        .arg("--numstat")
        .arg(after_option)
        .arg(before_option)
        .stdout(unsafe { Stdio::from_raw_fd(evolution_file.into_raw_fd()) })
        .output()
        .expect("failed to execute process");

   // println!("gitDir: {}, afterOption: {}, evolutionReportFile: {}", git_dir_option, after_option, evolution_report_file);
    println!("evolution report status: {}", evolution_report.status);
    
    //maat -l classi_evo.log -c git -a revisions > classi_freqs.csv

    let frequencies_report_file = "static/csvs/frequencies.csv"; //&[project_prefix, "frequencies.csv"].concat();
    let frequencies_file = File::create(frequencies_report_file).expect("couldn't create evolution report file");

    let frequencies_report = Command::new("maat")
        .arg("-l")
        .arg(evolution_report_file)
        .arg("-c")
        .arg("git")
        .arg("-a")
        .arg("revisions")
        .stdout(unsafe { Stdio::from_raw_fd(frequencies_file.into_raw_fd()) })
        .output()
        .expect("failed to execute process");

    println!("frequencies report status: {}", frequencies_report.status);
    println!("frequencies report status: {:#?}", frequencies_report.stdout);

    //python merge_comp_freqs.py classi_freqs.csv classi_lines.csv > merged_freqs_lines.csv

    let merged_frequencies_lines_report_file = "static/csvs/merged_frequencies_lines.csv"; //&[project_prefix, "merged_frequencies_lines.csv"].concat();
    let merged_frequencies_lines_file = File::create(merged_frequencies_lines_report_file).expect("couldn't create nerged report file");

    let merged_frequencies_lines_report = Command::new("python")
        .arg("static/py_scripts/merge_comp_freqs.py")
        .arg(frequencies_report_file)
        .arg(lines_report_file)
        .stdout(unsafe { Stdio::from_raw_fd(merged_frequencies_lines_file.into_raw_fd()) })
        .output()
        .expect("failed to execute process");

      io::stdout().write_all(&merged_frequencies_lines_report.stdout).unwrap();
      io::stderr().write_all(&merged_frequencies_lines_report.stderr).unwrap();

    println!("merged frequencies lines report status: {}", merged_frequencies_lines_report.status);
    println!("stdout: {:#?}", merged_frequencies_lines_report.stdout);

    //python ../../code_maat_inst/py_scripts/scripts/csv_as_enclosure_json.py --structure=classi_lines.csv --weights=classi_freqs

    let csv_as_enclosure_json_report_file = "static/public/csv_as_enclosure_json.json";
    let csv_as_enclosure_json_file = File::create(csv_as_enclosure_json_report_file).expect("couldn't create nerged report file");

    let csv_as_enclosure_json_report = Command::new("python")
        .arg("static/py_scripts/csv_as_enclosure_json.py")
        .arg("--weights")
        .arg(frequencies_report_file)
        .arg("--structure")
        .arg(lines_report_file)
        .stdout(unsafe { Stdio::from_raw_fd(csv_as_enclosure_json_file.into_raw_fd()) })
        .output()
        .expect("failed to execute process");

    println!("merged frequencies lines report status: {}", csv_as_enclosure_json_report.status);
    println!("stdout: {:#?}", csv_as_enclosure_json_report.stdout);
    
    rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static/public")))
        .launch();
    
}
