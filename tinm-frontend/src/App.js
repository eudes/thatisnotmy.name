import logo from './logo.svg';
import './App.css';
import {Routes, Route} from "react-router-dom";
import {TTS} from "./routes/tts";
import {Home} from "./routes/home";
import log from 'loglevel';


function App() {
  if (!process.env.NODE_ENV || process.env.NODE_ENV === 'development') {
    log.setLevel(log.levels.DEBUG)
  } else {
    log.setLevel(log.levels.INFO)
  }

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo"/>
          <Routes>
            <Route path="/" element={<Home/>}/>
            <Route exact path='/:language/:country/:word' element={<TTS/>}/>
          </Routes>
      </header>
    </div>
  );
}

export default App;
