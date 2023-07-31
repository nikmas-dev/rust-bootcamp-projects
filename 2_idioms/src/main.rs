use derive_more::Sub;
use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::iter;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;

type AmountOfProducts = u8;
const MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND: AmountOfProducts = 10;

type AmountOfMoney = u64;

pub type ProductName = &'static str;

#[derive(Debug)]
pub struct ProductInfo {
    name: ProductName,
    price: AmountOfMoney,
    amount: AmountOfProducts,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Product {
    KitKat,
    Oreo,
    Lays,
    Doritos,
    CocaCola,
    Pepsi,
    Water,
}

impl Product {
    fn as_str(&self) -> &'static str {
        match self {
            Self::KitKat => "KitKat",
            Self::Oreo => "Oreo",
            Self::Lays => "Lays",
            Self::Doritos => "Doritos",
            Self::CocaCola => "Coca-Cola",
            Self::Pepsi => "Pepsi",
            Self::Water => "Water",
        }
    }
}

type AmountOfCoins = u64;

#[derive(Debug, Copy, Clone, EnumIter, Eq, PartialEq, Hash)]
pub enum Coin {
    One = 1,
    Two = 2,
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}

#[derive(Debug)]
pub struct SuccessfulPurchase {
    pub product: Product,
    pub change: Vec<Coin>,
    pub vending_machine: VendingMachine<Idle>,
}

#[derive(Error, Debug)]
pub enum PurchaseError {
    #[error("not enough money: expected {expected}, got {got}")]
    NotEnoughMoney {
        expected: AmountOfMoney,
        got: AmountOfMoney,
        vending_machine: VendingMachine<InProcess>,
    },
    #[error("product {product} is not available")]
    ProductNotAvailable {
        product: ProductName,
        vending_machine: VendingMachine<InProcess>,
    },
    #[error("cannot give change")]
    CannotGiveChange {
        vending_machine: VendingMachine<InProcess>,
    },
}

pub type Change = Vec<Coin>;

pub struct ResetResult {
    pub vending_machine: VendingMachine<Idle>,
    pub refund: Vec<Coin>,
}

fn get_total_amount_from_coins(coins: &[Coin]) -> AmountOfMoney {
    coins.iter().map(|c| *c as AmountOfMoney).sum()
}

#[derive(Debug)]
pub struct Idle;

#[derive(Debug)]
pub struct InProcess(Vec<Coin>);

#[derive(Debug)]
pub struct VendingMachine<S> {
    products: HashMap<ProductName, ProductInfo>,
    earned_money: HashMap<Coin, AmountOfCoins>,
    state: S,
}

