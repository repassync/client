extern crate pkg_config;
extern crate xml;

use std::env;
use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use std::process::Command;

use xml::reader::{EventReader, XmlEvent};

fn main() {

    // check that required dependencies are installed
    pkg_config::Config::new().atleast_version("1.1.0").probe("openssl").unwrap();
    pkg_config::Config::new().atleast_version("3.20.0").probe("gtk+-3.0").unwrap();

    let profile = env::var("PROFILE").unwrap();
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let out_dir = env::var("OUT_DIR").unwrap();
    let datadir_in = Path::new(&out_dir).join("datadir.in");
    let data_dir = get_data_dir(&base_dir, &profile);
    if !datadir_in.exists() || profile == "release" {

        let mut f = File::create(datadir_in).unwrap();

        f.write_all(&format!("\"{}\"", data_dir.display()).as_bytes()).unwrap();

    }

    if profile == "debug" {
        // recompile resources if needed
        let resources_src = Path::new(&data_dir).join("repassync.gresource.xml");
        let resources_bin = Path::new(&data_dir).join("repassync.gresource");
        if !resources_bin.exists() || modified_after(&resources_src, &resources_bin) {
            let mut child = Command::new("glib-compile-resources")
                        .arg(resources_src)
                        .current_dir(data_dir)
                        .spawn()
                        .expect("failed to execute child");

            let ecode = child.wait()
                 .expect("failed to wait on child");

        }
    }

}

fn get_data_dir(base_dir: &str, profile: &String) -> PathBuf {
    if profile == "debug" {
        // if in debug mode, initialize the local data for testing
        Path::new(base_dir).join("data")
    } else {
        Path::new(&env::var("RELEASE_DATA_DIR").unwrap()).to_path_buf()
    }
}

fn modified_after(origin: &PathBuf, new: &PathBuf) -> bool {
    let metadata = origin.metadata().unwrap();
    let dir = origin.parent().unwrap();
    let resource_file = File::open(origin).unwrap();
    let mut parser = EventReader::new(resource_file);
    let origin_modified =
        metadata.modified().and_then(|m| newest(dir, &mut parser, false, m));
    let new_modified = new.metadata().and_then(|md| md.modified());
    match (origin_modified, new_modified) {
        (Ok(ts1), Ok(ts2)) => ts1 >= ts2,
        (_, _) => true
    }
}

fn newest(dir: &Path, reader: &mut EventReader<File>, read_path: bool, last: SystemTime) -> Result<SystemTime, Error> {
    match reader.next() {
        Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "file" => {
            newest(dir, reader, true, last)
        },
        Ok(XmlEvent::Characters(ref path)) if read_path => {
            let path = dir.join(path);
            let res =
                match path.metadata().and_then(|m| m.modified()) {
                    Ok(m) => {
                        if m > last {
                            newest(dir, reader, false, m)
                        } else {
                            newest(dir, reader, false, last)
                        }
                    },
                    e => {
                        e
                    }
                };
            res.or_else(|e| panic!("error: {:?} for file {:?}", e, path))
        },
        Ok(XmlEvent::EndDocument) => {
            Ok(last)
        },
        Ok(_) => {
            newest(dir, reader, read_path, last)
        },
        Err(e) => {
            Err(Error::new(ErrorKind::Other, e))
        }
    }
}
