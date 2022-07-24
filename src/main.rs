use chrono::{DateTime, Utc};

pub struct ItemMetaData {
    item_last_modify_date: String,
    item_is_dir: bool,
    item_size: u64,
}

impl std::fmt::Debug for ItemMetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ItemMetaData")
            .field("item_last_modify_date", &self.item_last_modify_date)
            .field("item_is_dir", &self.item_is_dir)
            .field("item_size", &self.item_size)
            .finish()
    }
}

impl ItemMetaData {
    pub fn new(item_last_modify_date: String, item_is_dir: bool, item_size: u64) -> ItemMetaData {
        ItemMetaData {
            item_last_modify_date,
            item_is_dir,
            item_size,
        }
    }
}

#[derive(Debug)]
pub struct DirectoryInfo {
    pub name: String,
    pub meta: Option<ItemMetaData>,
    pub children: Vec<Box<DirectoryInfo>>,
}

impl DirectoryInfo {
    pub fn new(new_child_name: &str, meta: Option<ItemMetaData>) -> DirectoryInfo {
        DirectoryInfo {
            meta,
            name: new_child_name.to_string(),
            children: Vec::<Box<DirectoryInfo>>::new(),
        }
    }
}

fn build_tree(path: &str, dirlist: &mut DirectoryInfo) {
    let path_list = std::fs::read_dir(path).unwrap();
    let mut cvec: Vec<Box<DirectoryInfo>> = Vec::new();

    path_list.into_iter().for_each(|each_item| {
        let each_item_unwrapped = each_item.unwrap();
        let item_meta = each_item_unwrapped.metadata().unwrap();
        let current_filename = each_item_unwrapped.file_name().into_string().unwrap();
        let datetime: DateTime<Utc> = item_meta.modified().unwrap().into();
        let last_modified = datetime.format("%d/%m/%Y %T").to_string();
        let item_len = item_meta.len().try_into().unwrap();

        match item_meta.is_file() {
            true => cvec.push(Box::new(DirectoryInfo {
                name: current_filename.clone(),
                meta: Some(ItemMetaData::new(last_modified, false, item_len)),
                children: Vec::new(),
            })),
            false => {
                let mut newdir = DirectoryInfo::new(
                    current_filename.as_ref(),
                    Some(ItemMetaData::new(last_modified, true, item_len)),
                );
                build_tree(
                    format!("{}/{}", path, current_filename).as_ref(),
                    &mut newdir,
                );
                cvec.push(Box::new(newdir));
            }
        }
    });

    dirlist.children = cvec;
}

fn main() {
    let init_path = "/home/isaac/Desktop";
    let init_path_meta = std::fs::metadata(init_path).unwrap();
    let datetime: DateTime<Utc> = init_path_meta.modified().unwrap().into();
    let last_modified = datetime.format("%d/%m/%Y %T").to_string();
    let item_len = init_path_meta.len();
    let mut map_list = DirectoryInfo::new(
        init_path,
        Some(ItemMetaData::new(last_modified, true, item_len)),
    );

    build_tree(init_path, &mut map_list);

    println!("{:?}", map_list);
}
