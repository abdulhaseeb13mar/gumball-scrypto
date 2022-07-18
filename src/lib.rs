use scrypto::prelude::*;

blueprint! {
    struct GumballMachine {
        gumballs: Vault,
        collected_xrd: Vault,
        price: Decimal,
        admin_badge: ResourceAddress,
    }

    impl GumballMachine {

        pub fn instantiate_machine(price: Decimal) -> (ComponentAddress, Bucket) {


            let gumballs: Bucket = ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Gumball")
                .metadata("symbol", "GUM")
                .initial_supply(300);

            let admin_badge: Bucket = ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Gumball Machine Admin")
                .initial_supply(1);

            let component = Self {
                gumballs: Vault::with_bucket(gumballs),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price,
                admin_badge: admin_badge.resource_address(),
            }
            .instantiate()
            .globalize();

            (component, admin_badge)
        }

        pub fn buy_gumball(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            info!("Buying gumball");
            let our_share = payment.take(self.price);
            self.collected_xrd.put(our_share);

            let gumball: Bucket = self.gumballs.take(1);
            (gumball,payment)
        }

        pub fn withdraw_xrd(&mut self, auth: Proof) -> Bucket {
            assert_eq!(auth.resource_address(), self.admin_badge, "You are not the admin");
            self.collected_xrd.take_all()
        }
    }
}
