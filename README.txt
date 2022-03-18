Meiid UUID generator based on current date for rust


All generated uuid are unique in the convention {####-####-year/month/day}, so the chances of encountering a duplicate ID even in external data is 1 in a billion.
Also, this module is able to generate 1 million "unique" uuid(s) in 60.5secs

This module makes it easy to generate uuid for database table or wherever you might need one

USAGE:
1. initializes Meiid struct
2. generates and returns a unique uuid as string
```rust
use rust::Meiid;
let init = Meiid::new();
let id = init.uuid();
```
id can then be used as you'd like to

ENjoy :)