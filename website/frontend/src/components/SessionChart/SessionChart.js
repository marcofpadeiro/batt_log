
import React from 'react';
import { Line } from 'react-chartjs-2';
import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Filler, TimeScale } from 'chart.js';
import 'chartjs-adapter-moment';

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Filler,
    TimeScale
);

const SessionChart = (data = []) => {
    const options = {
        responsive: true,
        scales: {
            x: {
                type: 'time',
                time: {
                    tooltipFormat: 'll HH:mm'
                },
                title: {
                    display: true,
                    text: 'Timestamp'
                }
            },
            y: {
                min: 0,
                max: 100,
                ticks: {
                    callback: function(value) {
                        return `${value}%`;
                    }
                },
                title: {
                    display: true,
                    text: 'Capacity %'
                }
            }
        },
        plugins: {
            legend: {
                display: false,
            },
        },
    };

    const isValidDataArray = Array.isArray(data.data);

    const chartData = {
        datasets: [{
            label: 'Capacity',
            data: isValidDataArray ? data.data.map(item => ({ x: item.timestamp, y: item.capacity })) : [],
            fill: false,
            borderColor: 'rgb(75, 192, 192)',
            tension: 0.1
        }]
    };

    return <Line options={options} data={chartData} />;
}

export default SessionChart;

