use chunked::{Chunker, ChunkedVec};

#[derive(Clone, Copy)]
pub struct SlotIndices {
    collection: u8,
    slot: u32
}

impl SlotIndices {
    pub fn new(collection: usize, slot: usize) -> SlotIndices {
        SlotIndices {
            collection: collection as u8,
            slot: slot as u32
        }
    }

    pub fn invalid() -> SlotIndices {
        SlotIndices {
            collection: u8::max_value(),
            slot: u32::max_value()
        }
    }

    pub fn collection(&self) -> usize {
        self.collection as usize
    }

    pub fn slot(&self) -> usize {
        self.slot as usize
    }
}

pub struct SlotMap {
    entries: ChunkedVec<SlotIndices>,
    free_ids: ChunkedVec<usize>
}

impl SlotMap {
    pub fn new(chunker: Box<Chunker>) -> Self {
        SlotMap {
            entries: ChunkedVec::new(chunker.child("_entries")),
            free_ids: ChunkedVec::new(chunker.child("_free_ids"))
        }
    }

    pub fn allocate_id(&mut self) -> usize {
        match self.free_ids.pop() {
            None => {
                self.entries.push(SlotIndices::invalid());
                self.entries.len() - 1
            },
            Some(free_id) => free_id
        }
    }

    pub fn associate(&mut self, id: usize, new_entry: SlotIndices) {
        let entry = self.entries.at_mut(id);
        entry.clone_from(&new_entry);
    }

    pub fn indices_of(&self, id: usize) -> &SlotIndices {
        self.entries.at(id)
    }

    pub fn free(&mut self, id: usize) {
        self.free_ids.push(id);
    }
}