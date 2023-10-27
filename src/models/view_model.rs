/// ViewModel trait for mapping database models to viewmodels for the API
pub trait ViewModel {
    type Model;
    /// Maps the database model to a viewmodel
    fn to_viewmodel(&self) -> Self::Model;
}
