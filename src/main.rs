
use std::process::Command;
use std::env;
use std::path::Path;
extern crate pbr;
use pbr::ProgressBar;
use std::thread;
use std::time::Duration;

// using vector
// fn command(c: &str, args: &str) {
//     let strings: Vec<&str> = args.split(" ").collect();
//     println!("{:?}", strings);
//     let output = Command::new(c)
//             .args(strings)
//             .output()
//             .expect("Command not found");  
//             println!("{}", String::from_utf8_lossy(&output.stdout));
// }

fn command(c: String) -> Vec<u8> {
    let output = Command::new("sh")
            .arg("-c")
            .arg(c.trim())
            .output()
            .expect("Command not found");
            // println!("{}", String::from_utf8_lossy(&output.stdout).trim());
            return output.stdout
        
}

fn help() -> String {
    let s = String::from(
    "
    Simplify switch kubernetes context!

    Commands Available:
    get         show list contexts config kubernetes
    merge       merge kubeconfig with existing kubeconfig in $HOME/.kube/config

    Usage:
    connect get 
    connect <type string connection>
    connect merge <type fullpath file kubeconfig>

    Notes:
    <string connection> defined using 'grep' so you can type similar name context.
    <type full path file kubeconfig> when using merge, automatic will backup existing kubeconfig to $HOME/.kube/config.backup
    ");
    s
    
}

fn bar (c: u64) {
    let count = c;
    let mut pb = ProgressBar::new(count);
    pb.format("=.");
    for _ in 0..count {
        pb.inc();
        thread::sleep(Duration::from_millis(1000));
    }
    return pb.finish_print("- done");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{}", help());
    }else {
        let query= &args[1];
        if query == "get" {
            // command("kubectl","config get-contexts"); using vector
            let p = command("kubectl config get-contexts".to_string());
            println!("{}", String::from_utf8_lossy(&p).trim());
        
        } else if query == "merge" && args.len() == 3 {
            // let mut rs:bool=true;
            let rs = Path::new(&args[2].trim()).is_file();
            if rs == true{
                let backup = command("echo '- backup process.' && cp $HOME/.kube/config $HOME/.kube/config.backup".to_string());
                println!("{}", String::from_utf8_lossy(&backup).trim());
                bar(1);
                // println!("\n");
                let merge = command("echo '- add new kubeconfig..' &&
                                            export KUBECONFIG=$HOME/.kube/config:".to_string()+&args[2].trim()+ " &&
                                            echo '- process merge..' &&
                                            kubectl config view --flatten > /tmp/config &&
                                            mv /tmp/config $HOME/.kube/config");
                println!("{}", String::from_utf8_lossy(&merge).trim());
                bar(2);
            }
            else{
                println!("{}", help())
            }
        } else if query == "--help" || query == "help" || query == "merge" {
            println!("{}", help());

        }else{
            let c = command(format!("{}{}{}","kubectl config get-contexts | grep ",query," | awk '{print $2}'"));
            let f = command(format!("{}{}","kubectl config use-context ",String::from_utf8_lossy(&c).trim()));
            println!("{}",String::from_utf8_lossy(&f).trim())
        }
    }
}
