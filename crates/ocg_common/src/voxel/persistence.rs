//! Managing chunk persistence and presence in memory.

use anyhow::Result;
use ocg_schemas::coordinates::AbsChunkPos;
use ocg_schemas::voxel::chunk::Chunk;
use ocg_schemas::voxel::chunk_group::ChunkGroup;
use ocg_schemas::OcgExtraData;

/// A single response to a chunk loading request, generated some time after calling [`ChunkPersistenceLayer::request_load`].
pub type ChunkProviderResult<ExtraData> = Result<(AbsChunkPos, Chunk<ExtraData>)>;

/// A provider for chunk data for chunks not present in memory that need to be created/loaded, and a sink for the same data when the chunks are unloaded.
/// Examples include a disk persistence layer, a world generator and a network protocol wrapper.
/// Asynchronous to provide support for disk IO and networking.
pub trait ChunkPersistenceLayer<ExtraData: OcgExtraData> {
    /// Reliably requests the given coordinates to be loaded. The request should not be forgotten, each chunk coordinate in the request should generate a corresponding response.
    /// Duplicated coordinates or coordinates requested again before a response has been received since the last request for the same coordinate may receive only one response.
    fn request_load(&mut self, coordinates: &[AbsChunkPos]);
    /// Reliably requests the saving of the given chunk data. Data submitted in later requests, or with a higher index in the array takes precedence over older data.
    /// While data is queued for saving in a buffer, if appropriate (i.e. storage is disk and not a network connection), that data should be returned upon request instead of freshly generated data.
    /// Chunk generation layers implementing this interface or non-persistent storage layers can elect to ignore save requests completely.
    fn request_save(&mut self, chunks: Box<[(AbsChunkPos, Chunk<ExtraData>)]>);
    /// Provides up to [`max_count`] resolved chunk loading responses.
    fn try_dequeue_responses(&mut self, max_count: usize) -> Vec<ChunkProviderResult<ExtraData>>;
}

/// An object responsible for managing the presence of voxel chunks in memory via a persistent storage system (disk or network).
pub struct ChunkLoader<ExtraData: OcgExtraData> {
    /// The managed group of chunks, kept private to ensure the loader state can be kept internally consistent.
    managed_group: ChunkGroup<ExtraData>,
    /// Reference to the persistence layer used for loading/saving chunks in the managed group.
    persistence_layer: Box<dyn ChunkPersistenceLayer<ExtraData>>,
}
