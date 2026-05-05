use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GptMarketAnalysis {
    pub analysis_reasoning: String,
    pub market_regime: String,
    pub opportunity: String,
    pub confidence_score: u8,
}
