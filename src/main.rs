fn main() {
    println!("Hello, world!");
}
pub fn get_paths<'a>(
    list_of_pools: &'a Vec<Pool>,
    token_in: &str,
    token_out: &str,
) -> Vec<Path<'a>> {
    let mut list_of_paths: Vec<Path> = vec![];
    for pool_to_start in list_of_pools {
        let mut path = Path::new();
        let mut pools_to_iterate: Vec<&Pool> = list_of_pools.iter().collect();
        if pool_to_start.token_00 != token_in && pool_to_start.token_01 != token_in {
            continue;
        }
        if let Some(index) = pools_to_iterate
            .iter()
            .position(|x| x.pool_address == pool_to_start.pool_address)
        {
            pools_to_iterate.remove(index);
            path.add_pool(pool_to_start);
        }
        if pool_to_start.token_00 != token_out && pool_to_start.token_01 != token_out {
            loop {
                let next_token = get_next_token(pool_to_start, token_in);
                if let Some(next_pool) = take_pool_from_list(&pools_to_iterate, next_token) {
                    if let Some(index) = pools_to_iterate
                        .iter()
                        .position(|x| x.pool_address == next_pool.pool_address)
                    {
                        let new_pool_to_add = pools_to_iterate.remove(index);
                        path.add_pool(new_pool_to_add);
                        if new_pool_to_add.token_00 == token_out
                            || new_pool_to_add.token_01 == token_out
                        {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }

        list_of_paths.push(path);
    }
    list_of_paths
}
pub fn get_next_token<'a>(pool: &'a Pool, token_in: &str) -> &'a str {
    if pool.token_00 == token_in {
        pool.token_01.as_str()
    } else {
        pool.token_00.as_str()
    }
}
pub fn take_pool_from_list<'a>(
    pools_to_iterate: &'a Vec<&Pool>,
    token_in: &str,
) -> Option<&'a Pool> {
    pools_to_iterate
        .iter()
        .find(|x| x.token_00 == token_in || x.token_01 == token_in)
        .map(|x| *x)
}

pub struct Path<'a> {
    pools: Vec<&'a Pool>,
}
impl<'a> Path<'a> {
    pub fn new() -> Self {
        Self { pools: vec![] }
    }
    pub fn add_pool(&mut self, pool: &'a Pool) {
        self.pools.push(pool);
    }
}
pub struct Pool {
    pool_address: i64,
    token_00: String,
    token_01: String,
    token_00_balance: i64,
    token_01_balance: i64,
}
impl Pool {
    pub fn new(
        pool_address: i64,
        token_00: String,
        token_01: String,
        token_00_balance: i64,
        token_01_balance: i64,
    ) -> Self {
        Self {
            pool_address,
            token_00,
            token_01,
            token_00_balance,
            token_01_balance,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_paths, Pool};

    #[test]
    fn first_test() {
        let list_of_pools: Vec<Pool> = vec![
            Pool::new(1, "ETH".to_string(), "USDC".to_string(), 1, 1700),
            Pool::new(2, "ETH".to_string(), "EUR".to_string(), 1, 1600),
            Pool::new(3, "BTC".to_string(), "USDC".to_string(), 1, 26000),
            Pool::new(4, "EUR".to_string(), "USDC".to_string(), 100, 130),
            Pool::new(5, "BTC".to_string(), "EUR".to_string(), 1, 2400),
            Pool::new(5, "EUR".to_string(), "DAI".to_string(), 1, 2400),
            Pool::new(5, "USDC".to_string(), "DAI".to_string(), 1, 2400),
        ];
        let paths = get_paths(&list_of_pools, "ETH", "DAI");
        assert_eq!(paths.len(), 2);
    }
}
