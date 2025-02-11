use std::collections::HashMap;

// each file/dir needs an inode
// Inode struct store metadata of file/dir
pub struct Inode {
    pub inode_number: usize, // unique id given to each metadata
    pub size: u64,
    pub permissions: u16,
    pub timestamp: u64,
    // data_blocks is a pointer to in-memory file content being stored into memory data blocks
    pub data_blocks: Vec<usize>,
}

// store how many data blocks, inodes available or used in fs
pub struct SuperBlock {
    pub allocated_inodes: Vec<usize>, // capture all the inode number here
    pub total_inodes: usize,
    pub free_inodes: usize,
    pub block_size: usize,
    pub free_blocks: Vec<usize>,
}

// link file name to inode number(id)
struct Directory {
    files: HashMap<String, usize>, // Maps file names to inode numbers
}

pub struct Filesystem {
    superblock: SuperBlock,                 // Tracks total/free inodes & blocks
    pub inodes: Vec<Inode>,                 // Stores all inodes
    directories: HashMap<usize, Directory>, // Maps inode numbers to directories
    data_blocks: Vec<Vec<u8>>,              // Simulates file storage (heap memory)
}

// using octal values(0o) here
pub const PERMISSION_READ: u16 = 0o400; // Owner read
pub const PERMISSION_WRITE: u16 = 0o200; // Owner write
pub const PERMISSION_EXECUTE: u16 = 0o100; // Owner execute
pub const PERMISSION_READ_OTHER: u16 = 0o004; // Others read
                                              // pub const PERMISSION_WRITE_OTHER: u16 = 0o002; // Others write
                                              // pub const PERMISSION_EXECUTE_OTHER: u16 = 0o001; // Others execute

static mut INODE_COUNTER: usize = 1;

impl Inode {
    pub fn new() -> Self {
        unsafe {
            let inode_number = INODE_COUNTER;
            INODE_COUNTER += 1;

            Inode {
                inode_number,
                size: 0,
                permissions: PERMISSION_READ
                    | PERMISSION_WRITE
                    | PERMISSION_EXECUTE
                    | PERMISSION_READ_OTHER, // Default permissions
                timestamp: 0,
                data_blocks: Vec::new(),
            }
        }
    }

    pub fn set_permissions(&mut self, perms: u16) {
        self.permissions = perms;
    }
}

impl Directory {
    fn new() -> Self {
        // initialize empty directory(without filename,id)
        Directory {
            files: HashMap::new(),
        }
    }

    // insert filename and inode number into hashmap
    fn add_file(&mut self, filename: String, inode_number: usize) {
        self.files.insert(filename, inode_number);
    }

    fn remove_file(&mut self, filename: &str) {
        self.files.remove(filename);
    }

    fn lookup_file(&self, filename: &str) -> Option<usize> {
        self.files.get(filename).copied() // .get will return reference value, so copied is used to convert &usize to  usize
    }
}

impl SuperBlock {
    fn new(total_inodes: usize, block_size: usize, total_blocks: usize) -> Self {
        SuperBlock {
            allocated_inodes: Vec::new(),
            total_inodes,
            free_inodes: total_inodes,
            block_size,
            free_blocks: (0..total_blocks).collect(), // Initialize all blocks as free
        }
    }
}

impl Filesystem {
    pub fn new(total_inodes: usize, block_size: usize, total_blocks: usize) -> Self {
        Filesystem {
            superblock: SuperBlock {
                allocated_inodes: Vec::new(),
                total_inodes,
                free_inodes: total_inodes,
                block_size,
                free_blocks: (0..total_blocks).collect(), // Initialize all blocks as free, using range expression here
            },
            inodes: Vec::new(),
            directories: HashMap::new(),
            data_blocks: vec![vec![0; block_size]; total_blocks], // it creates 2d vector
        }
    }

    pub fn create_file(&mut self, filename: String) -> Option<usize> {
        if self.superblock.free_inodes == 0 {
            return None;
        }

        let inode_num = self.superblock.allocated_inodes.len() + 1;
        // Inode is pub
        let inode = Inode {
            inode_number: inode_num,
            size: 0,
            permissions: PERMISSION_READ | PERMISSION_WRITE,
            timestamp: 0,
            data_blocks: Vec::new(),
        };

        self.inodes.push(inode);
        self.superblock.allocated_inodes.push(inode_num);

        // doing freeinodes -1 since freeinodes = to total inodes, so free_inodes is subtracted for each inode creation
        self.superblock.free_inodes -= 1;

        self.directories
            .entry(0)
            .or_insert_with(Directory::new)
            .add_file(filename, inode_num);

        Some(inode_num) // return inode number
    }
}
