struct MultiBuffer {
    /// A snapshot of the [`Excerpt`]s in the MultiBuffer.
    /// Use [`MultiBuffer::snapshot`] to get a up-to-date snapshot.
    snapshot: RefCell<MultiBufferSnapshot>,
    /// Contains the state of the buffers being edited
    buffers: RefCell<HashMap<BufferId, BufferState>>,
    subscriptions: Topic,
    /// If true, the multi-buffer only contains a single [`Buffer`] and a single [`Excerpt`]
    singleton: bool,
    replica_id: ReplicaId,
    history: History,
    title: Option<String>,
    capability: Capability,
}

/// The contents of a [`MultiBuffer`] at a single point in time.
#[derive(Clone, Default)]
pub struct MultiBufferSnapshot {
    singleton: bool,
    excerpts: SumTree<Excerpt>,
    initial_buffer_paths: BTreeMap<BufferId, Option<Arc<Path>>>,
    trailing_excerpt_update_count: usize,
    non_text_state_update_count: usize,
    edit_count: usize,
    is_dirty: bool,
    has_conflict: bool,
    show_headers: bool,
}

// Excerpts are ordered by (Option<Arc<Path>>, BufferId, Range<Anchor>)
// Note that ranges are always disjoint.

/// A slice into a [`Buffer`] that is being edited in a [`MultiBuffer`].
#[derive(Clone)]
struct Excerpt {
    /// The buffer being excerpted
    buffer_id: BufferId,
    /// The path of the excerpts buffer when the first excerpt for this buffer was inserted.
    initial_buffer_path: Option<Arc<Path>>,
    /// A snapshot of the buffer being excerpted
    buffer: BufferSnapshot,
    /// The range of the buffer to be shown in the excerpt
    range: ExcerptRange<text::Anchor>,
    /// The last row in the excerpted slice of the buffer
    max_buffer_row: BufferRow,
    /// A summary of the text in the excerpt
    text_summary: TextSummary,
    has_trailing_newline: bool,
}

impl MultiBuffer {
    /// Inserts excerpts into the buffer (in managed mode only).
    /// Excerpts are ordered by buffer path. If buffers don't have a path they are ordered first, sorted by order of creation (entity id).
    /// If a path changes causes excerpts to be reordered, this multibuffer will emit a Reordered event.
    /// When this happens, you'll need to sort any ordered collections of anchors that depend on anchors being ordered.
    pub fn insert_excerpts(
        &mut self,
        new_excerpts: impl IntoIterator<(Model<Buffer>, Range<Anchor>)>,
    ) {

        //
    }

    /// Remove excerpts whose range intersects with the range of the given anchors.
    pub fn remove_excerpts(
        &mut self,
        removed_excerpts: impl IntoIterator<(Model<Buffer>, Range<Anchor>)>,
    ) {
        //
    }
}
