// use std::env;
use std::fs;
use std::process::Command;

fn main() {

    let debug = false;


    let path = "./contracts/examples/digital-cash/scenarios/withdraw-multi-kda.scen.json";

    let mut projects_paths:Vec<String> = vec![];
    if path.ends_with("scen.json") {
        projects_paths.push(path.to_string());
    } else {
        projects_paths = scan_dir(path, 10);
    }

    let operator_success = run_scenarios_operator(projects_paths.clone());

    println!("operator_success: {}", operator_success);
    println!("Total projects: {}", projects_paths.len());
}

fn run_scenarios_operator(projects_paths: Vec<String>) -> i32 {
    let project_count = projects_paths.len();
    let mut failed_projects = 0;
    let mut ran_projects = 0;

    for project_path in projects_paths {
        ran_projects += 1;
        println!("Running project {}...", project_path);
        let mut cmd = Command::new("go");
        let output =
            cmd.stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .current_dir("../klever-go")
                .arg("run")
                .arg("./cmd/operator/")
                .arg("--key-file=./localWalletKey.pem")
                .arg("sc")
                .arg("run-scenarios")
                .arg("--path=".to_owned() + &project_path)
                .output()
                .expect("failed to execute process");

        if output.status.success() {
            println!("Running project {}...success", project_path);
        } else {
            println!("Running project {}...failed", project_path);
            failed_projects += 1;
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            break
        }
        // break
        println!("Ran {} projects of {}, {} failed", ran_projects, project_count, failed_projects);
    }
    return ran_projects - failed_projects;
}

fn scan_dir(path: &str, depth: i32) -> Vec<String> {
    let mut folders_with_scenarios:Vec<String> = vec![];
    if depth < 0 {
        return folders_with_scenarios;
    }

    let paths = fs::read_dir(path).expect("not a valid path or directory");
    for path in paths {
        let path_buff = path.unwrap().path();
        if fs::metadata(&path_buff).unwrap().is_dir() {
            if path_buff.to_str().unwrap().ends_with("scenarios") {
                println!("Dir: {}", fs::canonicalize(&path_buff).unwrap().display());
                folders_with_scenarios.push(fs::canonicalize(&path_buff).unwrap().display().to_string());
            }

            let mut sub_folders = scan_dir(&path_buff.to_str().unwrap(), depth - 1);
            folders_with_scenarios.append(&mut sub_folders);
        }
    }
    return folders_with_scenarios;
}
