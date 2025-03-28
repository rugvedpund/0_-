use std::io::Error;

use tokio::io::AsyncWriteExt;

pub async fn write_and_flush<T>(
    stream: &mut T,
    buf: &[u8],
) -> Result<(), Error>
where
    T: AsyncWriteExt + Unpin,
{
    stream.write_all(buf).await?;
    stream.flush().await
}
