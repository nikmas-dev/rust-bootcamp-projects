use std::collections::HashMap;
use thiserror::Error;

type AmountOfProducts = u8;
const MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND: AmountOfProducts = 10;

const INITIAL_AMOUNT_OF_MONEY: AmountOfMoney = 100;

pub type AmountOfMoney = u64;
pub type ProductName = &'static str;
pub type ProductPrice = AmountOfMoney;

#[derive(Debug)]
pub struct ProductInfo {
    name: ProductName,
    price: ProductPrice,
}

#[derive(Debug)]
struct InnerFullProduct {
    info: ProductInfo,
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

#[derive(Debug)]
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
    pub rest: Vec<Coin>,
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
}

#[derive(Debug)]
struct Idle;

#[derive(Debug)]
struct InProcess(Vec<Coin>);

#[derive(Debug)]
pub struct VendingMachine<S> {
    products: HashMap<ProductName, InnerFullProduct>,
    earned_money: AmountOfMoney,
    state: S,
}

impl Default for VendingMachine<Idle> {
    fn default() -> Self {
        Self {
            state: Idle,
            earned_money: INITIAL_AMOUNT_OF_MONEY,
            products: HashMap::from([
                (
                    Product::KitKat.as_str(),
                    InnerFullProduct {
                        info: ProductInfo {
                            name: Product::KitKat.as_str(),
                            price: 35,
                        },
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Oreo.as_str(),
                    InnerFullProduct {
                        info: ProductInfo {
                            name: Product::Oreo.as_str(),
                            price: 45,
                        },
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Lays.as_str(),
                    InnerFullProduct {
                        info: ProductInfo {
                            name: Product::Lays.as_str(),
                            price: 50,
                        },
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Doritos.as_str(),
                    InnerFullProduct {
                        info: ProductInfo {
                            name: Product::Doritos.as_str(),
                            price: 50,
                        },
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::CocaCola.as_str(),
                    InnerFullProduct {
                        info: ProductInfo {
                            name: Product::CocaCola.as_str(),
                            price: 60,
                        },
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Pepsi.as_str(),
                    InnerFullProduct {
                        info: ProductInfo {
                            name: Product::Pepsi.as_str(),
                            price: 60,
                        },
                        amount: MAX_AMOUNT_OF_PRODUCTS_OF_ONE_KIND,
                    },
                ),
                (
                    Product::Water.as_str(),
                    InnerFullProduct {
                        info: ProductInfo {
                            name: Product::Water.as_str(),
                            price: 20,
                        },
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

        // if full_product.amount == 0 {
        //     return Err(PurchaseError::ProductNotAvailable(product.as_str()));
        // }

        todo!()
    }
}

fn main() {
    println!("Implement me!");
}
