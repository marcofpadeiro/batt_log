import './App.css';
import Sidebar from './components/Sidebar/Sidebar';
import SessionChart from './components/SessionChart/SessionChart';
import React, { useEffect, useState } from 'react';

function App() {
    const [sessions, setSessions] = useState([]);
    const [selectedSessionId, setSelectedSessionId] = useState(null);
    const [chartData, setChartData] = useState([]);

    useEffect(() => {
        fetch('http://127.0.0.1:8080/sessions')
            .then(response => response.json())
            .then(setSessions)
            .catch(error => console.error('Error fetching sessions: ', error));
    }, []);

    useEffect(() => {
        if (selectedSessionId) {
            fetch(`http://127.0.0.1:8080/sessions/${selectedSessionId}/events`)
                .then(response => response.json())
                .then(setChartData)
                .catch(error => console.error(`Error fetching data for session ${selectedSessionId}: `, error));
        }
    }, [selectedSessionId]);

    const handleCardClick = (sessionId) => {
        setSelectedSessionId(sessionId);
    };

    return (
        <div className="App">
            <SessionChart data={chartData} />
            <Sidebar sessions={sessions} onCardClick={handleCardClick} />
        </div>
    );
}

export default App;

