// ProjectCodeRegistry — the AUTHORED CodeElement registry for keel's own source (D0148).
//
// Why keel's own code and not a downstream project's: keelArchViewsDoD names SelfSync's registry
// as the verification target, and this repo does not hold another project's data. Re-scoped to
// keel itself, which is real code with real risk rather than a synthetic probe — the alternative
// D0148 explicitly rejected. Every riskClass below is a judgment someone can disagree with, which
// is the point: an authored registry records responsibility, it does not measure the code.
//
// NOT EXHAUSTIVE, and `arch coverage` says so. These are the units whose failure would matter.
package ProjectCodeRegistry {
    private import EngineElement::*;
    private import EngineCodeAudit::*;

    part ceWriteApi : CodeElement {
        :>> id = "4ac8ba12-2293-47a7-b5cf-89dbb157e4b0";
        :>> title = "The write API";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "module";
        :>> filePath = "keel-cli/src/write.rs";
        :>> riskClass = RiskClass::dataLoss;
        :>> invariantSafety = InvariantSafety::enforced;
        :>> stpaRole = StpaRole::actuator;
        :>> designPattern = DesignPattern::service;
        :>> abstractness = "0.2";
    }

    part ceRecordClaim : CodeElement {
        :>> id = "8f4691f6-c2ca-4351-8088-c803fa00918c";
        :>> title = "record_claim";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "function";
        :>> filePath = "keel-cli/src/write.rs";
        :>> riskClass = RiskClass::correctness;
        :>> invariantSafety = InvariantSafety::checked;
        :>> codeHash = "3411fd186054f844";
    }

    part ceSyncIntegration : CodeElement {
        :>> id = "5302bc62-b3df-41ba-b6ec-981817c86261";
        :>> title = "sync/land integration";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "module";
        :>> filePath = "keel-cli/src/sync.rs";
        :>> riskClass = RiskClass::dataLoss;
        :>> invariantSafety = InvariantSafety::checked;
        :>> stpaRole = StpaRole::controller;
        :>> designPattern = DesignPattern::pipeline;
        :>> abstractness = "0.15";
        :>> controlActions = ("fetch", "merge", "gate", "push");
    }

    part ceLandGate : CodeElement {
        :>> id = "be2870ee-dd1a-4792-ad70-3a61e92598cb";
        :>> title = "cmd_land";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "function";
        :>> filePath = "keel-cli/src/sync.rs";
        :>> riskClass = RiskClass::dataLoss;
        :>> invariantSafety = InvariantSafety::enforced;
        :>> codeHash = "6408a8b34e1f45bc";
    }

    part ceGuards : CodeElement {
        :>> id = "deef797e-125b-43b9-b821-75ea29a7e810";
        :>> title = "The forward guards";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "module";
        :>> filePath = "keel-cli/src/guards.rs";
        :>> riskClass = RiskClass::correctness;
        :>> invariantSafety = InvariantSafety::enforced;
        :>> stpaRole = StpaRole::controller;
        :>> designPattern = DesignPattern::strategy;
        :>> abstractness = "0.1";
        :>> controlActions = ("block commit", "warn");
    }

    #Restructure part ceComputedViews : CodeElement {
        :>> id = "6adde151-2ff5-4fad-bf1e-b2a3d75ef179";
        :>> title = "Computed views";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "module";
        :>> filePath = "keel-cli/src/view.rs";
        :>> riskClass = RiskClass::correctness;
        :>> invariantSafety = InvariantSafety::checked;
        :>> stpaRole = StpaRole::sensor;
        :>> designPattern = DesignPattern::repository;
        :>> abstractness = "0.3";
    }

    part ceOrient : CodeElement {
        :>> id = "7028ac61-31e7-4a5e-a9b4-3af4c572e4d7";
        :>> title = "Orientation";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "module";
        :>> filePath = "keel-cli/src/orient.rs";
        :>> riskClass = RiskClass::correctness;
        :>> invariantSafety = InvariantSafety::checked;
        :>> stpaRole = StpaRole::sensor;
        :>> designPattern = DesignPattern::facade;
        :>> abstractness = "0.25";
    }

    part ceClaimHolderRule : CodeElement {
        :>> id = "6bc0c1a6-7a61-4f30-a777-52a140b1d171";
        :>> title = "claims (holder rule)";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "function";
        :>> filePath = "keel-cli/src/claim.rs";
        :>> riskClass = RiskClass::correctness;
        :>> invariantSafety = InvariantSafety::checked;
        :>> codeHash = "143f2ac12c017a51";
    }

    part ceActorProvenance : CodeElement {
        :>> id = "1caf4374-483c-4454-9fa0-3c31ac67e9c5";
        :>> title = "Actor provenance";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "module";
        :>> filePath = "keel-cli/src/actor.rs";
        :>> riskClass = RiskClass::security;
        :>> invariantSafety = InvariantSafety::enforced;
        :>> designPattern = DesignPattern::valueObject;
        :>> abstractness = "0.1";
    }

    part ceMigrate : CodeElement {
        :>> id = "19e1e256-0d3f-4936-a5b4-1e9483ea2de6";
        :>> title = "Gated migration";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "module";
        :>> filePath = "keel-cli/src/migrate.rs";
        :>> riskClass = RiskClass::dataLoss;
        :>> invariantSafety = InvariantSafety::checked;
        :>> designPattern = DesignPattern::pipeline;
        :>> abstractness = "0.1";
    }

    part ceParser : CodeElement {
        :>> id = "eb30999f-c245-452d-8814-25bdbe9e920a";
        :>> title = "SysML parser";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "module";
        :>> filePath = "keel-parser/src/parser.rs";
        :>> riskClass = RiskClass::correctness;
        :>> invariantSafety = InvariantSafety::checked;
        :>> abstractness = "0.15";
    }

    part ceArchViews : CodeElement {
        :>> id = "840b68cd-dae6-4314-8d84-261dadb53e31";
        :>> title = "Arch views";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> kind = "module";
        :>> filePath = "keel-cli/src/arch.rs";
        :>> riskClass = RiskClass::correctness;
        :>> invariantSafety = InvariantSafety::checked;
        :>> designPattern = DesignPattern::facade;
        :>> abstractness = "0.2";
    }

    // Real dependencies among the catalogued units.
    #DependsOn dependency from ceOrient to ceComputedViews;
    #DependsOn dependency from ceClaimHolderRule to ceComputedViews;
    #DependsOn dependency from ceRecordClaim to ceWriteApi;
    #DependsOn dependency from ceGuards to ceComputedViews;
    #DependsOn dependency from ceArchViews to ceComputedViews;
    #DependsOn dependency from ceComputedViews to ceParser;
    #DependsOn dependency from ceSyncIntegration to ceGuards;
    #DependsOn dependency from ceLandGate to ceSyncIntegration;
    #DependsOn dependency from ceMigrate to ceParser;
    #DependsOn dependency from ceWriteApi to ceParser;

    // The gate control loop: guards issue verdicts on what the write API produces, and read
    // the computed views to do it. The #Feedback edge is the return path.
    #Controls dependency from ceGuards to ceWriteApi;
    #Feedback dependency from ceComputedViews to ceGuards;
}
