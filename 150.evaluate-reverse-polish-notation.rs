// @leet start
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut tmp: Vec<i32> = Vec::new();

        for s in tokens.into_iter() {
            match s.parse::<i32>() {
                Ok(num) => {
                    tmp.push(num);
                }
                Err(_) => {
                    if let [.., a, b] = tmp.as_slice() {
                        let result = match s.as_str() {
                            "+" => a.checked_add(*b),
                            "-" => a.checked_sub(*b),
                            "*" => a.checked_mul(*b),
                            "/" => a.checked_div(*b),
                            _ => None,
                        };

                        if let Some(res) = result {
                            tmp.truncate(tmp.len() - 2);
                            tmp.push(res)
                        }
                    }
                }
            }
        }

        *tmp.first().unwrap()
    }
}
// @leet end

