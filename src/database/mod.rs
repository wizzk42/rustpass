use crate::{
    api::header::{
        Header,
        InnerHeader
    },
    errors::DatabaseIntegrityError,
    results::Result,
    internal::{
        database::binary::{
            constants::{
                KDB_MAGIC,
                KDBX_MAGIC,
            },
            version::get_kdbx_version,
        },
        keyfile,
        primitives::cryptopraphy
    },
};

pub(crate) mod kdb;
pub(crate) mod kdbx3;
pub(crate) mod kdbx4;
pub(crate) mod utils;
pub(crate) mod items;

/// A decrypted KeePass types
#[derive(Debug)]
pub struct Database {
    /// Header information of the KeePass types
    pub header: Header,

    /// Optional inner header information
    pub inner_header: InnerHeader,

    /// Root node of the KeePass types
    pub root: items::Group,
}

impl Database {

    pub fn open(
        source: &mut dyn std::io::Read,
        password: Option<&str>,
        keyfile: Option<&mut dyn std::io::Read>,
    ) -> Result<Database> {
        let mut key_elements: Vec<Vec<u8>> = Vec::new();

        if let Some(p) = password {
            key_elements.push(
                cryptopraphy::sha256(&[p.as_bytes()])?
                    .as_slice()
                    .to_vec(),
            );
        }

        if let Some(f) = keyfile {
            key_elements.push(keyfile::parse(f)?);
        }

        let mut data = Vec::new();
        source.read_to_end(&mut data)?;

        let (
            version,
            file_major_version,
            file_minor_version
        ) = get_kdbx_version(
            data.as_ref()
        )?;

        match version {
            KDB_MAGIC => {
                kdb::parse(data.as_ref(), &key_elements)
            }
            KDBX_MAGIC if file_major_version == 3 => {
                kdbx3::parse(data.as_ref(), &key_elements)
            }
            KDBX_MAGIC if file_major_version == 4 => {
                kdbx4::parse(data.as_ref(), &key_elements)
            }
            _ => Err(
                DatabaseIntegrityError::InvalidKDBXVersion {
                    version,
                    file_major_version,
                    file_minor_version,
                }.into()
            ),
        }
    }

    pub fn close(&self) {
        // nothing yet
    }
}
