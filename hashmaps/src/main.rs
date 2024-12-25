/* -----------------HashMaps----------------------
Hashmaps are data structure that stores key-value pairs.
It is allocated on the heap as it is dynamically sized and can grow and shrink.
it allows for efficient lookp, insertion and deletion of data.
Each key is hashed to a unique index in the underlying array.

All the keys, must be of the same type. And the values must be of the same type.

Requirements of Hashmap Key
----------------------------
ANy type that implements the Eq and Hash traits can be a key in HashMap,
1. bool
2. int,uint
3. String and &str
*/

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("a", 1);
    scores.insert("b", 10);
    scores.insert("c", 143);
    scores.insert("d", 183);

    if scores.contains_key("b") {
        let score = scores["b"];
        scores.remove("b");
        println!("Score is: {}", score);
    }
    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}
