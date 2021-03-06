use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as VerifiableCreds {
        SubjectCount: u32;
        Subjects: map u32 => T::AccountId;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_subject(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let subject = <SubjectCount<T>>::get();

            <SubjectCount<T>>::put(subject + 1);
            <Subjects<T>>::insert(subject, sender);

            Ok(())
        }
    }
}