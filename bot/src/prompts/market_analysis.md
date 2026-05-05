1. MULTI-TIMEFRAME TECHNICAL ANALYSIS CRITERIA
Evaluate the asset based on the provided Hourly (Short-Term) and Daily (Long-Term) indicators.

**INDICATORS PROVIDED:**
- **RSI:** < 30 (Oversold), > 70 (Overbought). In a strong uptrend, RSI can remain > 70 for extended periods.
- **MACD:** Look for crossovers (Line > Signal is bullish) and histogram expansion/contraction.
- **Moving Averages (SMA 50, SMA 200):** Determine the long-term trend (e.g., Price > SMA 200 is bullish).

**CONFIDENCE SCORING RUBRIC (STRICT):**
- **90-100%:** Perfect alignment across BOTH hourly and daily timeframes, supported by strong fundamental news.
- **70-89%:** Most indicators align with the primary trend on the daily timeframe; supportive news.
- **50-69%:** Mixed indicators, relying mostly on short-term momentum or a single strong signal.
- **<50%:** Contradictory signals across timeframes, highly uncertain choppy market.

2. RESPONSE FORMAT
You MUST force a "Chain of Thought" by writing your `analysis_reasoning` FIRST. This ensures logical deduction before concluding the opportunity.
Provide your response strictly in JSON format. Do not include markdown blocks outside the JSON:

{
  "analysis_reasoning": "Step 1: The daily timeframe shows a strong uptrend (Price > SMA 200) but is currently overbought (RSI > 75). Step 2: The hourly timeframe shows bearish divergence on the MACD and dropping volume. Step 3: Recent news is positive but already priced in. Conclusion: Waiting for a pullback before entering.",
  "market_regime": "Trending Up / Trending Down / Ranging",
  "opportunity": "BUY" | "SELL" | "HOLD",
  "confidence_score": 65
}