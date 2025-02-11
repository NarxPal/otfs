mod inode;
use std::io;

fn main() {
    let mut my_inode = inode::Inode::new();
    println!("Initial permissions: {:o}", my_inode.permissions);

    // Set custom permissions
    my_inode.set_permissions(inode::PERMISSION_READ | inode::PERMISSION_WRITE);
    println!("Updated permissions: {:o}", my_inode.permissions);

    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read file_name");

    let file_name = file_name.trim().to_string();

    let mut fs = inode::Filesystem::new(100, 4096, 50);
    fs.create_file(file_name);
    println!("{:?}", fs.inodes);
}
