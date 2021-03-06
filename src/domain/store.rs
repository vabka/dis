use kv::{Bucket, Config, Error, Store};

#[derive(Clone)]
pub struct Storage {
    store: Store,
    name: Option<String>,
}

impl Storage {
    pub fn new(path: &str, name: Option<&str>) -> Result<Self, Error> {
        let config = Config::new(path);
        let store = Store::new(config)?;
        Ok(Self {
            store,
            name: name.map(String::from),
        })
    }

    fn get_bucket(&self) -> Result<Bucket<&str, String>, Error> {
        self.store.bucket::<&str, String>(self.name.as_deref())
    }

    pub async fn insert(&self, key: &str, value: &String) -> Result<(), InsertError> {
        let bucket = self.get_bucket()?;
        if bucket.contains(&key)? {
            return Err(InsertError::ExistingKey);
        }
        bucket.set(&key, value)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn update(&self, key: &str, value: &String) -> Result<(), UpdateError> {
        let bucket = self.get_bucket()?;
        if !bucket.contains(&key)? {
            return Err(UpdateError::MissingKey);
        }
        bucket.set(&key, value)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn upsert(&self, key: &str, value: &String) -> Result<(), UpsertError> {
        let bucket = self.get_bucket()?;
        bucket.set(&key, value)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), DeleteError> {
        let bucket = self.get_bucket()?;
        if !bucket.contains(&key)? {
            return Err(DeleteError::MissingKey);
        }
        bucket.remove(&key)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn read(&self, key: &str) -> Result<String, ReadError> {
        let bucket = self.get_bucket()?;
        bucket.get(&key)?.ok_or(ReadError::NoData)
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

#[derive(thiserror::Error, Debug)]
pub enum ListError {
    #[error("Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for ListError {
    fn from(e: Error) -> Self {
        ListError::Kv(e)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InsertError {
    #[error("Key exists")]
    ExistingKey,
    #[error("Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for InsertError {
    fn from(e: kv::Error) -> Self {
        InsertError::Kv(e)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateError {
    #[error("Key missing")]
    MissingKey,
    #[error("Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for UpdateError {
    fn from(e: kv::Error) -> Self {
        UpdateError::Kv(e)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UpsertError {
    #[error("Error in kv::")]
    Kv(kv::Error),
}

impl From<kv::Error> for UpsertError {
    fn from(e: kv::Error) -> Self {
        UpsertError::Kv(e)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteError {
    #[error("Key missing")]
    MissingKey,
    #[error("Error in kv::")]
    Kv(kv::Error),
}

impl From<Error> for DeleteError {
    fn from(e: Error) -> Self {
        DeleteError::Kv(e)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReadError {
    #[error("Key missing")]
    MissingKey,
    #[error("No data")]
    NoData,
    #[error("Error in kv:{}", 0)]
    Kv(kv::Error),
}

impl From<kv::Error> for ReadError {
    fn from(e: kv::Error) -> Self {
        ReadError::Kv(e)
    }
}
