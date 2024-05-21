import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}


// src/App.js
import React from "react";
import { BrowserRouter as Router, Route, Switch, Link } from "react-router-dom";
import Balance from "./components/Balance";
import Transfer from "./components/Transfer";
import Admin from "./components/Admin";

const App = () => {
  return (
    <Router>
      <nav>
        <ul>
          <li><Link to="/">Balance</Link></li>
          <li><Link to="/transfer">Transfer</Link></li>
          <li><Link to="/admin">Admin</Link></li>
        </ul>
      </nav>
      <Switch>
        <Route exact path="/" component={Balance} />
        <Route path="/transfer" component={Transfer} />
        <Route path="/admin" component={Admin} />
      </Switch>
    </Router>
  );
};

export default App;
