import { HttpAgent, Actor } from '@dfinity/agent';
import { idlFactory } from './idl';

const agent = new HttpAgent();
const canisterId = 'your-canister-id';
const actor = Actor.createActor(idlFactory, { agent, canisterId });

async function transfer() {
    const recipient = document.getElementById('recipient').value;
    const amount = parseInt(document.getElementById('amount').value, 10);
    if (recipient && amount) {
        try {
            await actor.transfer(recipient, amount);
            alert('Transfer successful!');
        } catch (error) {
            alert('Transfer failed: ' + error);
        }
    } else {
        alert('Please provide a valid recipient and amount.');
    }
}
document.addEventListener('DOMContentLoaded', () => {
    console.log('Website is fully loaded');
    
});
async function buy() {
    try {
        document.getElementById('buy')
        const amount = await actor.amount_to_buy(await agent.getPrinciple());
        await actor.buy(amount);
    } catch (error) {
        alert('Failed to buy: ' + error);
    }
}

async function getBalance() {
    try {
        const balance = await actor.balance_of(await agent.getPrincipal());
        document.getElementById('balance').innerText = `Your balance: ${balance}`;
    } catch (error) {
        alert('Failed to get balance: ' + error);
    }
}

document.addEventListener('DOMContentLoaded', () => {
    getBalance();
});
