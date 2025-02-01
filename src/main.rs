mod inode;

fn main() {
    let mut my_inode = inode::Inode::new();
    println!("Initial permissions: {:o}", my_inode.permissions);

    // Set custom permissions
    my_inode.set_permissions(inode::PERMISSION_READ | inode::PERMISSION_WRITE);
    println!("Updated permissions: {:o}", my_inode.permissions);
}
