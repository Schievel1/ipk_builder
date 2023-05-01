pub mod ui;

use anyhow::{Context, bail, Result};
use flate2::{write::GzEncoder, Compression};
use log::trace;
use std::{
    fs::{File, self},
    io::Read,
    mem::size_of_val,
    path::{Path, PathBuf},
};
use tar::{Builder, Header};
use ui::{IpkBuilder, ScriptSource};

pub fn header_from_file(f: &mut File) -> Result<(Header, Vec<u8>)> {
    let buffer: Vec<u8> = f.bytes().flatten().collect();
    Ok((header_from_buf(&buffer[..]), buffer))
}

pub fn header_from_buf<T: ?Sized>(b: &T) -> Header {
    let mut header = Header::new_gnu();
    header.set_size(size_of_val(b) as u64);
    header.set_uid(0);
    header.set_gid(0);
    header.set_mode(0o644);
    header.set_cksum();
    header
}

pub fn append_file<P: AsRef<Path>>(
    src_path: P,
    arch: &mut Builder<flate2::write::GzEncoder<&File>>,
    tgt_path: &str,
    tar_path: Option<&PathBuf>,
) -> Result<()> {
    match tar_path {
        Some(s) => trace!(
            "Packaging {:-<width$} into {}",
            src_path.as_ref().display(),
            s.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
            width = 120
        ),
        None => trace!("Packaging {}...", src_path.as_ref().display()),
    }

    let mut file = File::open(&src_path).context(format!(
        "Could not open {} to append to tar",
        src_path.as_ref().display()
    ))?;
    let mut headbuf = header_from_file(&mut file)?;
    arch.append_data(&mut headbuf.0, tgt_path, &headbuf.1[..])?;
    Ok(())
}

pub fn make_package(
    data: &IpkBuilder,
) -> Result<String> {
    let mut control_tar: PathBuf = data.output_path.clone().unwrap().into();
    let mut data_tar:  PathBuf = data.output_path.clone().unwrap().into();
    {
        // do this in it's own scope so files are dropped and closed at the end of the scope
        control_tar.push("control.tar.gz");
        let control_archive =
            File::create(&control_tar).context("Could not create control.tar.gz")?;
        let enc = GzEncoder::new(&control_archive, Compression::default());
        let mut tar = tar::Builder::new(enc);

        trace!(
            "Packaging control file into {}",
            control_tar
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        );
        if data.control_file.file_or_text == ScriptSource::FromPath {
           append_file(data.control_file.picked_path.clone().unwrap(), &mut tar, "control", Some(&control_tar))?;
        } else {
        let mut header = header_from_buf(data.control_file.from_textbox.as_bytes());
        tar.append_data(
            &mut header,
            "control",
            data.control_file.from_textbox.as_bytes(),
        )?;
        }

        if data.postinst.enabled {
        trace!(
            "Packaging postinst script into {}",
            control_tar
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        );
        if data.postinst.file_or_text == ScriptSource::FromPath {
           append_file(data.postinst.picked_path.clone().unwrap(), &mut tar, "postinst", Some(&control_tar))?;
        } else {
        let mut header = header_from_buf(data.postinst.from_textbox.as_bytes());
        header.set_mode(0o755);
        header.set_cksum();
        tar.append_data(
            &mut header,
            "postinst",
            data.postinst.from_textbox.as_bytes(),
        )?;
        }
        }

        if data.preinst.enabled {
        trace!(
            "Packaging preinst script into {}",
            control_tar
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        );
        if data.preinst.file_or_text == ScriptSource::FromPath {
           append_file(data.preinst.picked_path.clone().unwrap(), &mut tar, "postinst", Some(&control_tar))?;
        } else {
        let mut header = header_from_buf(data.postinst.from_textbox.as_bytes());
        header.set_mode(0o755);
        header.set_cksum();
        tar.append_data(
            &mut header,
            "preinst",
        data.preinst.from_textbox.as_bytes(),
        )?;
        }
        }

        if data.prerm.enabled {
        trace!(
            "Packaging prerm script into {}",
            control_tar
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        );
        if data.prerm.file_or_text == ScriptSource::FromPath {
           append_file(Path::new(&data.prerm.picked_path.clone().unwrap()), &mut tar, "postinst", Some(&control_tar))?;
        } else {
        let mut header = header_from_buf(data.prerm.from_textbox.as_bytes());
        header.set_mode(0o755);
        header.set_cksum();
        tar.append_data(
            &mut header,
            "prerm",
            data.prerm.from_textbox.as_bytes(),
        )?;
        }
        }
        tar.finish()?;
    }
        trace!("Created control tar archive {}", control_tar.display());

        data_tar.push("data.tar.gz");
        let data_archive = File::create(&data_tar).context("Could not create data.tar.gz")?;
        let enc = GzEncoder::new(&data_archive, Compression::default());
        let mut tar = tar::Builder::new(enc);
        tar.append_dir_all(
            &data.data_path.clone().unwrap(), "/"
        )?;

    let package_name = "outpackage.ipk".to_owned();
    let mut package_tar: PathBuf = data.output_path.clone().unwrap().into();
    package_tar.push(package_name);
    // try to delete any previous package, do nothing if not found
    if let Err(e) = fs::remove_file(&package_tar) {
        if std::io::ErrorKind::NotFound != e.kind() {
            bail!(
                "Error deleting {} before creating package.",
                package_tar.display()
            );
        }
    }
    let package_archive = File::create(&package_tar).context("Could not create package archive")?;
    let enc = GzEncoder::new(&package_archive, Compression::default());
    let mut tar = tar::Builder::new(enc);
    append_file(&control_tar, &mut tar, "control.tar.gz", Some(&package_tar))
        .context("Error appending control.tar.gz to package archive")?;
    append_file(&data_tar, &mut tar, "data.tar.gz", Some(&package_tar))
        .context("Error appending data.tar.gz to package archive")?;
        trace!(
            "Packaging debian_binary script into {}",
            package_tar
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        );
        if data.debian_binary.file_or_text == ScriptSource::FromPath {
           append_file(Path::new(&data.debian_binary.picked_path.clone().unwrap()), &mut tar, "debin_binary", Some(&package_tar))?;
        } else {
        let mut header = header_from_buf(data.debian_binary.from_textbox.as_bytes());
        header.set_mode(0o755);
        header.set_cksum();
        tar.append_data(
            &mut header,
            "debian_binary",
            data.debian_binary.from_textbox.as_bytes(),
        )?;
        }

    // cleanup
    fs::remove_file(control_tar).context("Error removing contor.tar.gz")?;
    fs::remove_file(data_tar).context("Error removing data.tar.gz")?;

    Ok(package_tar.display().to_string())
}
