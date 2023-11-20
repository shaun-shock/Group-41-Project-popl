# Group-41-Project-popl


Certainly! Here's the entire README in a single block for easy copying:

```markdown
# Simple File System in Rust

This project implements a simple file system with indexed allocation in Rust. It provides a virtual disk to store files and includes a library linked with an application.

## Running the Program

To run the program, follow these steps:

1. **Create and Format Virtual Disk:**
   ```bash
   $ ./create_format <FILENAME> <SIZE>
   ```

2. **Run the Application:**
   ```bash
   $ ./app <FILENAME>
   ```

## How to Use

### `create_format_vdisk`

```rust
use simple_file_system::create_format_vdisk;

// Create and format a virtual disk
// Returns 0 on success, -1 on error
let result = create_format_vdisk("vdiskname", m);
```

### `sfs_mount`

```rust
use simple_file_system::sfs_mount;

// Mount the file system
// Returns 0 on success, -1 on error
let result = sfs_mount("vdiskname");
```

### `sfs_umount`

```rust
use simple_file_system::sfs_umount;

// Unmount the file system
// Returns 0 on success, -1 on error
let result = sfs_umount();
```

### `sfs_create`

```rust
use simple_file_system::sfs_create;

// Create a new file
// Returns 0 on success, -1 on error
let result = sfs_create("filename");
```

### Performance Comparison

As part of our project, we are converting this codebase to Rust and comparing its performance with the original implementation. We have currently incorporated the mount, umount, vdisk, and sfs_create functions.

For detailed information on the Rust implementation and performance comparison, refer to the project documentation.

## Contributing

If you'd like to contribute to the project, feel free to open issues or pull requests on our GitHub repository. We welcome any suggestions, improvements, or bug fixes.

## License

This project is licensed under the [MIT License](LICENSE).
```

Feel free to copy and use this single-window README template for your Rust project.
