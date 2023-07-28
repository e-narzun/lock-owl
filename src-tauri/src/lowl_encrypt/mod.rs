// todo remove unused imports

use anyhow::Context;
use core::fmt;
#[allow(unused_imports)]
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::der::Writer,
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fs::remove_file, time::SystemTime};
use std::{
    fs::create_dir,
    fs::File,
    fs::OpenOptions,
    io::{self, Read},
    path::Path,
};
use std::{io::Seek, path::PathBuf};
use tauri::api::path::BaseDirectory;
mod aes_encryption;
pub mod utility;

pub struct LowlFileHeader {
    pub(crate) version: u16,
    pub(crate) file_extension: String,
    pub(crate) origin_creation_date: i64,
    pub(crate) creator_name: String,
    pub(crate) header_length: u16,
}
impl fmt::Debug for LowlFileHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("Version", &self.version)
            .field("File extension", &self.file_extension)
            .field("Origin creation date", &self.origin_creation_date)
            .field("Creator name", &self.creator_name)
            .field("Header length", &self.header_length)
            .finish()
    }
}
#[derive(Serialize, Deserialize)]
pub struct FileDisplay {
    filename: String,
    path: String,
    status: bool,
}

pub fn generate_new_keys(
    overwrite: bool,
    password: &str,
) -> Result<(RsaPrivateKey, RsaPublicKey), anyhow::Error> {
    let mut rng = rand::thread_rng();
    let bits: usize = 2048;
    let priv_key: RsaPrivateKey = RsaPrivateKey::new(&mut rng, bits)?;
    let pub_key: RsaPublicKey = RsaPublicKey::from(&priv_key);
    let directory = utility::get_default_dir(BaseDirectory::AppConfig)
        .map_err(|_err| anyhow::anyhow!("Unable to get default path"))?;
    store_keys(&priv_key, &pub_key, directory, overwrite, password)
        .map_err(|_err| anyhow::anyhow!("Unable to store the keys."))?;
    return Ok((priv_key, pub_key));
}

pub fn load_private_key(name: &str, password: &str) -> Result<RsaPrivateKey, anyhow::Error> {
    dbg!("Load key");
    let location: PathBuf = utility::get_default_dir(BaseDirectory::AppConfig)
        .map_err(|_err| anyhow::anyhow!("Unable to get default path"))?;
    let private_key_path: PathBuf = location.as_path().join(format!("{}.key", name));
    let mut file = OpenOptions::new().read(true).open(&private_key_path)?;
    let mut buffer: Vec<u8> = vec![];
    let _ = file.read_to_end(&mut buffer);
    let decrypted_private_key = aes_encryption::decrypt_with_password(password, &buffer)?;
    let private_key =
        rsa::RsaPrivateKey::from_pkcs1_pem(std::str::from_utf8(&decrypted_private_key)?)?;
    return Ok(private_key);
}
#[allow(dead_code)]
fn load_public_key(name: &str) -> Result<RsaPublicKey, ()> {
    let location: PathBuf =
        utility::get_default_dir(BaseDirectory::AppConfig).map_err(|_err| ())?;
    let public_key_path: PathBuf = location.as_path().join(format!("{}.key", name));
    Ok(rsa::RsaPublicKey::read_pkcs1_pem_file(public_key_path).map_err(|_err| ())?)
}

fn store_keys(
    priv_key: &RsaPrivateKey,
    pub_key: &RsaPublicKey,
    path: PathBuf,
    overwrite: bool,
    password: &str,
) -> Result<(), ()> {
    if !path.is_dir() {
        let _ = create_dir(&path);
    }
    let private_key_path: PathBuf = path.as_path().join("private.key");
    let public_key_path: PathBuf = path.as_path().join("public.key");
    let binding = priv_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::CR).unwrap();
    let private_key: &[u8] = binding.as_bytes();
    let binding = pub_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::CR).unwrap();
    let public_key: &[u8] = binding.as_bytes();
    let encrypted_private_key =
        aes_encryption::encrypt_with_password(password, private_key).unwrap();
    let _ = write_pem_to_file(
        encrypted_private_key.as_slice(),
        overwrite,
        private_key_path,
    )
    .map_err(|_err| ())?;
    let _ = write_pem_to_file(public_key, overwrite, public_key_path).map_err(|_err| ())?;
    return Ok(());
}

