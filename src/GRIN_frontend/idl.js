export const idlFactory = ({ IDL }) => {
    return IDL.Service({
        'init': IDL.Func([IDL.Text, IDL.Text, IDL.Nat], [], []),
        'transfer': IDL.Func([IDL.Principal, IDL.Nat], [IDL.Result], []),
        'balance_of': IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
        'total_supply': IDL.Func([], [IDL.Nat], ['query']),
        'airdrop': IDL.Func([IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat))], [IDL.Result], []),
    });
};
