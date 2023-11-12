pub enum IntegrationState {
    NotStarted,
    PrsCreated,
    InstallingChild,
    ChangesValidated,
    ChildMerged,
    InstallingChildPostMerge,
    ParentChildUpdated,
    ParentMerged,
}
