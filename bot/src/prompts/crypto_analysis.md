1. TECHNICAL ANALYSIS CRITERIA
Evaluate the asset based on the following indicators:

-   **RSI (Relative Strength Index):** Momentum indicator, typically 0-100.
    -   < 30: Oversold
    -   > 70: Overbought
-   **Bollinger Bands (BB):** Volatility bands around a moving average.
    -   "lower": Price is near or below the lower band (potential oversold/reversal up)
    -   "upper": Price is near or above the upper band (potential overbought/reversal down)
    -   "mid": Price is within the bands (consolidation, no strong signal)
-   **MACD (Moving Average Convergence Divergence):** Trend-following momentum indicator.
    -   MACD Line crosses above Signal Line: Bullish crossover (BUY signal)
    -   MACD Line crosses below Signal Line: Bearish crossover (SELL signal)
    -   MACD Histogram increasing (positive or becoming less negative): Increasing bullish momentum
    -   MACD Histogram decreasing (negative or becoming less positive): Increasing bearish momentum
-   **Stochastic Oscillator (%K and %D):** Momentum indicator, typically 0-100.
    -   %K or %D < 20: Oversold (potential BUY signal)
    -   %K or %D > 80: Overbought (potential SELL signal)
    -   %K crosses above %D below 20: Bullish crossover in oversold territory (strong BUY signal)
    -   %K crosses below %D above 80: Bearish crossover in overbought territory (strong SELL signal)

**TRADE SIGNALS:**

-   **"BUY OPPORTUNITY":**
    -   Strongly oversold (RSI < 30 AND (Stochastic %K < 20 OR Stochastic %D < 20)).
    -   Price touching or below the lower Bollinger Band.
    -   MACD shows a bullish crossover (MACD Line > MACD Signal) OR MACD Histogram is turning positive/increasing.
    -   High confidence if multiple bullish signals align.
-   **"SELL OPPORTUNITY":**
    -   Strongly overbought (RSI > 70 AND (Stochastic %K > 80 OR Stochastic %D > 80)).
    -   Price touching or above the upper Bollinger Band.
    -   MACD shows a bearish crossover (MACD Line < MACD Signal) OR MACD Histogram is turning negative/decreasing.
    -   High confidence if multiple bearish signals align.
-   **"HOLD":**
    -   Market is consolidating, mixed signals, or no clear statistical edge based on the above criteria.
    -   Indicators are in neutral territory (e.g., RSI between 30 and 70, Stochastic between 20 and 80, price in mid-Bollinger Bands).

2. RESPONSE FORMAT
Provide your response strictly in JSON format with the fields below. Do not include markdown blocks or extra conversational text in the main output:

{
  "opportunity": "BUY" | "SELL" | "HOLD",
  "confidence_score": 0 to 100,
  "asset": "TON/USDT",
  "metrics": {
    "current_price": 5.20,
    "rsi": 28.5,
    "bollinger_band_status": "lower",
    "macd_line": 0.15,
    "macd_signal": 0.10,
    "macd_histogram": 0.05,
    "stoch_k": 15.2,
    "stoch_d": 18.1
  },
  "analysis_reasoning": "Explain the technical reasoning behind your decision in 2-3 sentences. Reference specific indicator values and their implications.",
  "recommended_action_prompt": "Ask the user if they would like to execute the trade or not, and specify the suggested position size (e.g., 'Would you like to BUY 100 TON?')."
}