fn write_pem_to_file(key: &[u8], overwrite: bool, path: PathBuf) -> Result<(), ()> {
    if !path.parent().unwrap().is_dir() || path.is_file() && !overwrite {
        return Err(());
    }
    let mut file = create_new_file(&path, overwrite).map_err(|_err| ())?;
    match file.write(key) {
        Result::Ok(file) => file,
        Result::Err(_err) => return Err(()),
    };
    match file.sync_all() {
        Result::Ok(_) => return Ok(()),
        Result::Err(_err) => return Err(()),
    };
}

fn create_new_file(path: &Path, overwrite: bool) -> Result<File, io::Error> {
    if path.exists() && !overwrite {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "This file already exists. ",
        ));
    }
    if overwrite {
        return Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(overwrite)
            .create(overwrite)
            .open(&path)?);
    } else {
        return Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)?);
    }
}

pub fn encrypt_file(
    path: &Path,
    overwrite: bool,
    version: u16,
    creator: &str,
) -> Result<FileDisplay, anyhow::Error> {
    if !path.is_file() {
        return Err(anyhow::anyhow!("No file found"));
    }

    let old_file_meta: LowlFileHeader = LowlFileHeader {
        version: version,
        file_extension: path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_os_string()
            .into_string()
            .map_err(|_err| anyhow::anyhow!("Unable to read extension"))?,
        origin_creation_date: path
            .metadata()?
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            .try_into()?,
        creator_name: creator.to_string(),
        header_length: 0 as u16,
    };

    let parent_folder: &Path = path.parent().context("Unknown location")?;
    let new_file_path = parent_folder
        .join(path.file_stem().unwrap())
        .with_extension("lowl");

    let mut new_file = create_new_file(new_file_path.as_path(), overwrite)
        .map_err(|_err| anyhow::anyhow!("Couldn't create new file"))?;

    add_header(&mut new_file, old_file_meta)?;
    let mut old_file: File = OpenOptions::new().read(true).write(true).open(&path)?;
    let pub_key =
        load_public_key("public").map_err(|_err| anyhow::anyhow!("Unable to load public key"))?;
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    const BUFFER_LEN: usize = 245;
    let mut buffer = [0u8; BUFFER_LEN];
    loop {
        let read_count = old_file.read(&mut buffer)?;
        let encrypted_data: Vec<u8> =
            pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &buffer[..read_count])?;
        new_file.write(&encrypted_data)?;
        if read_count != BUFFER_LEN {
            break;
        }
    }
    new_file.sync_all()?;
    remove_file(path)?;

    let answer: FileDisplay = FileDisplay {
        filename: new_file_path
            .file_name()
            .expect("Couldn't get filename")
            .to_string_lossy()
            .into_owned(),
        path: new_file_path.to_string_lossy().into_owned(),
        status: true,
    };

    return Ok(answer);
}

