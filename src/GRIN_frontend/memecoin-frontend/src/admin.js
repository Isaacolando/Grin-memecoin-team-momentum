// src/components/Admin.js
import React, { useState } from "react";
import { memecoinActor } from "../icpService";
import { Principal } from "@dfinity/principal";

const Admin = () => {
  const [newAdminId, setNewAdminId] = useState("");
  const [message, setMessage] = useState("");

  const addAdmin = async () => {
    try {
      const newAdmin = Principal.fromText(newAdminId);
      const result = await memecoinActor.add_admin(newAdmin);
      setMessage(result);
    } catch (error) {
      console.error("Error adding admin:", error);
    }
  };

  return (
    <div>
      <h2>Add Admin</h2>
      <input
        type="text"
        placeholder="New Admin Principal ID"
        value={newAdminId}
        onChange={(e) => setNewAdminId(e.target.value)}
      />
      <button onClick={addAdmin}>Add Admin</button>
      {message && <p>{message}</p>}
    </div>
  );
};

export default Admin;
