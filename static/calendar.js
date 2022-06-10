const TIME_OPTIONS = {hour: '2-digit', minute:'2-digit'}

export function renderEvents (data, container) {
    let output = ''
    for (event of data) {
        output += renderSingleEvent(event)
    }
    output += ''
    container.innerHTML = output
}

function renderSingleEvent (data) {
    const start = new Date(data.start.dateTime)
    const end = new Date(data.end.dateTime)
    // const top = timeToPx(start);
    // const height = timeToPx(end) - top;
    return `
    <div class="grid_event shadow">
        <div class="event_headline">
            <span class="event_time">
                ${start.toLocaleTimeString('pl-PL', TIME_OPTIONS)} - ${end.toLocaleTimeString('pl-PL', TIME_OPTIONS)}
            </span>
            <span class="event_date">${start.toLocaleDateString('pl-PL')}</span>
        </div>
        <div class="event_summary">
            ${data.summary}
        </div>
    </div>`
}

export function renderNextEvent (data, container) {
    let output
    if (data.length == 0) {
        output = ''
    } else {
        const next = getNextEvent(data)
        const start = new Date(next.start.dateTime)
        const end = new Date(next.end.dateTime)
        output = `
        <div id="current" class="shadow">
            <span id="current_time">${new Date().toLocaleTimeString('pl-PL', TIME_OPTIONS)}</span>
            <span id="current_date">${new Date().toLocaleDateString('pl-PL')}</span>
        </div>
        <div class="next_content shadow">
            <div id="next_headline">
                <span id="next_time">
                    ${start.toLocaleTimeString('pl-PL', TIME_OPTIONS)}-${end.toLocaleTimeString('pl-PL', TIME_OPTIONS)}</span>
                <span id="next_date">${start.toLocaleDateString('pl-PL')}</span>
            </div>
            <div id="next_summary">
                ${next.summary}
            </div>
            <div id="next_link">
                <a href="${next.hangoutLink}">Dołącz &#8594;</a>
            </div>
        </div>
        `
    }
    container.innerHTML = output
}

function getNextEvent (data) {
    const now = new Date()
    const filtered = data.filter((a) => new Date(a.end.dateTime) > now)
    const sorted = filtered.sort((a, b) => new Date(a.start.dateTime) - new Date(b.start.dateTime))
    return sorted[0]
}

// let timeToPx = (dateTime) => {
//     console.log(dateTime)
//     return (dateTime - new Date().setHours(0,0,0,0)) / (30 * 1000)  
// }
