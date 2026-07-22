use rapina::prelude::*;

schema! {

    Item {
        name: String,
        quantity: i32,
        price: i64,
    }
}
