import { renderEvents, renderNextEvent } from './calendar.js'

window.onload = () => {
    connect()
}

let connect = () => {
    const socket = new WebSocket(`ws://${window.ws_host}:${window.ws_port}/ws`)
    
    const all_events_div = document.getElementById('all_events').firstElementChild
    const next_event_div = document.getElementById('next_event').firstElementChild
    
    const status_div = document.getElementById('status');
    let calendar_events = []

    status_div.textContent = 'Starting App'

    socket.addEventListener('open', (event) => {
        socket.send('Hello')
        status_div.textContent = 'Connection established'
    });

    socket.addEventListener('error', (event) => {
        // status_div.textContent = `Connection error - ${event.message}`
        socket.close()
    });

    socket.addEventListener('close', () => {
        status_div.textContent = 'Disconnected, reconnecting...'
        setTimeout(() => {
            connect()
        }, 10000)
    });

    socket.addEventListener('message', (event) => {
        let json = JSON.parse(event.data)
        calendar_events = json
        renderEvents(json, all_events_div)
        status_div.textContent = `Message received at ${new Date()}`
    });

    let time_interval = setInterval(() => {
        renderNextEvent(calendar_events, next_event_div)
    }, 5000)
}