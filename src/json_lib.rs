use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    project_name: String,
    strict_mode: bool,
    branches: Vec<BranchConf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BranchConf {
    name: String,
    build_cmd: String,
    deployments: Vec<DeployConf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeployConf {
    pool: String,
    cmd_args: String,
}

#[ignore]
#[test]
fn test_json_lib() {
    let c = Config {
        project_name: "pivot-maintenance".to_string(),
        strict_mode: true,
        branches: vec![
            BranchConf {
                name: "develop".to_string(),
                build_cmd: "mvn build".to_string(),
                deployments: vec![
                    DeployConf {
                        pool: "na-1y".to_string(),
                        cmd_args: "something for 1y".to_string(),
                    },
                    DeployConf {
                        pool: "na-2y".to_string(),
                        cmd_args: "something for 2y".to_string(),
                    },
                ],
            },
            BranchConf {
                name: "master".to_string(),
                build_cmd: "mvn build".to_string(),
                deployments: vec![
                    DeployConf {
                        pool: "na-3y".to_string(),
                        cmd_args: "something for 3y".to_string(),
                    },
                    DeployConf {
                        pool: "na-4y".to_string(),
                        cmd_args: "something for 4y".to_string(),
                    },
                ],
            },
        ],
    };
    let mut buf = vec![];
    let mut ser = serde_json::Serializer::new(&mut buf);
    c.serialize(&mut ser).expect("Failed to serialize");

    // let json = serde_json::to_string(&c).unwrap();
    let json = String::from_utf8(buf).unwrap();
    println!("Output json is {}", json);
    let cdup: Config = serde_json::from_str(json.as_str()).unwrap();
    println!("deserialized object is {:?}", cdup);
}
