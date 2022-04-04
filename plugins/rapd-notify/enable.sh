export PATH=$PATH:/usr/share/rapd/plugins/notify
rapc hook_add_player_start $(which rapd-notify)
rapc hook_add_server_shutdown $(which rapd-notify)\ shutdown
rapc hook_add_player_pause $(which rapd-notify)\ pause
rapc hook_add_player_unpause $(which rapd-notify)\ unpause
