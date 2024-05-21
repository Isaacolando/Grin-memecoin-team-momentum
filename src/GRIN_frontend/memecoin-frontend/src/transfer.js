// src/components/Transfer.js
import React, { useState } from "react";
import { memecoinActor } from "../icpService";
import { Principal } from "@dfinity/principal";

const Transfer = () => {
  const [toPrincipalId, setToPrincipalId] = useState("");
  const [amount, setAmount] = useState("");
  const [message, setMessage] = useState("");

  const transferTokens = async () => {
    try {
      const toPrincipal = Principal.fromText(toPrincipalId);
      const result = await memecoinActor.transfer(toPrincipal, BigInt(amount));
      setMessage(result);
    } catch (error) {
      console.error("Error transferring tokens:", error);
    }
  };

  return (
    <div>
      <h2>Transfer Tokens</h2>
      <input
        type="text"
        placeholder="To Principal ID"
        value={toPrincipalId}
        onChange={(e) => setToPrincipalId(e.target.value)}
      />
      <input
        type="text"
        placeholder="Amount"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
      />
      <button onClick={transferTokens}>Transfer</button>
      {message && <p>{message}</p>}
    </div>
  );
};

export default Transfer;
