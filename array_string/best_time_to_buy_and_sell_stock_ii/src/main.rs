use std::i32;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut comportamiento = true; // comprando -> vendiendo
    let mut resultado: i32 = 0;

    let mut aux_index: usize = 0;

    let mut l = 0;

    while l < prices.len() {
        let siguiente = *prices.get(l + 1).unwrap_or_else(|| &0);
        let mut anterior = i32::MAX;

        if l != 0 {
            anterior = *prices.get(l - 1).unwrap_or_else(|| &0);
        }

        match comportamiento {
            true => {
                if l == prices.len() - 1 {
                    // edge case: No se compra en el ultimo index
                    break;
                }

                if prices[l] < anterior && prices[l] < siguiente {
                    comportamiento = false;
                    aux_index = l;
                }
            }
            false => {
                // Vendiendo

                if prices[l] > anterior && prices[l] > siguiente {
                    resultado += prices[l] - prices[aux_index];
                    comportamiento = true;
                }
            }
        };

        l += 1;
    }

    resultado
}

fn main() {
    let prices = vec![7, 6, 4, 3, 1];
    println!("{}", max_profit(prices));
}
