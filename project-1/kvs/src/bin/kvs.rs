use clap::clap_app;

use kvs::KvStore;

fn main() {
    let _matchs = clap_app!(kvs =>
        (version:env!("CARGO_PKG_VERSION"))
        (author:"l1nxy <l1nxy.zy@gmail.com>")
        (about:"this is my talentlan project 1")
        (@setting ArgRequiredElseHelp)
        (@subcommand set =>
            (about:"Set key value and store in memory")
            (@arg keyvalue: +required ... "set key and value")
        )
        (@subcommand get =>
            (about:"Get value by a key")
            (@arg key:+required ... "Get key")
        )
        (@subcommand rm =>
          (about:"Remove a key and value pair")
          (@arg key:+required ... "Remove key")
        )
    ).get_matches();
    let mut store = KvStore::new();
    match _matchs.subcommand_name() {
        Some("set") => {
            match _matchs.values_of("keyvalue") {
                Some(arg) => {
                    let args_vec: Vec<_> = arg.collect();
                    let key = args_vec[0].to_string();
                    let value = args_vec[1].to_string();
                    store.set(key, value);
                }
                None => unimplemented!("unimplemented"),
            }
        }
        Some("get") =>
            {
                let args = _matchs.value_of("key");
                match args {
                    Some(arg) => {
                        let key = arg.to_string();
                        let v = store.get(key);
                        match v {
                            Some(get_value) => println!("{}", get_value),
                            _ => unimplemented!("unimplemented"),
                        }
                    }
                    None => unimplemented!("unimplemented"),
                }
            }
        Some("rm") => {
            let args = _matchs.value_of("key");
            match args {
                Some(arg) => {
                    let key = arg.to_string();
                    let v = store.get(key.clone());
                    match v {
                        Some(_value) => store.remove(key).unwrap(),
                        _ => unimplemented!("unimplemented"),
                    };
                }
                None => unimplemented!("unimplemented"),
            }
        }
        _ => unimplemented!("unimplemented")
    }
}