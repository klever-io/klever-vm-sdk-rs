mod dns_proxy {
    use klever_sc::imports::*;

    #[klever_sc::proxy]
    pub trait Dns {
        #[payable("KLV")]
        #[endpoint]
        fn register(&self, name: &ManagedBuffer);
    }
}

use klever_sc::imports::*;

/// Standard smart contract module that deals with registering usernames in a DNS contract.
///
/// Klever usernames/herotags need to be requested by the beneficiary.
/// For a contract, this means that they need an endpoint via which to request a username from the DNS.
///
#[klever_sc::module]
pub trait DnsModule {
    #[proxy]
    fn dns_proxy(&self, to: ManagedAddress) -> dns_proxy::Proxy<Self::Api>;

    #[payable("KLV")]
    #[only_owner]
    #[endpoint(dnsRegister)]
    fn dns_register(&self, dns_address: ManagedAddress, name: ManagedBuffer) {
        let payment = self.call_value().klv_value().clone_value();
        self.dns_proxy(dns_address)
            .register(&name)
            .with_klv_transfer(payment)
            .execute_on_dest_context::<IgnoreValue>();
    }
}
