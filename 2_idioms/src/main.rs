use derive_more::Sub;
use std::cmp::Reverse;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;

type AmountOfProducts = u8;
const MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND: AmountOfProducts = 10;

const INITIAL_AMOUNT_OF_MONEY: [Coin; 15] = [
    Coin::Fifty,
    Coin::Twenty,
    Coin::Twenty,
    Coin::Ten,
    Coin::Ten,
    Coin::Five,
    Coin::Five,
    Coin::Two,
    Coin::Two,
    Coin::Two,
    Coin::Two,
    Coin::One,
    Coin::One,
    Coin::One,
    Coin::One,
];

type AmountOfMoneyPrimitive = u64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Sub)]
pub struct AmountOfMoney(AmountOfMoneyPrimitive);

pub type ProductName = &'static str;

#[derive(Debug)]
pub struct ProductInfo {
    name: ProductName,
    price: AmountOfMoney,
    amount: AmountOfProducts,
}

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

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Coin {
    One = 1,
    Two = 2,
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}

pub struct SuccessfulPurchase {
    pub product: Product,
    pub change: Vec<Coin>,
    pub vending_machine: VendingMachine<Idle>,
}

#[derive(Error, Debug)]
pub enum PurchaseError {
    #[error("not enough money: expected {expected.0}, got {got.0}")]
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
    #[error("cannot give rest")]
    CannotGiveRest {
        vending_machine: VendingMachine<InProcess>,
    },
}

pub struct ResetResult {
    pub vending_machine: VendingMachine<Idle>,
    pub refund: Vec<Coin>,
}

impl From<&[Coin]> for AmountOfMoney {
    fn from(value: &[Coin]) -> Self {
        AmountOfMoney(value.iter().map(|c| *c as AmountOfMoneyPrimitive).sum())
    }
}

#[derive(Debug)]
pub struct Idle;

#[derive(Debug)]
pub struct InProcess(Vec<Coin>);

#[derive(Debug)]
pub struct VendingMachine<S> {
    products: HashMap<ProductName, ProductInfo>,
    earned_money: Vec<Coin>,
    state: S,
}

impl Default for VendingMachine<Idle> {
    fn default() -> Self {
        Self {
            state: Idle,
            earned_money: Vec::from(INITIAL_AMOUNT_OF_MONEY),
            products: HashMap::from([
                (
                    Product::KitKat.as_str(),
                    ProductInfo {
                        name: Product::KitKat.as_str(),
                        price: AmountOfMoney(35),
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Oreo.as_str(),
                    ProductInfo {
                        name: Product::Oreo.as_str(),
                        price: AmountOfMoney(45),
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Lays.as_str(),
                    ProductInfo {
                        name: Product::Lays.as_str(),
                        price: AmountOfMoney(50),
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Doritos.as_str(),
                    ProductInfo {
                        name: Product::Doritos.as_str(),
                        price: AmountOfMoney(50),
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::CocaCola.as_str(),
                    ProductInfo {
                        name: Product::CocaCola.as_str(),
                        price: AmountOfMoney(60),
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Pepsi.as_str(),
                    ProductInfo {
                        name: Product::Pepsi.as_str(),
                        price: AmountOfMoney(60),
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Water.as_str(),
                    ProductInfo {
                        name: Product::Water.as_str(),
                        price: AmountOfMoney(20),
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
            ]),
        }
    }
}

impl<T> VendingMachine<T> {
    /// 93
    /// amount_of_50s = get_50s
    ///
    /// if amount_of_50s > 0:
    ///    n = int(93 / 50)
    ///    if n > amount_of_50s:
    ///
    ///
    fn take_money_and_give_change(
        &mut self,
        inserted_coins: Vec<Coin>,
        expected_rest: AmountOfMoney,
    ) {
        let mut available_coins = Coin::iter()
            .map(|c| c as AmountOfMoneyPrimitive)
            .collect::<Vec<_>>();
        available_coins.sort_by_key(|c| Reverse(*c));

        for coin in available_coins {}
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
        let inserted_money = AmountOfMoney::from(&*self.state.0);

        if inserted_money < product_price {
            return Err(PurchaseError::NotEnoughMoney {
                expected: product_price,
                got: inserted_money,
                vending_machine: self,
            });
        }

        let expected_rest = inserted_money - product_price;

        todo!()
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
}

fn main() {
    println!("Implement me!");
}
