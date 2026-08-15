// =====================================================================
// ProjectVerifyBackfill — the #Verify edges backfilled for issue087 (dcBackfillVerifyEdges).
//
// WHAT THESE EDGES ARE. The `verification-trace` guard found 77 pairs where a PASSED DoD names a
// SystemRequirement in its evidence while no #Verify edge reaches that requirement — so
// `tier-satisfaction` reported the requirement unverified even though its work is done and passing.
// The prose claim already existed in every one of these; the edge makes it TRAVERSABLE, which is
// the engine's whole premise (a claim that lives only in prose is not state).
//
// HOW EACH WAS JUDGED, and why the rule ended up inverted. The first pass tried to detect the claim
// by its phrasing — "Delivers srX", "Satisfies srX", the DoD's opening subject — and kept
// under-detecting, because an enumerated "(1) srX: <what was built>" is exactly as much a claim as
// the verb form. Chasing phrasings was the wrong shape. The guard had already established that the
// DoD names the requirement in its EVIDENCE; that is a claim however it is worded. So the question
// became the opposite one: does the DoD DISCLAIM it — deferred, excluded, not delivered, out of
// scope? Only those, plus what the current thread knows first-hand, are held back.
//
// FOUR PAIRS ARE DELIBERATELY ABSENT, and their requirements stay honestly unverified:
//   srDcEnrollment          <- dcEnrollmentDoD: role is not recorded; `Actor` has no role attribute
//                              (issue110, D0146 proposed). The requirement demands kind AND role.
//   srDcAuthorityFromRegistry <- dcMechanizeOwnershipDoD / storyOwnershipDoD: kind is checked and
//                              refused, role is not, for the same reason.
//   srDcPerActorWriteTargets <- dcMechanizeOwnershipDoD: not delivered at all — sprint 293 built the
//                              ownership guard and did not route writes to per-actor targets.
// Those four are the ones this thread built and therefore knows to be partial. Authoring their edges
// would have been the easy way to reach a clean number, and would have made the metric a lie.
// =====================================================================
package ProjectVerifyBackfill {
    private import EngineRelationships::*;

    #Verify dependency from dcActivationManifestDoD to srPortActivationManifest;
    #Verify dependency from dcAuthorityQueueDoD to srDcHumanAuthorityQueue;
    #Verify dependency from dcContentionAdjudicationDoD to srDcContentionAdjudication;
    #Verify dependency from dcDeclaredMarkersDoD to srDcDeclaredMarkerVocabulary;
    #Verify dependency from dcLandLoopDoD to srDcGateOnMergedTree;
    #Verify dependency from dcLandLoopDoD to srDcSyncAwareOrient;
    #Verify dependency from dcUnresolvedEvidenceDoD to srDcUnresolvedEvidenceClass;
    #Verify dependency from storyActivationDoD to srPortActivationManifest;
    #Verify dependency from storyArchitectSureInteractionsDoD to srViewerAuthoringSurface;
    #Verify dependency from storyArchitectSureInteractionsDoD to srViewerContextActions;
    #Verify dependency from storyArchitectSureInteractionsDoD to srViewerNavHistory;
    #Verify dependency from storyArchitectSureInteractionsDoD to srViewerSearchDiscovery;
    #Verify dependency from storyAuthorityQueueDoD to srDcHumanAuthorityQueue;
    #Verify dependency from storyContentionDoD to srDcContentionAdjudication;
    #Verify dependency from storyEncodingEngineDoD to srEncodingOverride;
    #Verify dependency from storyEncodingEngineDoD to srEncodingRender;
    #Verify dependency from storyEncodingEngineDoD to srEncodingSpec;
    #Verify dependency from storyEncodingOverrideDoD to srEncodingModular;
    #Verify dependency from storyEncodingOverrideDoD to srEncodingOverride;
    #Verify dependency from storyEncodingOverrideDoD to srEncodingRender;
    #Verify dependency from storyEncodingOverrideDoD to srEncodingSchemaSemantics;
    #Verify dependency from storyEncodingOverrideDoD to srEncodingSpec;
    #Verify dependency from storyEncodingSemanticsDoD to srEncodingSchemaSemantics;
    #Verify dependency from storyEncodingSemanticsDoD to srEncodingSpec;
    #Verify dependency from storyKeelViewerArchitectureDoD to srViewerKeelApi;
    #Verify dependency from storyKeelViewerCritiqueDoD to srViewerInProgramEdit;
    #Verify dependency from storyKeelViewerCritiqueDoD to srViewerIterativeCritique;
    #Verify dependency from storyKeelViewerGenUiDoD to srViewerGenerativeUi;
    #Verify dependency from storyKeelViewerGenUiDoD to srViewerSchemaApi;
    #Verify dependency from storyLandLoopDoD to srDcGateOnMergedTree;
    #Verify dependency from storyLandLoopDoD to srDcSyncAwareOrient;
    #Verify dependency from storyModularEncodingDoD to srEncodingModular;
    #Verify dependency from storyScopeAwareAuthoringDoD to srViewerRelationshipGrammar;
    #Verify dependency from storyScopeAwareAuthoringDoD to srViewerScopedAuthoring;
    #Verify dependency from storyServeCorsDoD to srViewerKeelApi;
    #Verify dependency from storyServeDiscArchDoD to srServeOutputSchemaValidate;
    #Verify dependency from storyServeDiscArchDoD to srServeScopedContext;
    #Verify dependency from storyServeTier2aDoD to srServeOutputSchemaValidate;
    #Verify dependency from storyUnresolvedEvidenceDoD to srDcUnresolvedEvidenceClass;
    #Verify dependency from storyViewerAppMvpDoD to srViewerGenerativeUi;
    #Verify dependency from storyViewerAppMvpDoD to srViewerGraphExpand;
    #Verify dependency from storyViewerBaselineCompareDoD to srViewerBaselineCompare;
    #Verify dependency from storyViewerChangeImpactDoD to srViewerChangeImpact;
    #Verify dependency from storyViewerCreateItemDoD to srViewerAuthoringSurface;
    #Verify dependency from storyViewerCreateLinkageDoD to srViewerCreateLinkage;
    #Verify dependency from storyViewerCreateLinkageDoD to srViewerRelationshipGrammar;
    #Verify dependency from storyViewerCritiquePlanDoD to srViewerIterativeCritique;
    #Verify dependency from storyViewerEditItemDoD to srViewerAuthoringSurface;
    #Verify dependency from storyViewerExportShareDoD to srViewerExportShare;
    #Verify dependency from storyViewerFoundationCloseDoD to srViewerGenerativeUi;
    #Verify dependency from storyViewerFoundationCloseDoD to srViewerGraphExpand;
    #Verify dependency from storyViewerFoundationCloseDoD to srViewerKeelApi;
    #Verify dependency from storyViewerFoundationCloseDoD to srViewerVisualViewpoint;
    #Verify dependency from storyViewerInProgEditDoD to srViewerInProgramEdit;
    #Verify dependency from storyViewerInProgramEditDoD to srViewerInProgramEdit;
    #Verify dependency from storyViewerIterativeCritiqueDoD to srViewerIterativeCritique;
    #Verify dependency from storyViewerSchemaApiDoD to srViewerSchemaApi;
    #Verify dependency from storyViewerSystemBoundViewsDoD to srViewerSystemBoundViews;
    #Verify dependency from viewerAuthoringEndpointsDoD to srViewerAuthoringSurface;
    #Verify dependency from viewerBaselineCompareDoD to srViewerBaselineCompare;
    #Verify dependency from viewerChangeImpactDoD to srViewerChangeImpact;
    #Verify dependency from viewerCreateLinkageDoD to srViewerCreateLinkage;
    #Verify dependency from viewerCreateLinkageDoD to srViewerRelationshipGrammar;
    #Verify dependency from viewerEditItemDoD to srViewerAuthoringSurface;
    #Verify dependency from viewerExportShareDoD to srViewerExportShare;
    #Verify dependency from viewerGenerativeUiDoD to srViewerGenerativeUi;
    #Verify dependency from viewerGraphExpandDoD to srViewerGraphExpand;
    #Verify dependency from viewerInProgramEditDoD to srViewerInProgramEdit;
    #Verify dependency from viewerIterativeCritiqueDoD to srViewerIterativeCritique;
    #Verify dependency from viewerKeelApiDoD to srViewerKeelApi;
    #Verify dependency from viewerSchemaApiDoD to srViewerSchemaApi;
    #Verify dependency from viewerSystemBoundViewsDoD to srViewerSystemBoundViews;
    #Verify dependency from viewerVisualViewpointDoD to srViewerVisualViewpoint;
    #Verify dependency from storyPortabilityDoD to srPortCatalogExchange;
    #Verify dependency from storyPortabilityDoD to srPortModularProcessUnit;
    #Verify dependency from keelProcessPortabilityDoD to srPortCatalogExchange;
    #Verify dependency from keelProcessPortabilityDoD to srPortModularProcessUnit;
}
