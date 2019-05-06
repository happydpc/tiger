#[derive(Fail, Debug)]
pub enum StateError {
    #[fail(display = "No document is open")]
    NoDocumentOpen,
    #[fail(display = "Requested document was not found")]
    DocumentNotFound,
    #[fail(display = "Sheet has no export settings")]
    NoExistingExportSettings,
    #[fail(display = "Cannot perform undo operation")]
    UndoOperationNowAllowed,
    #[fail(display = "Requested frame is not in document")]
    FrameNotInDocument,
    #[fail(display = "Requested animation is not in document")]
    AnimationNotInDocument,
    #[fail(display = "Expected a hitbox to be selected")]
    NoHitboxSelected,
    #[fail(display = "A hitbox with this name already exists")]
    HitboxAlreadyExists,
    #[fail(display = "An animation with this name already exists")]
    AnimationAlreadyExists,
    #[fail(display = "Not currently editing any frame")]
    NotEditingAnyFrame,
    #[fail(display = "Not currently editing any animation")]
    NotEditingAnyAnimation,
    #[fail(display = "Frame does not have a hitbox with the requested name")]
    InvalidHitboxName,
    #[fail(display = "Expected an animation frame to be selected")]
    NoAnimationFrameSelected,
    #[fail(display = "Animation does not have a frame at the requested index")]
    InvalidAnimationFrameIndex,
    #[fail(display = "No animation frame found for requested time")]
    NoAnimationFrameForThisTime,
    #[fail(display = "Not currently adjusting export settings")]
    NotExporting,
    #[fail(display = "Not currently renaming an item")]
    NotRenaming,
    #[fail(display = "Not currently adjusting animation frame duration")]
    NotAdjustingAnimationFrameDuration,
    #[fail(display = "Not currently adjusting animation frame position")]
    NotAdjustingAnimationFramePosition,
    #[fail(display = "Not currently adjusting hitbox size")]
    NotAdjustingHitboxSize,
    #[fail(display = "Missing data while adjusting hitbox size")]
    MissingHitboxSizeData,
    #[fail(display = "Not currently adjusting hitbox position")]
    NotAdjustingHitboxPosition,
    #[fail(display = "Missing data while adjusting hitbox position")]
    MissingHitboxPositionData,
}
