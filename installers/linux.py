from os import system, path, makedirs

def install():
    client_id = input("Your Spotify ClientId: ")
    client_secret = input("Your Spotify Client Secret: ")

    service_path = path.expanduser("~") + "/.config/systemd/user"
    abs_service_path = path.abspath(service_path)
    if not path.exists(service_path):
        makedirs(service_path)
    service_file = open(service_path + "/album-notifier.service", mode="w")

    path_to_notifier = path.expanduser("~") + "/.local/bin/album-notifier";
    service_file.write("[Unit]\nDescription=Album Notifier\n\n[Service]\nType=simple\nExecStart=" + path_to_notifier + "\n\n[Install]\nWantedBy=default.target")
    service_file.close()

    system("systemctl --user enable album-notifier.service")
    pass