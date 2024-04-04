import CapacityWithDelta from './CapacityWithDelta';
import '../../styles/Sidebar.css'

function Element({ session, onClick }) {
    const options = {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: 'numeric',
        minute: 'numeric',
        second: 'numeric',
    };

    let duration = secondsToDuration(session.duration);
    let start = new Date(session.start).toLocaleDateString('en-UK', options)
    let avgPower = (session.average_power_consumption / 1000000).toFixed(2);

    return (
        <div className="card" onClick={onClick} >
            <div className="card-section" id="type">{session.session_type}</div>
            <div className="card-section" id="start-date">{start}</div>
            <div className="card-section" id="duration">{duration}</div>
            <div className="card-section" id="capacity"><CapacityWithDelta start={session.capacity_start} delta={session.capacity_delta} end={session.capacity_end} /></div>
            <div className="card-section" id="avg-consumption">Avg: {avgPower}W</div>
        </div>
    );
}

function secondsToDuration(d) {
    const hours = Math.floor(d / 3600);
    const minutes = Math.floor((d % 3600) / 60);

    let duration = "";
    duration += hours + "h";
    if (minutes < 10)
        duration += "0";
    duration += minutes + "m";

    return duration;
}

export default Element;
