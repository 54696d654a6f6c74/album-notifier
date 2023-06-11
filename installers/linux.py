from os import system, path, makedirs


def install():
    client_id = input("Your Spotify ClientId: ")
    client_secret = input("Your Spotify Client Secret: ")
    artists_file_location = input("Path to artists file: ")

    notifier_bin_folder = path.abspath(path.expanduser("~") + "/.local/bin/album-notifier")
    artists_file_path = artists_file_location

    setup_notifier_bin(notifier_bin_folder)
    setup_notifier_env(notifier_bin_folder, client_id, client_secret, artists_file_path)
    setup_service("/album-notifier", notifier_bin_folder)

def setup_notifier_bin(notifer_bin_folder):
    if not path.exists(notifer_bin_folder):
        makedirs(notifer_bin_folder)

    system("cp ./album-notifier " + notifer_bin_folder)

def setup_notifier_env(notifer_bin_folder, client_id, client_secret, artists_file_path):
    if not path.exists(notifer_bin_folder):
        makedirs(notifer_bin_folder)

    env_file = open(notifer_bin_folder + '/.env', mode="w")

    env_file.write(
"""CLIENT_ID={0}
    CLIENT_SECRET={1}
    ARTIST_LIST_PATH={2}
""".format(client_id, client_secret, artists_file_path))

def setup_service(notifier_exec_name, notifier_bin_folder):
    service_path = path.expanduser("~") + "/.config/systemd/user"

    if not path.exists(service_path):
        makedirs(service_path)
    service_file = open(service_path + "/album-notifier.service", mode="w")

    service_file.write(
"""[Unit]
    Description=Album Notifier
    
    [Service]
    Type=simple
    ExecStart={0}/{1}
    WorkingDirectory={0}
    
    [Install]
    WantedBy=default.target
""".format(notifier_bin_folder, notifier_exec_name))
    service_file.close()

    system("systemctl --user enable album-notifier.service")