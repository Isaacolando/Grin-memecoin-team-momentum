import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
// src/declarations/memecoin/index.js

export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    balance_of: IDL.Func([IDL.Principal], [IDL.Nat], ["query"]),
    total_supply: IDL.Func([], [IDL.Nat], ["query"]),
    transfer: IDL.Func([IDL.Principal, IDL.Nat], [IDL.Text], []),
    airdrop: IDL.Func([IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat))], [IDL.Text], []),
    add_admin: IDL.Func([IDL.Principal], [IDL.Text], []),
    is_admin: IDL.Func([IDL.Principal], [IDL.Bool], ["query"]),
  });
};

export const canisterId = "YOUR_CANISTER_ID";
