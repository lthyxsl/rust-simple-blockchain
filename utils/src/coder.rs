use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

pub fn my_serialize<T>(o: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(o).unwrap()
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(&data);
    hasher.result_str()
}

#[cfg(test)]
mod tests {

    use super::*;

    // 测试序列化与反序列化
    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_serialize() {
        let p = Point { x: 1, y: 2 };
        let se = my_serialize(&p);
        let de: Point = my_deserialize(&se);
        assert_eq!(p, de);
    }
}
