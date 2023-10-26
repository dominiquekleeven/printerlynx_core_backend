pub trait ViewModel {
    type Model;
    fn to_viewmodel(&self) -> Self::Model;
}
