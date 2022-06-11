A simple calendar dashboard webapp, integrating with Google Calendar.
---
At the moment the app is intended for a local network usage (no auth etc.)
It can be run as a systemd on a server(ish) device and accessed through a web browser.

A GCP service account is required for authorization - with at least read only calendar scope.
Target calendar has to be shared with the created service account user.

A `settings.toml` file is required for configuration:

```toml
[calendar]
calendar_id = "your_calendar_id" # Google Calendar ID, often your Google email addres
refresh_secs = 900 # calendar data refresh rate
days_ahead = 4 # number of days from today's date for event polling / display
key_path = "key.json" # a path to authorization key obtainend from GCP
debug = false # displays test data, instead of polling a real calendar

[server]
host = "192.168.1.104"
port = 3000
```

In order to use a full screen mode on mobile devices, a SPA manifest.json is provided.
The app can be pinned on devices' home screen. If run from there it should behave similarly to a native app.
