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

// store metadata of entire filesystem
pub struct SuperBlock {
    pub allocated_inodes: Vec<usize>, // capture all the inode number here
    pub total_inodes: usize,
    pub free_inodes: usize,
    pub block_size: usize,
    pub free_blocks: Vec<usize>,
}

// link file name to inode number
struct Directory {
    files: HashMap<String, usize>, // Maps file names to inode numbers
}

struct Filesystem {
    superblock: SuperBlock,                 // Tracks total/free inodes & blocks
    inodes: Vec<Inode>,                     // Stores all inodes
    directories: HashMap<usize, Directory>, // Maps inode numbers to directories
    data_blocks: Vec<Vec<u8>>,              // Simulates file storage (heap memory)
}

// using octal values(0o) here
pub const PERMISSION_READ: u16 = 0o400; // Owner read
pub const PERMISSION_WRITE: u16 = 0o200; // Owner write
pub const PERMISSION_EXECUTE: u16 = 0o100; // Owner execute
pub const PERMISSION_READ_OTHER: u16 = 0o004; // Others read
pub const PERMISSION_WRITE_OTHER: u16 = 0o002; // Others write
pub const PERMISSION_EXECUTE_OTHER: u16 = 0o001; // Others execute

static mut INODE_COUNTER: usize = 1;

impl Inode {
    pub fn new() -> Self {
        unsafe {
            let inode_number = INODE_COUNTER;
            INODE_COUNTER + 1;

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
        Directory {
            files: HashMap::new(),
        }
    }

    fn add_file(&mut self, filename: String, inode_number: usize) {
        self.files.insert(filename, inode_number);
    }

    fn remove_file(&mut self, filename: &str) {
        self.files.remove(filename);
    }

    fn lookup_file(&self, filename: &str) -> Option<usize> {
        self.files.get(filename).copied()
    }
}
