import { useState } from "react";
import axios from "axios";

export default function Home() {
  const [wallet, setWallet] = useState("");
  const [result, setResult] = useState(null);

  const verifyWallet = async () => {
    const response = await axios.post("http://localhost:3001/verify", {
      wallet,
      txHistory: []
    });

    setResult(response.data);
  };

  return (
    <div style={{ padding: 40 }}>
      <h1>Analyza Fair Mint</h1>

      <input
        placeholder="Enter Wallet Address"
        value={wallet}
        onChange={(e) => setWallet(e.target.value)}
      />

      <button onClick={verifyWallet}>Check Eligibility</button>

      {result && (
        <div>
          <p>Fairness Score: {result.fairnessScore}</p>
          <p>Eligible: {result.eligible ? "Yes" : "No"}</p>
        </div>
      )}
    </div>
  );
}
