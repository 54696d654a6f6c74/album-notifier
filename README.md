## What is this?
Soon to be cross platform simple notification service for when a new album is released. No need to get added to a marketing mailing list and get spammed. Just a simple background worker that sends a toast when it detects a new album on Spotify.

## How does it work?
The install script sets-up a simple background service that runs the program on login.

The program itself gets the newest albums for artists specified by you in a simple text file. If the newest album is diffrent from the previous search, it sends a system toast. This repeats on a cron schedule.

## Install

### Pre-requisites:

1. Have a Spotify account
2. Client ID and Client Secret for a Spotify App
    - [How to create one](https://developer.spotify.com/documentation/web-api/tutorials/getting-started#create-an-app) if you don't have one
3. Python 3 installed on your system
4. Systemd on your system

### Installation:

1. Download the latest release from the [releases page](https://github.com/54696d654a6f6c74/album-notifier/releases)
2. Create an artists file anywhere on your system (you will need the absolute path to it)
3. Run the `install.py` script and provide:
 - Your Spotify App client ID and secret
 - An absolute path to your artists file

## Artists file

A simple text file listing all the artists you want to be notified about new albums

### Eample:
```
Nightwish
Within Temptation
Sabaton
```

**Note:** Using these names is the same as searching on Spotify **as a guest**. If the top result is not quite the right artist you can edit the name to help Spotify find the right one e.g. `Sabaton -> Sabaton (band)`.