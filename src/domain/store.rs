use kv::{Bucket, Error, Store};
use log::debug;

#[derive(Clone)]
pub struct Storage {
    store: Store,
    name: Option<String>,
}

impl Storage {
    pub fn new(path: &str, name: Option<&str>) -> Result<Self, kv::Error> {
        let config = kv::Config::new(path);
        let store = kv::Store::new(config)?;
        Ok(Self {
            store,
            name: name.map(String::from),
        })
    }

    fn get_bucket(&self) -> Result<Bucket<&str, String>, kv::Error> {
        self.store
            .bucket::<&str, String>(self.name.as_ref().map(String::as_str))
    }

    pub async fn insert(&self, key: &str, value: &str) -> Result<(), InsertError> {
        let bucket = self.get_bucket()?;
        if bucket.contains(key)? {
            return Err(InsertError::ExistingKey);
        }
        bucket.set(key, value)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn update(&self, key: &str, value: &str) -> Result<(), UpdateError> {
        let bucket = self.get_bucket()?;
        if !bucket.contains(key)? {
            return Err(UpdateError::MissingKey);
        }
        bucket.set(key, value)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn upsert(&self, key: &str, value: &str) -> Result<(), UpsertError> {
        let bucket = self.get_bucket()?;
        bucket.set(key, value)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), DeleteError> {
        let bucket = self.get_bucket()?;
        if !bucket.contains(key)? {
            return Err(DeleteError::MissingKey);
        }
        bucket.remove(key)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn read(&self, key: &str) -> Result<String, ReadError> {
        let bucket = self.get_bucket()?;
        bucket.get(key)?.ok_or(ReadError::NoData)
    }

    pub async fn list(&self) -> Result<Vec<String>, ListError> {
        let bucket = self.get_bucket()?;
        let mut keys = vec![];
        for item in bucket.iter() {
            let item = item?;
            let key: String = item.key()?;
            keys.push(key);
        }
        Ok(keys)
    }
}

#[derive(err_derive::Error, Debug)]
pub enum ListError {
    #[error(display = "Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for ListError {
    fn from(e: Error) -> Self {
        ListError::Kv(e)
    }
}

#[derive(err_derive::Error, Debug)]
pub enum InsertError {
    #[error(display = "Key exists")]
    ExistingKey,
    #[error(display = "Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for InsertError {
    fn from(e: kv::Error) -> Self {
        InsertError::Kv(e)
    }
}

#[derive(err_derive::Error, Debug)]
pub enum UpdateError {
    #[error(display = "Key missing")]
    MissingKey,
    #[error(display = "Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for UpdateError {
    fn from(e: kv::Error) -> Self {
        UpdateError::Kv(e)
    }
}

#[derive(err_derive::Error, Debug)]
pub enum UpsertError {
    #[error(display = "Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for UpsertError {
    fn from(e: kv::Error) -> Self {
        UpsertError::Kv(e)
    }
}

#[derive(err_derive::Error, Debug)]
pub enum DeleteError {
    #[error(display = "Key missing")]
    MissingKey,
    #[error(display = "Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for DeleteError {
    fn from(e: kv::Error) -> Self {
        DeleteError::Kv(e)
    }
}

#[derive(err_derive::Error, Debug)]
pub enum ReadError {
    #[error(display = "Key missing")]
    MissingKey,
    #[error(display = "No data")]
    NoData,
    #[error(display = "Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for ReadError {
    fn from(e: kv::Error) -> Self {
        ReadError::Kv(e)
    }
}
