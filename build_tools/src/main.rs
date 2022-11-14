use std::{ffi::OsStr, io::Write, path::PathBuf, process::Command, str::FromStr};

use walkdir::WalkDir;

fn main() {
    let args = std::env::args().skip(1).peekable(); // skip executable name

    let mut run = false;
    let mut build = false;
    let mut clean = false;
    // let mut release = false;
    let mut unreconized = Vec::new();

    for string in args {
        match string.as_str().trim() {
            "run" => {
                run = true;
                build = true;
            }
            "build" => {
                build = true;
            }
            "clean" => {
                clean = true;
            }
            _ => {
                unreconized.push(string);
            }
        }
    }

    if clean {
        let mut run_cmd = Command::new("cargo");
        run_cmd.arg("clean");
        assert!(run_cmd.status().unwrap().success());

        let mut path = workspace_path();
        path.push("java_rt");
        path.push("out");
        std::fs::remove_dir_all(&path).unwrap();
        path.pop();
        path.push("build");
        std::fs::remove_dir_all(&path).unwrap();
    }

    if build {
        build_vm_binary("main");

        let bin = create_raw_binary("main");

        {
            let mut path = workspace_path();
            path.push("java_rt");

            let mut run_cmd = Command::new("mkdir");
            run_cmd.current_dir(path.as_path());
            run_cmd.arg("-p");
            run_cmd.arg("build");
            assert!(run_cmd.status().unwrap().success());

            let mut run_cmd = Command::new("mkdir");
            run_cmd.current_dir(path.as_path());
            run_cmd.arg("-p");
            run_cmd.arg("out");
            assert!(run_cmd.status().unwrap().success());

            let mut source = path.clone();
            source.push("src");
            let source = source;
            let im_source = source.as_path();

            let mut dest = path.clone();
            dest.push("build");
            let dest = dest;
            let im_dest = dest.as_path();

            let mut run_cmd = Command::new("unzip");
            run_cmd.current_dir(path.as_path());
            run_cmd.arg("-n");
            run_cmd.arg("./lib/*.jar");
            run_cmd.arg("-d");
            run_cmd.arg("./build");
            assert!(run_cmd.status().unwrap().success());

            let files: Vec<PathBuf> = walkdir::WalkDir::new(im_source)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    if e.path().extension() == Some(OsStr::new("java")) {
                        if let Ok(meta) = e.metadata() {
                            let smp = e.path().to_owned();
                            // strip the prefix
                            let smp = smp.as_path().strip_prefix(im_source).unwrap();

                            let mut buf = im_dest.to_path_buf();
                            buf.push(smp);
                            buf.set_extension("class");

                            // buf is not the src/-path- but build/-path-

                            let sm = meta.modified();
                            let dm = std::fs::metadata(&buf);
                            if let (Ok(sm), Ok(Ok(dm))) = (sm, dm.map(|e| e.modified())) {
                                if sm.gt(&dm) {
                                    std::fs::remove_file(buf).unwrap();
                                    // recompile because the source was edited
                                    Some(e.into_path())
                                } else {
                                    //skip
                                    None
                                }
                            } else {
                                //std::fs::remove_file(buf).unwrap();
                                //compile because we cant read the metadata
                                Some(e.into_path())
                            }
                        } else {
                            // std::fs::remove_file(buf).unwrap();
                            Some(e.into_path())
                        }
                    } else {
                        None
                    }
                })
                .collect();

            if !files.is_empty() {
                let mut run_cmd = Command::new("javac");
                run_cmd.current_dir(im_dest);
                run_cmd.arg("-d");
                run_cmd.arg(im_dest);
                for cf in Some(im_dest) {
                    // println!("{:?}", cf);
                    run_cmd.arg("-cp");
                    run_cmd.arg(cf);
                }

                run_cmd.args(files);
                assert!(run_cmd.status().unwrap().success());
            }

            //remove the stupid stuff from the brock jar
            {
                let mut rm = path.clone();
                rm.push("build");
                rm.push("README.TXT");
                std::fs::remove_file(rm).unwrap();

                let mut meta_inf = path.clone();
                meta_inf.push("build");
                meta_inf.push("package.bluej");
                std::fs::remove_file(meta_inf).unwrap();
            }

            let mut run_cmd = Command::new("javac");
            run_cmd.current_dir(path.as_path());
            run_cmd.arg("--version");
            let out = run_cmd.output().unwrap();
            assert!(out.status.success());
            let version = std::str::from_utf8(&out.stdout).unwrap().trim();
            let version = version.replace("javac ", "");

            let mut dest = dest.to_owned();
            dest.push("bin.bin");
            std::fs::copy(bin, &dest).unwrap();
            dest.pop();
            dest.push("META-INF");
            dest.push("MANIFEST.MF");
            println!("{:?}", dest);
            std::fs::File::create(dest)
                .unwrap()
                .write_all(
                    format!("Manifest-Version: 1.0\nCreated-By: {version}\nMain-Class: Main\n")
                        .as_bytes(),
                )
                .unwrap();

            let mut run_cmd = Command::new("zip");
            run_cmd.current_dir(im_dest);
            run_cmd.arg("../out/JavaRT.jar");
            run_cmd.arg("-r");
            run_cmd.arg("-u");
            let files: Vec<_> = WalkDir::new(im_dest)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .map(|e| e.into_path().strip_prefix(im_dest).unwrap().to_owned())
                .collect();
            run_cmd.args(&files);
            assert!(run_cmd.status().unwrap().success());
        }
    }

    if run {
        let mut run_cmd = Command::new("java");
        let mut path = workspace_path();
        path.push("java_rt");
        path.push("out");
        run_cmd.current_dir(path);
        run_cmd.arg("-jar");
        run_cmd.arg("JavaRT.jar");

        assert!(run_cmd.status().unwrap().success());
    }
}

pub fn workspace_path() -> PathBuf {
    let mut run_cmd = Command::new("cargo");
    run_cmd.arg("locate-project");
    run_cmd.arg("--message-format");
    run_cmd.arg("plain");
    run_cmd.arg("--workspace");

    let out = run_cmd.output().unwrap();
    assert!(out.status.success());
    let path = std::str::from_utf8(&out.stdout).unwrap();
    let path = path.trim();
    let path = path.split('\n').last().unwrap();
    let mut path = PathBuf::from_str(path).unwrap();
    path.pop();
    path
}

pub fn build_vm_binary(name: &str) {
    let mut run_cmd = Command::new("cargo");
    run_cmd.current_dir(std::env::current_dir().unwrap());

    run_cmd
        .arg("+nightly")
        .arg("build")
        .arg("--release")
        .arg("--package")
        .arg(name)
        .arg("--target")
        .arg("mips.json")
        .arg("-Zbuild-std=core,compiler_builtins,alloc")
        .arg("-Zbuild-std-features=compiler-builtins-mem");

    assert!(run_cmd.status().unwrap().success());
}

pub fn create_raw_binary(name: &str) -> PathBuf {
    let llvm_tools = llvm_tools::LlvmTools::new().unwrap();
    let objcopy = llvm_tools.tool(&llvm_tools::exe("llvm-objcopy")).unwrap();

    let mut run_cmd = Command::new(objcopy);
    let mut path = workspace_path();
    path.push("target");
    path.push("mips");
    path.push("release");
    run_cmd.current_dir(path.clone());

    run_cmd
        .arg("-O")
        .arg("binary")
        .arg("-I")
        .arg("elf32-big")
        .arg(&format!("./{}", name))
        .arg(&format!("./{}.bin", name));

    assert!(run_cmd.status().unwrap().success());

    path.push(&format!("{}.bin", name));
    path
}