impl Default for VendingMachine<Idle> {
    fn default() -> Self {
        Self {
            state: Idle,
            earned_money: HashMap::from([
                (Coin::Fifty, 2),
                (Coin::Twenty, 2),
                (Coin::Ten, 2),
                (Coin::Five, 2),
                (Coin::Two, 4),
                (Coin::One, 4),
            ]),
            products: HashMap::from([
                (
                    Product::KitKat.as_str(),
                    ProductInfo {
                        name: Product::KitKat.as_str(),
                        price: 35,
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Oreo.as_str(),
                    ProductInfo {
                        name: Product::Oreo.as_str(),
                        price: 45,
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Lays.as_str(),
                    ProductInfo {
                        name: Product::Lays.as_str(),
                        price: 50,
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Doritos.as_str(),
                    ProductInfo {
                        name: Product::Doritos.as_str(),
                        price: 50,
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::CocaCola.as_str(),
                    ProductInfo {
                        name: Product::CocaCola.as_str(),
                        price: 60,
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Pepsi.as_str(),
                    ProductInfo {
                        name: Product::Pepsi.as_str(),
                        price: 60,
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Water.as_str(),
                    ProductInfo {
                        name: Product::Water.as_str(),
                        price: 20,
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
            ]),
        }
    }
}

impl VendingMachine<Idle> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_coins(self, coins: Vec<Coin>) -> VendingMachine<InProcess> {
        VendingMachine {
            state: InProcess(coins),
            products: self.products,
            earned_money: self.earned_money,
        }
    }
}

impl VendingMachine<InProcess> {
    pub fn get_product(mut self, product: Product) -> Result<SuccessfulPurchase, PurchaseError> {
        let full_product = self.products.get_mut(product.as_str()).unwrap();

        if full_product.amount == 0 {
            return Err(PurchaseError::ProductNotAvailable {
                product: product.as_str(),
                vending_machine: self,
            });
        }

        let product_price = full_product.price;
        let inserted_money = get_total_amount_from_coins(&self.state.0);

        if inserted_money < product_price {
            return Err(PurchaseError::NotEnoughMoney {
                expected: product_price,
                got: inserted_money,
                vending_machine: self,
            });
        }

        let expected_change = inserted_money - product_price;

        self.take_money_and_give_change(product, expected_change)
    }

    pub fn insert_coins(&mut self, coins: Vec<Coin>) {
        self.state.0.extend(coins);
    }

    pub fn reset(self) -> ResetResult {
        ResetResult {
            vending_machine: VendingMachine {
                state: Idle,
                products: self.products,
                earned_money: self.earned_money,
            },
            refund: self.state.0,
        }
    }

    fn take_money_and_give_change(
        mut self,
        product: Product,
        expected_change: AmountOfMoney,
    ) -> Result<SuccessfulPurchase, PurchaseError> {
        let mut inserted_coins_map = HashMap::<Coin, AmountOfCoins>::new();

        for coin in &self.state.0 {
            *inserted_coins_map.entry(*coin).or_default() += 1;
        }

        for (coin, amount) in &inserted_coins_map {
            self.earned_money
                .entry(*coin)
                .and_modify(|c| *c += amount)
                .or_insert(*amount);
        }

        let mut available_coin_denominations = Coin::iter().collect::<Vec<_>>();
        available_coin_denominations.sort_by_key(|c| Reverse(*c as AmountOfMoney));

        let mut change = Vec::new();
        let mut left_change = expected_change;

        for denomination in available_coin_denominations {
            let required_amount_of_coins_of_current_denomination =
                left_change / denomination as AmountOfMoney;

            match required_amount_of_coins_of_current_denomination.cmp(&0) {
                Ordering::Equal => {
                    continue;
                }
                Ordering::Greater => {
                    let available_amount_of_coins_of_current_denomination =
                        self.earned_money.get_mut(&denomination);

                    match available_amount_of_coins_of_current_denomination {
                        Some(available_amount_of_coins_of_current_denomination) => {
                            let amount_of_coins_to_give = std::cmp::min(
                                *available_amount_of_coins_of_current_denomination,
                                required_amount_of_coins_of_current_denomination,
                            );

                            *available_amount_of_coins_of_current_denomination -=
                                amount_of_coins_to_give;

                            left_change -= amount_of_coins_to_give * denomination as AmountOfMoney;

                            change.extend(
                                iter::repeat(denomination).take(amount_of_coins_to_give as usize),
                            );
                        }
                        None => continue,
                    }
                }
                Ordering::Less => unreachable!(),
            }
        }

        if left_change > 0 {
            for (coin, amount) in inserted_coins_map {
                *self.earned_money.get_mut(&coin).unwrap() -= amount;
            }

            return Err(PurchaseError::CannotGiveChange {
                vending_machine: self,
            });
        }

        self.products.get_mut(product.as_str()).unwrap().amount -= 1;

        Ok(SuccessfulPurchase {
            product,
            change,
            vending_machine: VendingMachine {
                state: Idle,
                products: self.products,
                earned_money: self.earned_money,
            },
        })
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_total_amount_of_coins_from_map(map: &HashMap<Coin, AmountOfCoins>) -> AmountOfMoney {
        map.iter()
            .map(|(coin, amount)| *coin as AmountOfMoney * *amount)
            .sum()
    }

    #[test]
    fn should_successfully_return_product_and_change() {
        let vending_machine = VendingMachine::default();

        let initial_amount_of_money =
            get_total_amount_of_coins_from_map(&vending_machine.earned_money);

        let vending_machine = vending_machine.insert_coins(vec![Coin::Fifty, Coin::Twenty]);
        let SuccessfulPurchase {
            product,
            change,
            vending_machine,
        } = vending_machine.get_product(Product::CocaCola).unwrap();
        assert_eq!(change, vec![Coin::Ten]);
        assert_eq!(product, Product::CocaCola);
        assert_eq!(
            vending_machine
                .products
                .get(Product::CocaCola.as_str())
                .unwrap()
                .amount,
            9
        );

        let final_amount_of_money =
            get_total_amount_of_coins_from_map(&vending_machine.earned_money);

        assert_eq!(final_amount_of_money - initial_amount_of_money, 60);
    }

    #[test]
    fn should_return_error_when_not_enough_coins_were_inserted() {
        let vending_machine = VendingMachine::default();

        let initial_amount_of_money =
            get_total_amount_of_coins_from_map(&vending_machine.earned_money);

        let vending_machine = vending_machine.insert_coins(vec![Coin::Fifty]);
        let error = vending_machine.get_product(Product::CocaCola).unwrap_err();

        match error {
            PurchaseError::NotEnoughMoney {
                expected,
                got,
                vending_machine,
            } => {
                assert_eq!(expected, 60);
                assert_eq!(got, 50);

                assert_eq!(
                    vending_machine
                        .products
                        .get(Product::CocaCola.as_str())
                        .unwrap()
                        .amount,
                    10
                );

                let final_amount_of_money =
                    get_total_amount_of_coins_from_map(&vending_machine.earned_money);

                assert_eq!(initial_amount_of_money, final_amount_of_money);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_return_error_when_product_is_not_available() {
        let mut vending_machine = VendingMachine::default();

        let initial_amount_of_money =
            get_total_amount_of_coins_from_map(&vending_machine.earned_money);

        vending_machine
            .products
            .get_mut(Product::CocaCola.as_str())
            .unwrap()
            .amount = 0;

        let vending_machine = vending_machine.insert_coins(vec![Coin::Fifty, Coin::Ten]);
        let error = vending_machine.get_product(Product::CocaCola).unwrap_err();

        match error {
            PurchaseError::ProductNotAvailable {
                product,
                vending_machine,
            } => {
                assert_eq!(product, Product::CocaCola.as_str());

                assert_eq!(
                    vending_machine
                        .products
                        .get(Product::CocaCola.as_str())
                        .unwrap()
                        .amount,
                    0
                );

                let final_amount_of_money =
                    get_total_amount_of_coins_from_map(&vending_machine.earned_money);

                assert_eq!(initial_amount_of_money, final_amount_of_money);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn should_return_error_when_cannot_give_change() {
        let vending_machine = VendingMachine {
            earned_money: HashMap::from([(Coin::Twenty, 1)]),
            ..Default::default()
        };

        let initial_amount_of_money =
            get_total_amount_of_coins_from_map(&vending_machine.earned_money);

        let vending_machine = vending_machine.insert_coins(vec![Coin::Fifty, Coin::Twenty]);
        let error = vending_machine.get_product(Product::CocaCola).unwrap_err();

        match error {
            PurchaseError::CannotGiveChange { vending_machine } => {
                let ResetResult {
                    vending_machine,
                    refund,
                } = vending_machine.reset();

                assert_eq!(
                    vending_machine
                        .products
                        .get(Product::CocaCola.as_str())
                        .unwrap()
                        .amount,
                    10
                );

                let final_amount_of_money =
                    get_total_amount_of_coins_from_map(&vending_machine.earned_money);

                assert_eq!(initial_amount_of_money, final_amount_of_money);

                assert_eq!(refund, vec![Coin::Fifty, Coin::Twenty]);
            }
            _ => unreachable!(),
        }
    }
}
