// src/icpService.js
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "./declarations/memecoin";
import { Principal } from "@dfinity/principal";

const canisterId = "YOUR_CANISTER_ID";

const agent = new HttpAgent();
agent.fetchRootKey(); // Remove this line for production

export const createActor = () => {
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });
};

export const memecoinActor = createActor();
