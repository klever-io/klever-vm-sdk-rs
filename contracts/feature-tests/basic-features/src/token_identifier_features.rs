use klever_sc::imports::*;

#[klever_sc::module]
pub trait TokenIdentifierFeatures {
    #[endpoint]
    fn token_identifier_klv(&self) -> TokenIdentifier {
        TokenIdentifier::klv()
    }

    #[endpoint]
    fn token_identifier_is_valid_1(&self, token_id: TokenIdentifier) -> bool {
        token_id.is_valid()
    }

    #[endpoint]
    fn token_identifier_is_valid_2(&self, bytes: ManagedBuffer) -> bool {
        TokenIdentifier::from(bytes).is_valid_kda_identifier()
    }
}
