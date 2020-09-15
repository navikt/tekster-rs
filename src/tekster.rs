use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;
use std::fs::File;

lazy_static! {
    pub static ref TEKSTER: String = serde_json::to_string(&bygg_tekster().unwrap()).unwrap();
}

fn bygg_tekster() -> std::io::Result<HashMap<String, String>> {
    dotenv::dotenv().ok();

    let tekster_sti = std::env::var("TEKSTER_PATH").expect("TEKSTER_PATH");
    let tekster_sprak = std::env::var("TEKSTER_SPRAK").expect("TEKSTER_SPRAK");

    let mut tekst_map: HashMap<String, String> = HashMap::new();
    let fil_match = &format!("_{}", tekster_sprak);

    for sti in fs::read_dir(tekster_sti)? {
        if let Ok(tekstfil) = sti {
            if tekstfil.path().to_string_lossy().find(fil_match).is_some() {
                let mut fil = File::open(tekstfil.path())?;
                let mut innhold = String::new();
                fil.read_to_string(&mut innhold)?;
                
                let mut nokkel = match tekstfil.path().file_stem() {
                    Some(n) => match n.to_os_string().into_string() {
                        Ok(nokkel) => nokkel,
                        Err(_) => continue
                    },
                    None => continue
                };
                let lengde = nokkel.len();
                nokkel.truncate(lengde - 3);
                tekst_map.insert(nokkel, innhold);
            }
        }
    }
    Ok(tekst_map)
}
