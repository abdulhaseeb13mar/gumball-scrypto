use scrypto::prelude::*;

blueprint! {
    struct GumballMachine {
        gumballs: Vault,
        collected_xrd: Vault,
        price: Decimal,
    }

    impl GumballMachine {

        pub fn instantiate_machine(price: Decimal) -> ComponentAddress {
            let gumballs: Bucket = ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Gumball")
                .metadata("symbol", "GUM")
                .initial_supply(300);

            Self {
                gumballs: Vault::with_bucket(gumballs),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price,
            }
            .instantiate()
            .globalize()
        }

        pub fn buy_gumball(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            info!("Buying gumball");
            let our_share = payment.take(self.price);
            self.collected_xrd.put(our_share);
            let gumball: Bucket = self.gumballs.take(1);
            (gumball,payment)
        }
    }
}
