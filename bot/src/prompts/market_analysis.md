1. MULTI-TIMEFRAME TECHNICAL AND FUNDAMENTAL ANALYSIS CRITERIA
Evaluate the asset based on the provided Daily (Medium-Term) and Weekly (Long-Term) indicators, alongside fundamental data and macroeconomic context.

**INDICATORS PROVIDED:**

- **RSI:** < 30 (Oversold), > 70 (Overbought). In a strong uptrend, RSI can remain > 70 for extended periods.
- **MACD:** Look for crossovers (Line > Signal is bullish) and histogram expansion/contraction.
- **Moving Averages (SMA 50, SMA 200):** Determine the long-term trend (e.g., Price > SMA 200 is bullish).
- **ATR (Average True Range):** Measures market volatility. Higher ATR indicates higher volatility.
- **Historical Volatility:** Measures the degree of variation of a trading price series over time. Higher percentage indicates higher risk.

**FUNDAMENTAL DATA & MACROECONOMIC CONTEXT:**

- **Income Statement, Balance Sheet, Cash Flow:** Analyze quarterly trends in revenue, net income, EPS, assets, liabilities, and cash flow to assess the company's financial health and growth trajectory.
- **GDP, CPI, Fed Funds Rate:** Interpret these macroeconomic indicators to understand the broader economic environment and its potential impact on the asset and sector.

**CONFIDENCE SCORING RUBRIC (STRICT):**

- **90-100%:** Strong alignment across ALL technical timeframes, supported by robust fundamental health, positive economic outlook, and manageable volatility. Clear long-term catalysts.
- **70-89%:** Good alignment across daily and weekly technical timeframes, with supportive fundamental health and economic context. Volatility within acceptable bounds.
- **50-69%:** Mixed technical indicators, or some divergence between timeframes. Fundamental health or economic outlook might be neutral or present minor concerns. Volatility might be moderate to high.
- **<50%:** Contradictory technical signals, weak or deteriorating fundamental health, negative economic outlook, or excessive volatility indicating high risk. Avoid/Exit opportunity.

1. RESPONSE FORMAT (all fields MUST BE FILLED)
You MUST force a "Chain of Thought" by writing your `analysis_reasoning` FIRST. This ensures logical deduction before concluding the opportunity.
Provide your response strictly in JSON format, all fields present in the example are required and have to be present in the response. Do not include markdown blocks outside the JSON:

{
  "analysis_reasoning": "Step 1: Analyze weekly and daily technicals (RSI, MACD, SMAs) to identify primary trend and momentum. Step 2: Evaluate fundamental data (revenue, EPS, balance sheet) for financial health and growth. Step 3: Assess macroeconomic indicators (GDP, CPI, Fed Funds) for broader market context. Step 4: Calculate volatility (ATR, Historical Volatility) to quantify risk. Step 5: Synthesize all factors to determine market regime, opportunity (BUY/SELL/HOLD), confidence score. Step 6: Generate `key_fundamentals` (field is required), `economic_outlook`, `volatility_metrics`, and `suggested_position_size` based on the comprehensive analysis, `confiedce_score` should be in u8 type. For example, if volatility is high but confidence is strong, suggest a smaller position. If volatility is low and confidence is high, suggest a larger position. Position size should be a percentage (e.g., \"5% of portfolio\").",
  "market_regime": "Trending Up / Trending Down / Ranging",
  "opportunity": "BUY" | "SELL" | "HOLD",
  "confidence_score": 65,
  "key_fundamentals": "Summarize key fundamental insights (e.g., 'Strong revenue growth and healthy balance sheet.')",
  "economic_outlook": "Summarize macroeconomic impact (e.g., 'Rising interest rates may pressure growth stocks.')",
  "volatility_metrics": "Summarize volatility (e.g., 'ATR indicates moderate daily price swings.')",
  "suggested_position_size": "Suggest a position size as a percentage (e.g., '3% of portfolio')"
}

