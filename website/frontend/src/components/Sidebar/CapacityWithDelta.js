function CapacityWithDelta({ start, delta, end }) {
    // Determine the arrow direction based on the delta
    const arrow = delta > 0 ? '↑' : delta < 0 ? '↓' : '→';

    // Determine the color based on the delta
    const color = delta > 0 ? 'green' : delta < 0 ? 'red' : 'black';

    // Format the delta with a plus sign for positive numbers
    const formattedDelta = (delta > 0 ? '+' : '') + delta;

    return (
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
            <span>{start}</span>
            <span style={{ fontWeight: "bold", margin: '0 10px', color: color, fontSize: '18px' }}>
                {arrow} {formattedDelta}
            </span>
            <span>{end}</span>
        </div>
    );
}

export default CapacityWithDelta;
