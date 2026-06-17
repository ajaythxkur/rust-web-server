import http from 'k6/http';

export const options = {
    vus: 100,
    duration: '30s',
};

export default function () {
    http.post(
        'http://localhost:8585/bookings',
        JSON.stringify({
            owner: '6a3252208fe8256018257c75',
            start_time: '2026-06-17T13:00:00.000Z',
            duration_in_minutes: 10
        }),
        {
            headers: { 'Content-Type': 'application/json' }
        }
    );
}