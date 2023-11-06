use std::path::PathBuf;

use floem::views::VirtualListVector;

use lapce_rpc::file::{FileNodeItem, FileNodeViewData};

pub struct FileNodeVirtualList {
    file_node_item: FileNodeItem,
    rename_path: Option<PathBuf>,
}

impl FileNodeVirtualList {
    pub fn new(file_node_item: FileNodeItem, rename_path: Option<PathBuf>) -> Self {
        Self {
            file_node_item,
            rename_path,
        }
    }
}

impl VirtualListVector<FileNodeViewData> for FileNodeVirtualList {
    type ItemIterator = Box<dyn Iterator<Item = FileNodeViewData>>;

    fn total_len(&self) -> usize {
        self.file_node_item.children_open_count
    }

    fn slice(&mut self, range: std::ops::Range<usize>) -> Self::ItemIterator {
        let min = range.start;
        let max = range.end;
        let mut i = 0;
        let mut view_items = Vec::new();
        for item in self.file_node_item.sorted_children() {
            i = item.append_view_slice(
                &mut view_items,
                self.rename_path.as_deref(),
                min,
                max,
                i + 1,
                0,
            );
            if i > max {
                return Box::new(view_items.into_iter());
            }
        }

        Box::new(view_items.into_iter())
    }
}
