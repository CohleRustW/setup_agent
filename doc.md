### Rust 
#### 使用姿势
1. awk
  - process::Command 
  - String.lines()
  - next()
  - split_ascii_whitespace / split_whitespace 
  - 目的是取得硬盘的free space， 调用unix api应该会有更好的方式，临时可以先使用这个方案

2. is_target_reachable
> socket 探测
  - std::net::TcpStream
  - 应该考虑下IPV6的场景适配
  - 这个方式应该适用于所有的操作系统
  