pub fn decrypt_file(
    path: &Path,
    overwrite: bool,
    password: &str,
) -> Result<FileDisplay, anyhow::Error> {
    if !path.is_file() {
        return Err(anyhow::anyhow!("No file found"));
    }
    let mut old_file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(overwrite)
        .open(path)?;

    let old_header = get_file_header(&mut old_file)?;

    let key: rsa::RsaPrivateKey = load_private_key("private", password)?;

    let parent_folder = path.parent().context("Unknown location")?;
    let file_name = path.file_stem().context("Couldn't get filename")?;

    let new_file_path = parent_folder
        .join(&file_name)
        .with_extension(old_header.file_extension);

    let mut new_file = create_new_file(new_file_path.as_path(), overwrite)?;

    const BUFFER_LEN: usize = 256;
    let mut buffer = [0u8; BUFFER_LEN];
    loop {
        let read_count = old_file.read(&mut buffer)?;
        if read_count == 0 {
            break;
        }
        let encrypted_data: Vec<u8> = key.decrypt(Pkcs1v15Encrypt, &buffer[..read_count])?;
        new_file.write(&encrypted_data)?;
        if read_count != BUFFER_LEN {
            break;
        }
    }

    new_file.sync_all()?;
    remove_file(&path)?;

    let old_filetime = filetime::FileTime::from_unix_time(old_header.origin_creation_date, 0);
    filetime::set_file_mtime(&new_file_path, old_filetime)?;

    let answer: FileDisplay = FileDisplay {
        filename: new_file_path
            .file_name()
            .expect("Couldn't get filename")
            .to_string_lossy()
            .into_owned(),
        path: new_file_path.to_string_lossy().into_owned(),
        status: false,
    };

    return Ok(answer);
}

fn add_header(file: &mut File, header_input: LowlFileHeader) -> Result<(), anyhow::Error> {
    if header_input.creator_name.len() > 255 && header_input.file_extension.len() > 255 {
        return Err(anyhow::anyhow!("Invalid header parameter"));
    }
    file.rewind().expect("Unable to reset file cursor.");

    // Heder input data
    let mut header_information: Vec<u8> = vec![];
    header_information.extend(header_input.origin_creation_date.to_be_bytes());
    header_information.push(header_input.file_extension.len() as u8);
    header_information.extend(header_input.file_extension.as_bytes());
    header_information.push(header_input.creator_name.len() as u8);
    header_information.extend(header_input.creator_name.as_bytes());

    let mut header_output: Vec<u8> = vec![];
    header_output.extend(String::from("lowl").as_bytes());
    header_output.extend(header_input.version.to_be_bytes());
    header_output.extend((header_information.len() as u16).to_be_bytes());
    header_output.extend(header_information);

    file.write(&header_output)?;
    file.sync_all()?;
    return Ok(());
}

fn get_file_header(file: &mut File) -> Result<LowlFileHeader, anyhow::Error> {
    file.rewind().expect("Unable to reset file cursor");

    if std::str::from_utf8(&read_n_bytes(file, 4)?)? != "lowl" {
        return Err(anyhow::anyhow!("Wrong file formate"));
    }
    let version: usize = utility::convert_to_unsigned(read_n_bytes(file, 2)?).unwrap();
    let header_length: usize =
        utility::convert_to_unsigned(read_n_bytes(file, 2).unwrap()).unwrap();
    let creation_date: usize = utility::convert_to_signed(read_n_bytes(file, 8).unwrap()).unwrap();

    let old_extension_len = utility::convert_to_unsigned(read_n_bytes(file, 1).unwrap()).unwrap();
    let old_extension: String =
        std::str::from_utf8(&read_n_bytes(file, old_extension_len).unwrap())
            .unwrap()
            .to_string();
    let creator_len = utility::convert_to_unsigned(read_n_bytes(file, 1).unwrap()).unwrap();
    let creator: String = std::str::from_utf8(&read_n_bytes(file, creator_len).unwrap())
        .unwrap()
        .to_string();

    return Ok(LowlFileHeader {
        version: version as u16,
        file_extension: old_extension,
        origin_creation_date: creation_date as i64,
        creator_name: creator,
        header_length: header_length as u16,
    });
}

fn read_n_bytes(file: &mut File, length: usize) -> Result<Vec<u8>, anyhow::Error> {
    let mut buffer: Vec<u8> = vec![0; length];
    file.read_exact(&mut buffer)
        .map_err(|err| anyhow::anyhow!("Couldn't read from file: {}", err))?;
    return Ok(buffer);
}
