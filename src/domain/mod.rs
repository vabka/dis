use kv::Bucket;

pub struct Storage<'a, 'b> {
    file_path: &'a str,
    name: Option<&'b str>,
}

impl<'x, 'y> Storage<'x, 'y> {
    fn get_bucket(&self) -> Result<Bucket<&str, String>, kv::Error> {
        let config = kv::Config::new(self.file_path);
        let store = kv::Store::new(config)?;
        store.bucket::<&str, String>(self.name)
    }

    pub async fn insert(&mut self, key: &str, value: &str) -> Result<(), InsertError> {
        let bucket = self.get_bucket()?;
        if bucket.contains(key)? {
            return Err(InsertError::ExistingKey);
        }
        bucket.set(key, value)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn update(&mut self, key: &str, value: &str) -> Result<(), UpdateError> {
        let bucket = self.get_bucket()?;
        if !bucket.contains(key)? {
            return Err(UpdateError::MissingKey);
        }
        bucket.set(key, value)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn upsert(&mut self, key: &str, value: &str) -> Result<(), UpsertError> {
        let bucket = self.get_bucket()?;
        bucket.set(key, value)?;
        bucket.flush_async().await?;
        Ok(())
    }

    pub async fn delete(&mut self, key: &str) -> Result<(), DeleteError> {
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
        if bucket.contains(key)? {
            return Err(ReadError::MissingKey);
        }
        bucket.get(key)?.ok_or(ReadError::NoData)
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