use crate::Result;
use crate::{Commands, KvsError};

use bincode::config::Configuration;
use bincode::decode_from_std_read;
use bincode::{config, encode_into_std_write};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::SeekFrom::Start;
use std::io::{BufReader, BufWriter, Seek, Write};
use std::path::PathBuf;

/// Struct holding stored key-value pairs.
/// Both keys and values are of `String` type.
/// Currently, key-value pairs are stored in memory using a hash map.
///
/// Usage:
///
/// ```rust
/// use kvs::KvStore;
///
/// let mut store = KvStore::open("log.avro");
///
/// store.set("hello".to_owned(), "world".to_owned())?;
///
/// let value = store.get("hello".to_owned())?;
///
/// assert_eq!(value, Some("world".to_owned()));
/// ```
pub struct KvStore {
    log_path: PathBuf,
    index: HashMap<String, u64>,
    bincode_config: Configuration,
    current_offset: u64,
}

/// Key-value store
impl KvStore {
    /// Gets a value corresponding to a string key
    pub fn get(&self, key: String) -> Result<Option<String>> {
        if let Some(&offset) = self.index.get(&key) {
            let log_file = OpenOptions::new().read(true).open(&self.log_path)?;
            let mut buf_reader = BufReader::new(log_file);
            buf_reader.seek(Start(offset))?;

            let command: Commands = decode_from_std_read(&mut buf_reader, self.bincode_config)?;

            return match command {
                Commands::Set { value, .. } => Ok(Some(value)),
                _ => Err(KvsError::UnexpectedCommand),
            };
        }

        Ok(None)
    }

    /// Sets a value under a string key
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let log_file = OpenOptions::new().append(true).open(&self.log_path)?;
        let mut buf_writer = BufWriter::new(log_file);

        let command = Commands::Set {
            key: key.clone(),
            value: value.clone(),
        };

        let bytes_written =
            encode_into_std_write(command, &mut buf_writer, self.bincode_config)? as u64;

        buf_writer.flush()?;

        self.index.insert(key, bytes_written);
        self.current_offset += bytes_written;

        Ok(())
    }

    /// Removes a value under a string key
    pub fn remove(&mut self, key: String) -> Result<()> {
        if let Some(_offset) = self.index.get(&key) {
            let log_file = OpenOptions::new().append(true).open(&self.log_path)?;
            let mut buf_writer = BufWriter::new(log_file);

            let command = Commands::Rm { key: key.clone() };

            let bytes_written =
                encode_into_std_write(command, &mut buf_writer, self.bincode_config)? as u64;

            buf_writer.flush()?;

            self.index.insert(key, self.current_offset + bytes_written);
            self.current_offset += bytes_written;

            return Ok(());
        }

        Err(KvsError::KeyNotFound)
    }

    /// Create a new store with command log file under the given folder
    /// Open the store under the given folder otherwise
    pub fn open<Path: Into<PathBuf>>(path: Path) -> Result<Self> {
        let path = path.into();

        let bincode_config = config::standard();
        bincode_config.with_big_endian().with_fixed_int_encoding();

        let mut store = KvStore {
            log_path: path.join("command_log.bin"),
            index: HashMap::new(),
            bincode_config,
            current_offset: 0,
        };

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .read(true)
            .open(&store.log_path)?;

        let mut buf_reader = BufReader::new(file);

        while let Ok(command) = decode_from_std_read(&mut buf_reader, store.bincode_config) {
            match command {
                Commands::Set { key, .. } => {
                    store.index.insert(key, store.current_offset);
                }
                Commands::Rm { key } => {
                    store.index.remove(&key);
                }
                _ => return Err(KvsError::UnexpectedCommand),
            }

            store.current_offset = buf_reader.stream_position()?;
        }

        Ok(store)
    }
}
