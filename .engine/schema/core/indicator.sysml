// =====================================================================
// EngineIndicator — monitored MEASURES (D0089; grounded in ISO/IEC/IEEE 15939 + PSM + OMG SMM
// measurement information model). An Indicator is a measure that INFORMS a decision/need (a leading
// indicator, D0088) — watched by DIRECTION, not gated by a threshold (a threshold-gated measure is a
// requirement). Its data arrives by a measurement METHOD: computed (objective, repo-derived), pulled
// (objective, external API/scraper), or manual (subjective, e.g. a survey). A Measurement is one
// datapoint (value @ time + provenance): computed indicators compute their series on demand (no
// stored Measurements, compute-don't-store); pulled/manual indicators RECORD irreducible point-in-time
// observations (a market-share or survey value can't be recomputed from the repo). An Indicator is
// NOT a requirement (no verify/satisfy expected; excluded from the assurance/orphan required-edge sets).
// =====================================================================
package EngineIndicator {
    doc /* Monitored measures (D0089). An Indicator informs by direction (D0088); data arrives via a computed/pulled/manual method; a Measurement is one recorded datapoint with provenance. */

    private import ScalarValues::*;
    private import EngineElement::*;

    enum def IndicatorGoal { minimize; maximize; observe; }       // which direction is "better" (observe = neither)
    enum def MeasurementMethod { computed; pulled; manual; }      // objective-repo / objective-external / subjective

    part def Indicator :> Element {
        attribute measures : String;              // what is monitored (one line)
        attribute goal : IndicatorGoal;           // direction-aware status: improving iff moving the goal way
        attribute unit : String[0..1];            // e.g. "%", "points/sprint", "documents", "NPS"
        attribute method : MeasurementMethod;     // how the data arrives
        attribute collectionRef : String[0..1];   // computed: a report/metric binding; pulled: a collection skill/command; manual: the procedure
    }

    part def Measurement :> Element {
        attribute value : String;                 // the datapoint value (numeric or categorical; String for generality)
        attribute measuredAt : Timestamp;          // the observation time (ISO-8601) — the irreducible "when"
        attribute source : String[0..1];          // provenance: API/URL/tool for pulled; the survey/run for manual
    }
}
