lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stato"),
        ("Your Desktop", "Via aparato"),
        ("desk_tip", "Via aparato povas esti alirita kun tiu identigilo kaj pasvorto"),
        ("Password", "Pasvorto"),
        ("Ready", "Preta"),
        ("Established", ""),
        ("connecting_status", "Konektante al la reto RustDesk..."),
        ("Enable Service", "Ebligi servon"),
        ("Start Service", "Starti servon"),
        ("Service is running", ""),
        ("Service is not running", "La servo ne funkcias"),
        ("not_ready_status", "Ne preta, bonvolu kontroli la retkonekto"),
        ("Control Remote Desktop", "Kontroli foran aparaton"),
        ("Transfer File", "Transigi dosieron"),
        ("Connect", "Konekti al"),
        ("Recent Sessions", "Lastaj sesioj"),
        ("Address Book", "Adresaro"),
        ("Confirmation", "Konfirmacio"),
        ("TCP Tunneling", "Tunelado TCP"),
        ("Remove", "Forigi"),
        ("Refresh random password", "Regeneri hazardan pasvorton"),
        ("Set your own password", "Agordi vian propran pasvorton"),
        ("Enable Keyboard/Mouse", "Ebligi klavaro/muso"),
        ("Enable Clipboard", "Sinkronigi poŝon"),
        ("Enable File Transfer", "Ebligi dosiertransigado"),
        ("Enable TCP Tunneling", "Ebligi tunelado TCP"),
        ("IP Whitelisting", "Listo de IP akceptataj"),
        ("ID/Relay Server", "Identigila/Relajsa servilo"),
        ("Import Server Config", "Enporti servilan agordon"),
        ("Export Server Config", ""),
        ("Import server configuration successfully", "Importi servilan agordon sukcese"),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", "Nevalida servila agordo"),
        ("Clipboard is empty", "La poŝo estas malplena"),
        ("Stop service", "Haltu servon"),
        ("Change ID", "Ŝanĝi identigilon"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Nur la signoj a-z, A-Z, 0-9, _ (substreko) povas esti uzataj. La unua litero povas esti inter a-z, A-Z. La longeco devas esti inter 6 kaj 16."),
        ("Website", "Retejo"),
        ("About", "Pri"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Muta"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Aŭdia enigo"),
        ("Enhancements", ""),
        ("Hardware Codec", ""),
        ("Adaptive bitrate", ""),
        ("ID Server", "Servilo de identigiloj"),
        ("Relay Server", "Relajsa servilo"),
        ("API Server", "Servilo de API"),
        ("invalid_http", "Devas komenci kun http:// aŭ https://"),
        ("Invalid IP", "IP nevalida"),
        ("Invalid format", "Formato nevalida"),
        ("server_not_support", "Ankoraŭ ne subtenata de la servilo"),
        ("Not available", "Nedisponebla"),
        ("Too frequent", "Tro ofte ŝanĝita, bonvolu reprovi poste"),
        ("Cancel", "Nuligi"),
        ("Skip", "Ignori"),
        ("Close", "Fermi"),
        ("Retry", "Reprovi"),
        ("OK", "Konfermi"),
        ("Password Required", "Pasvorto deviga"),
        ("Please enter your password", "Bonvolu tajpi vian pasvorton"),
        ("Remember password", "Memori pasvorton"),
        ("Wrong Password", "Erara pasvorto"),
        ("Do you want to enter again?", "Ĉu vi aliri denove?"),
        ("Connection Error", "Eraro de konektado"),
        ("Error", "Eraro"),
        ("Reset by the peer", "La konekto estas fermita de la samtavolano"),
        ("Connecting...", "Konektante..."),
        ("Connection in progress. Please wait.", "Konektado farata. Bonvolu atendi."),
        ("Please try 1 minute later", "Reprovi post 1 minuto"),
        ("Login Error", "Eraro de konektado"),
        ("Successful", "Sukceso"),
        ("Connected, waiting for image...", "Konektita, atendante bildon..."),
        ("Name", "Nomo"),
        ("Type", ""),
        ("Modified", "Modifita"),
        ("Size", "Grandeco"),
        ("Show Hidden Files", "Montri kaŝitajn dosierojn"),
        ("Receive", "Akcepti"),
        ("Send", "Sendi"),
        ("Refresh File", ""),
        ("Local", ""),
        ("Remote", ""),
        ("Remote Computer", "Fora komputilo"),
        ("Local Computer", "Loka komputilo"),
        ("Confirm Delete", "Konfermi la forigo"),
        ("Delete", ""),
        ("Properties", ""),
        ("Multi Select", ""),
        ("Select All", ""),
        ("Unselect All", ""),
        ("Empty Directory", ""),
        ("Not an empty directory", ""),
        ("Are you sure you want to delete this file?", "Ĉu vi vere volas forigi tiun dosieron?"),
        ("Are you sure you want to delete this empty directory?", ""),
        ("Are you sure you want to delete the file of this directory?", ""),
        ("Do this for all conflicts", "Same por ĉiuj konfliktoj"),
        ("This is irreversible!", ""),
        ("Deleting", "Forigado"),
        ("files", "dosiero"),
        ("Waiting", "Atendante..."),
        ("Finished", "Finita"),
        ("Speed", ""),
        ("Custom Image Quality", "Agordi bildan kvaliton"),
        ("Privacy mode", "Modo privata"),
        ("Block user input", "Bloki uzanta enigo"),
        ("Unblock user input", "Malbloki uzanta enigo"),
        ("Adjust Window", "Adapti fenestro"),
        ("Original", "Originala rilatumo"),
        ("Shrink", "Ŝrumpi"),
        ("Stretch", "Streĉi"),
        ("Scrollbar", "Rulumbreto"),
        ("ScrollAuto", "Rulumu Aŭtomate"),
        ("Good image quality", "Bona bilda kvalito"),
        ("Balanced", "Normala bilda kvalito"),
        ("Optimize reaction time", "Optimigi reakcia tempo"),
        ("Custom", ""),
        ("Show remote cursor", "Montri foran kursoron"),
        ("Show quality monitor", ""),
        ("Disable clipboard", "Malebligi poŝon"),
        ("Lock after session end", "Ŝlosi foran komputilon post malkonektado"),
        ("Insert", "Enmeti"),
        ("Insert Lock", "Ŝlosi foran komputilon"),
        ("Refresh", "Refreŝigi ekranon"),
        ("ID does not exist", "La identigilo ne ekzistas"),
        ("Failed to connect to rendezvous server", "Malsukcesis konekti al la servilo rendezvous"),
        ("Please try later", "Bonvolu provi poste"),
        ("Remote desktop is offline", "La fora aparato estas senkonektita"),
        ("Key mismatch", "Miskongruo de klavoj"),
        ("Timeout", "Konekta posttempo"),
        ("Failed to connect to relay server", "Malsukcesis konekti al la relajsa servilo"),
        ("Failed to connect via rendezvous server", "Malsukcesis konekti per servilo rendezvous"),
        ("Failed to connect via relay server", "Malsukcesis konekti per relajsa servilo"),
        ("Failed to make direct connection to remote desktop", "Malsukcesis konekti direkte"),
        ("Set Password", "Agordi pasvorton"),
        ("OS Password", "Pasvorto de la operaciumo"),
        ("install_tip", "Vi ne uzas instalita versio. Pro limigoj pro UAC, kiel aparato kontrolata, en kelkaj kazoj, ne estos ebla kontroli la muson kaj klavaron aŭ registri la ekranon. Bonvolu alkliku la butonon malsupre por instali RustDesk sur la operaciumo por eviti la demando supre."),
        ("Click to upgrade", "Alklaki por plibonigi"),
        ("Click to download", "Alklaki por elŝuti"),
        ("Click to update", "Alklaki por ĝisdatigi"),
        ("Configure", "Konfiguri"),
        ("config_acc", "Por uzi vian foran aparaton, bonvolu doni la permeson \"alirebleco\" al RustDesk."),
        ("config_screen", "Por uzi vian foran aparaton, bonvolu doni la permeson \"ekranregistrado\" al RustDesk."),
        ("Installing ...", "Instalante..."),
        ("Install", "Instali"),
        ("Installation", "Instalado"),
        ("Installation Path", "Vojo de instalo"),
        ("Create start menu shortcuts", "Aldoni ligilojn sur la startmenuo"),
        ("Create desktop icon", "Aldoni ligilojn sur la labortablo"),
        ("agreement_tip", "Starti la instaladon signifas akcepti la permesilon."),
        ("Accept and Install", "Akcepti kaj instali"),
        ("End-user license agreement", "Uzanta permesilon"),
        ("Generating ...", "Generante..."),
        ("Your installation is lower version.", "Via versio de instalaĵo estas pli malalta ol la lasta."),
        ("not_close_tcp_tip", "Bonvolu ne fermu tiun fenestron dum la uzo de la tunelo"),
        ("Listening ...", "Atendante konekton al la tunelo..."),
        ("Remote Host", "Fora gastiganto"),
        ("Remote Port", "Fora pordo"),
        ("Action", "Ago"),
        ("Add", "Aldoni"),
        ("Local Port", "Loka pordo"),
        ("Local Address", ""),
        ("Change Local Port", ""),
        ("setup_server_tip", "Se vi bezonas pli rapida konekcio, vi povas krei vian propran servilon"),
        ("Too short, at least 6 characters.", "Tro mallonga, almenaŭ 6 signoj."),
        ("The confirmation is not identical.", "Ambaŭ enigoj ne kongruas"),
        ("Permissions", "Permesoj"),
        ("Accept", "Akcepti"),
        ("Dismiss", "Malakcepti"),
        ("Disconnect", "Malkonekti"),
        ("Allow using keyboard and mouse", "Permesi la uzon de la klavaro kaj muso"),
        ("Allow using clipboard", "Permesi la uzon de la poŝo"),
        ("Allow hearing sound", "Permesi la uzon de la sono"),
        ("Allow file copy and paste", "Permesu kopii kaj alglui dosierojn"),
        ("Connected", "Konektata"),
        ("Direct and encrypted connection", "Konekcio direkta ĉifrata"),
        ("Relayed and encrypted connection", "Konekcio relajsa ĉifrata"),
        ("Direct and unencrypted connection", "Konekcio direkta neĉifrata"),
        ("Relayed and unencrypted connection", "Konekcio relajsa neĉifrata"),
        ("Enter Remote ID", "Tajpu foran identigilon"),
        ("Enter your password", "Tajpu vian pasvorton"),
        ("Logging in...", "Konektante..."),
        ("Enable RDP session sharing", "Ebligi la kundivido de sesio RDP"),
        ("Auto Login", "Aŭtomata konektado (la ŝloso nur estos ebligita post la malebligado de la unua parametro)"),
        ("Enable Direct IP Access", "Permesi direkta eniro per IP"),
        ("Rename", "Renomi"),
        ("Space", "Spaco"),
        ("Create Desktop Shortcut", "Krei ligilon sur la labortablon"),
        ("Change Path", "Ŝanĝi vojon"),
        ("Create Folder", "Krei dosierujon"),
        ("Please enter the folder name", "Bonvolu enigi la dosiernomon"),
        ("Fix it", "Riparu ĝin"),
        ("Warning", "Averto"),
        ("Login screen using Wayland is not supported", "Konektajn ekranojn uzantajn Wayland ne estas subtenitaj"),
        ("Reboot required", "Restarto deviga"),
        ("Unsupported display server", "La aktuala bilda servilo ne estas subtenita"),
        ("x11 expected", "Bonvolu uzi x11"),
        ("Port", ""),
        ("Settings", "Agordoj"),
        ("Username", " Uzanta nomo"),
        ("Invalid port", "Pordo nevalida"),
        ("Closed manually by the peer", "Manuale fermita de la samtavolano"),
        ("Enable remote configuration modification", "Permesi foran redaktadon de la konfiguracio"),
        ("Run without install", "Plenumi sen instali"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Ĉiam konekti per relajso"),
        ("whitelist_tip", "Nur la IP en la blanka listo povas kontroli mian komputilon"),
        ("Login", "Konekti"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Malkonekti"),
        ("Tags", "Etikedi"),
        ("Search ID", "Serĉi ID"),
        ("whitelist_sep", "Vi povas uzi komon, punktokomon, spacon aŭ linsalton kiel apartigilo"),
        ("Add ID", "Aldoni identigilo"),
        ("Add Tag", "Aldoni etikedo"),
        ("Unselect all tags", "Malselekti ĉiujn etikedojn"),
        ("Network error", "Reta eraro"),
        ("Username missed", "Uzantnomo forgesita"),
        ("Password missed", "Pasvorto forgesita"),
        ("Wrong credentials", "Identigilo aŭ pasvorto erara"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Redakti etikedo"),
        ("Forget Password", "Forgesi pasvorton"),
        ("Favorites", "Favorataj"),
        ("Add to Favorites", "Aldoni al la favorataj"),
        ("Remove from Favorites", "Forigi el la favorataj"),
        ("Empty", "Malplena"),
        ("Invalid folder name", "Dosiernomo nevalida"),
        ("Socks5 Proxy", "Socks5 prokura servilo"),
        ("Hostname", "Nomo de gastiga"),
        ("Discovered", "Malkovritaj"),
        ("install_daemon_tip", ""),
        ("Remote ID", "Fora identigilo"),
        ("Paste", "Alglui"),
        ("Paste here?", ""),
        ("Are you sure to close the connection?", "Ĉu vi vere volas fermi la konekton?"),
        ("Download new version", "Elŝuti la novan version"),
        ("Touch mode", "Tuŝa modo"),
        ("Mouse mode", ""),
        ("One-Finger Tap", ""),
        ("Left Mouse", ""),
        ("One-Long Tap", ""),
        ("Two-Finger Tap", ""),
        ("Right Mouse", ""),
        ("One-Finger Move", ""),
        ("Double Tap & Move", ""),
        ("Mouse Drag", ""),
        ("Three-Finger vertically", ""),
        ("Mouse Wheel", ""),
        ("Two-Finger Move", ""),
        ("Canvas Move", ""),
        ("Pinch to Zoom", ""),
        ("Canvas Zoom", ""),
        ("Reset canvas", "Restarigi kanvaso"),
        ("No permission of file transfer", "Neniu permeso de dosiertransigo"),
        ("Note", "Notu"),
        ("Connection", ""),
        ("Share Screen", ""),
        ("Chat", ""),
        ("Total", ""),
        ("items", ""),
        ("Selected", ""),
        ("Screen Capture", ""),
        ("Input Control", ""),
        ("Audio Capture", ""),
        ("File Connection", ""),
        ("Screen Connection", ""),
        ("Do you accept?", ""),
        ("Open System Setting", ""),
        ("How to get Android input permission?", ""),
        ("android_input_permission_tip1", ""),
        ("android_input_permission_tip2", ""),
        ("android_new_connection_tip", ""),
        ("android_service_will_start_tip", ""),
        ("android_stop_service_tip", ""),
        ("android_version_audio_tip", ""),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", ""),
        ("Overwrite", ""),
        ("This file exists, skip or overwrite this file?", ""),
        ("Quit", ""),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", ""),
        ("Failed", ""),
        ("Succeeded", ""),
        ("Someone turns on privacy mode, exit", ""),
        ("Unsupported", ""),
        ("Peer denied", ""),
        ("Please install plugins", ""),
        ("Peer exit", ""),
        ("Failed to turn off", ""),
        ("Turned off", ""),
        ("In privacy mode", ""),
        ("Out privacy mode", ""),
        ("Language", ""),
        ("Keep RustDesk background service", ""),
        ("Ignore Battery Optimizations", ""),
        ("android_open_battery_optimizations_tip", ""),
        ("Start on Boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", ""),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", ""),
        ("Use both passwords", ""),
        ("Set permanent password", ""),
        ("Enable Remote Restart", ""),
        ("Allow remote restart", ""),
        ("Restart Remote Device", ""),
        ("Are you sure you want to restart", ""),
        ("Restarting Remote Device", ""),
        ("remote_restarting_tip", ""),
        ("Copied", ""),
        ("Exit Fullscreen", "Eliru Plenekranon"),
        ("Fullscreen", "Plenekrane"),
        ("Mobile Actions", "Poŝtelefonaj Agoj"),
        ("Select Monitor", "Elektu Monitoron"),
        ("Control Actions", "Kontrolaj Agoj"),
        ("Display Settings", "Montraj Agordoj"),
        ("Ratio", "Proporcio"),
        ("Image Quality", "Bilda Kvalito"),
        ("Scroll Style", "Ruluma Stilo"),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", "Rekta Konekto"),
        ("Relay Connection", "Relajsa Konekto"),
        ("Secure Connection", "Sekura Konekto"),
        ("Insecure Connection", "Nesekura Konekto"),
        ("Scale original", "Skalo originalo"),
        ("Scale adaptive", "Skalo adapta"),
        ("General", ""),
        ("Security", ""),
        ("Theme", ""),
        ("Dark Theme", ""),
        ("Light Theme", ""),
        ("Dark", ""),
        ("Light", ""),
        ("Follow System", ""),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", ""),
        ("Enable Audio", ""),
        ("Unlock Network Settings", ""),
        ("Server", ""),
        ("Direct IP Access", ""),
        ("Proxy", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", ""),
        ("Use IP Whitelisting", ""),
        ("Network", ""),
        ("Enable RDP", ""),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable Recording Session", ""),
        ("Allow recording session", ""),
        ("Enable LAN Discovery", ""),
        ("Deny LAN Discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", ""),
        ("Full Access", ""),
        ("Screen Share", ""),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland postulas Ubuntu 21.04 aŭ pli altan version."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland postulas pli altan version de linuksa distro. Bonvolu provi X11-labortablon aŭ ŝanĝi vian OS."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Bonvolu Elekti la ekranon por esti dividita (Funkciu ĉe la sama flanko)."),
        ("Show RustDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", ""),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", ""),
        ("Default Scroll Style", ""),
        ("Default Image Quality", ""),
        ("Default Codec", ""),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", ""),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("install_cert_tip", ""),
        ("confirm_install_cert_tip", ""),
        ("RDP Settings", ""),
        ("Sort by", ""),
        ("New Connection", ""),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", ""),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", ""),
        ("Empty Username", ""),
        ("Empty Password", ""),
        ("Me", ""),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", ""),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", ""),
        ("System Sound", ""),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", ""),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("switch_display_elevated_connections_tip", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("elevated_switch_display_msg", ""),
        ("Choose Display Behavior", ""),
        ("Switch Display", ""),
        ("Open in New Window", ""),
    ].iter().cloned().collect();
}
