# Rust中的异步

- 当一个future还没开始poll的时候（await），可以自由的移动；但当其开始poll了，就必须pin住，若其成员都是Unpin类型，就可以安全获得其可变借用；