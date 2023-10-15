use crate::models::account_model::{AccountDbModel, AccountViewModel};
use crate::models::printfile_model::{PrintFileDbModel, PrintFileViewModel};

pub fn printfile_to_viewmodel(printfile: PrintFileDbModel) -> PrintFileViewModel {
    PrintFileViewModel {
        uuid: printfile.uuid,
        user_uuid: printfile.user_uuid,
        name: printfile.name,
        size: printfile.size,
        checksum: printfile.checksum,
        file_type: printfile.file_type,
        file_storage_type: printfile.file_storage_type,
        created_at: printfile.created_at,
    }
}

pub fn account_to_viewmodel(account: AccountDbModel) -> AccountViewModel {
    AccountViewModel {
        uuid: account.uuid,
        email: account.email,
        username: account.username,
    }
}
