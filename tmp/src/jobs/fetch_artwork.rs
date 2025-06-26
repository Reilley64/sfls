use crate::jobs::Job;
use crate::repositories;
use crate::state::AppState;
use tracing::info;

pub struct FetchArtworkPayload {
    pub media_id: i64,
}

impl FetchArtworkPayload {
    pub fn new(media_id: i64) -> Self {
        Self { media_id }
    }
}

pub struct FetchArtwork {
    pub state: AppState,
    pub payload: FetchArtworkPayload,
}

impl FetchArtwork {
    pub fn new(state: AppState, payload: FetchArtworkPayload) -> Self {
        Self { state, payload }
    }
}

#[async_trait::async_trait]
impl Job for FetchArtwork {
    async fn run(&self) -> Result<(), anyhow::Error> {
        info!("Fetching artwork for Media {}", self.payload.media_id);

        let mut connection = self.state.pool.get().await?;

        let media = repositories::media::find_by_id(&mut connection, self.payload.media_id)
            .await
            .map_err(|e| anyhow::Error::msg(format!("Error calling media::find_by_id: {e}")))?
            .ok_or(anyhow::Error::msg(format!(
                "Media with id {} not found",
                self.payload.media_id
            )))?;

        self.state
            .artwork_fetcher_factory
            .artwork_fetchers
            .get(&media.type_)
            .ok_or(anyhow::Error::msg("Unknown media type"))?
            .fetch_artwork(self.state.clone(), media.id)
            .await?;

        info!(
            "Finished downloading artwork for media: {}",
            self.payload.media_id
        );
        Ok(())
    }
}
