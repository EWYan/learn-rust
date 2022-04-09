# learn-rust
learn rust notes
- case0

  calculate hash value of a file.  
  
  what i learned:  
  
  #1 how to enable crate feature -> alloc
- case1-ffi

  try to call C function api that include in libc from rust block  
  
  i choosed abs api for test  
  
  search the function signature of abs from <https://en.cppreference.com/w/c/numeric/math/abs>  
  
  "int        abs( int n );"  
  
  so give it a try.
- case2-filerw

  try to read file contents from file  
  
  and write modified contents to new file
- case3-calcrc

  try to calculate large binary file's checksum  

- case4-jsonparser

  try to parser input json files  
- case5-rust file hierarchy

  ref.
  
  https://doc.rust-lang.org/rust-by-example/mod/split.html
  
  https://fasterthanli.me/articles/rust-modules-vs-files
  
  at first i try to create a new project:
  
  $ cargo new --bin case5-filehierarchy
  
  ```sh
  yan@YAN:/mnt/c/d_vault/git_trace/learn-rust$ cargo new --bin case5-filehierarchy
       Created binary (application) `case5-filehierarchy` package
  yan@YAN:/mnt/c/d_vault/git_trace/learn-rust$ cd case5-filehierarchy/
  yan@YAN:/mnt/c/d_vault/git_trace/learn-rust/case5-filehierarchy$ tree .
  .
  ├── Cargo.toml
  └── src
      └── main.rs
  
  1 directory, 2 files
  ```
  
  把默认生成的src文件夹改成其它的名称，如nor-src，运行cargo r报如下错误：
  
  ```sh
  yan@YAN:/mnt/c/d_vault/git_trace/learn-rust/case5-filehierarchy$ cargo r
  error: failed to parse manifest at `/mnt/c/d_vault/git_trace/learn-rust/case5-filehierarchy/Cargo.toml`
  
  Caused by:
    no targets specified in the manifest
    either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
  ```
  
  如log显示，cargo默认只寻找指定的几个文件夹或者section，按照默认就好。
  
  多文件夹/文件的可见性（scope）可以通过关键字mod，pub，super等进行引入，如下文件结构，代码见case5-filehierarchy：
  
  ```
  .
  ├── Cargo.lock
  ├── Cargo.toml
  ├── src
  │   ├── main.rs
  │   └── math
  │       ├── add.rs
  │       └── mod.rs
  ```
  
  
  
  
