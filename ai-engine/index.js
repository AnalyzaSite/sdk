const express = require("express");
const app = express();

app.use(express.json());

function calculateFairnessScore(wallet, txHistory) {
  let score = 100;

  if (txHistory.length > 10) score -= 30;
  if (wallet.includes("bot")) score -= 50;

  return Math.max(score, 0);
}

app.post("/verify", (req, res) => {
  const { wallet, txHistory } = req.body;

  const score = calculateFairnessScore(wallet, txHistory);

  res.json({
    wallet,
    fairnessScore: score,
    eligible: score >= 70
  });
});

app.listen(3001, () => {
  console.log("Analyza AI Engine running on port 3001");
});
