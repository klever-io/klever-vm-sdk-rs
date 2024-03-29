pub fn contract_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct ContractObj<A>
        where
            A: klever_sc::api::VMApi,
        {
            _phantom: core::marker::PhantomData<A>,
        }
    }
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
    quote! {
        impl<A> klever_sc::contract_base::ContractBase for ContractObj<A>
        where
            A: klever_sc::api::VMApi,
        {
            type Api = A;
        }
    }
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn contract_obj<A>() -> ContractObj<A>
        where
            A: klever_sc::api::VMApi,
        {
            ContractObj {
                _phantom: core::marker::PhantomData,
            }
        }

        pub struct ContractBuilder;

        impl klever_sc::contract_base::CallableContractBuilder for self::ContractBuilder {
            fn new_contract_obj<A: klever_sc::api::VMApi + Send + Sync>(
                &self,
            ) -> klever_sc::types::heap::Box<dyn klever_sc::contract_base::CallableContract> {
                klever_sc::types::heap::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
    }
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
    quote! {
        impl<A> AutoImpl for ContractObj<A> where
            A: klever_sc::contract_base::ContractBase
                + klever_sc::api::ErrorApi
                + klever_sc::api::EndpointArgumentApi
                + klever_sc::api::EndpointFinishApi
                + klever_sc::api::ManagedTypeApi
        {
        }
    }
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
    quote! {
        impl<A> klever_sc::contract_base::CallableContract for ContractObj<A>
        where
            A: klever_sc::api::VMApi + Send + Sync,
        {
            fn call(&self, fn_name: &str) -> bool {
                EndpointWrappers::call(self, fn_name)
            }
        }
    }
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct Proxy<A>
        where
            A: klever_sc::api::VMApi + 'static,
        {
            pub address: klever_sc::types::ManagedOption<A, klever_sc::types::ManagedAddress<A>>,
        }

        impl<A> klever_sc::contract_base::ProxyObjBase for Proxy<A>
        where
            A: klever_sc::api::VMApi + 'static,
        {
            type Api = A;

            fn new_proxy_obj() -> Self {
                Proxy {
                    address: klever_sc::types::ManagedOption::none(),
                }
            }

            fn contract(mut self, address: klever_sc::types::ManagedAddress<Self::Api>) -> Self {
                self.address = klever_sc::types::ManagedOption::some(address);
                self
            }

            fn extract_opt_address(
                &mut self,
            ) -> klever_sc::types::ManagedOption<
                Self::Api,
                klever_sc::types::ManagedAddress<Self::Api>,
            > {
                core::mem::replace(&mut self.address, klever_sc::types::ManagedOption::none())
            }

            fn extract_address(&mut self) -> klever_sc::types::ManagedAddress<Self::Api> {
                self.extract_opt_address().unwrap_or_sc_panic(klever_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET)
            }
        }
    }
}

pub fn call_method_api_static_init() -> proc_macro2::TokenStream {
    quote! {
        <Self::Api as klever_sc::api::VMApi>::init_static();
    }
}
