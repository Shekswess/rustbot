//! Authored replies and patterns from PyBot 7d63c7f, bot/knowledge/technology.py.

pub(super) struct PlatformSteps {
    pub mac: &'static str,
    pub windows: &'static str,
    pub tail: &'static str,
}

pub(super) struct Definition {
    pub keys: &'static [&'static str],
    pub reply: &'static str,
    pub avoid: &'static [&'static str],
}

pub(super) const PLATFORM_MAC: &str =
    r#"(?i)\b(?:mac|macbook|mac book|macbooks|imac|apple|os x|osx|macos)\b"#;

pub(super) const SCREEN_NAMED_RE: &str = r#"(?i)\b(?:text|texts|font|fonts|writing|letters|zoom|magnify|magnified|magnifier|screen|screens|browser|page|pages|website|websites|document|documents|word|excel|powerpoint|laptop|laptops|computer|computers|desktop|mac|macbook|windows|phone|phones|iphone|android|ipad|tablet|kindle|app|apps|settings|display|cursor|mouse|read|reading|tv|television|spreadsheet|email)\b"#;

pub(super) const NOWHERE_TO_KEEP_STILL: &str = r#"(?i)\b(?:usb|usb stick|usb drive|memory stick|flash drive|pen drive|stick|sd card|memory card|micro sd|external drive|external hard|hard drive|hard disk|disk|disc|drive|card)\b"#;

pub(super) const IN_A_FOLDER: &str = r#"(?i)\b(?:word|excel|doc|docx|spreadsheet|essay|report|homework|presentation|powerpoint|folder|onedrive|google drive|dropbox)\b"#;

pub(super) const A_CARD: &str = r#"(?i)\b(?:sd card|memory card|micro sd|card|cards)\b"#;

pub(super) const A_DRIVE: &str =
    r#"(?i)\b(?:external drive|external hard|hard drive|hard disk|disk|disc|drive)\b"#;

pub(super) const EMPTY_TRASH: PlatformSteps = PlatformSteps {
    mac: "On a Mac, open the Bin from the Dock and press Empty in the top right, or press Command-Shift-Delete to do it from the keyboard; holding Option while you press Empty skips the question. Nothing is freed until the Bin is emptied, so a full disk is often just this.",
    windows: "On Windows, right-click the Recycle Bin on the desktop and choose Empty Recycle Bin, or open it and press Empty Recycle Bin at the top; the files are only really gone once you do, so a full disk is often just this.",
    tail: "",
};

pub(super) const FORCE_QUIT: PlatformSteps = PlatformSteps {
    mac: "On a Mac press Command-Option-Esc, pick the app in the list and press Force Quit: that closes the one app and leaves everything else open, and if the whole Mac is stuck, hold the power button for ten seconds instead.",
    windows: "On Windows press Ctrl-Shift-Esc for Task Manager, pick the app and choose End task: that closes the one app and leaves everything else open, and if the whole machine is stuck, hold the power button for ten seconds instead.",
    tail: "",
};

pub(super) const DEFAULT_BROWSER: PlatformSteps = PlatformSteps {
    mac: "On a Mac, open System Settings > Desktop & Dock and scroll to Default web browser near the bottom (on older macOS it is Safari > Preferences > General > Default web browser). Pick the one you want from the menu and every link opens there from then on.",
    windows: "On Windows, open Settings > Apps > Default apps, type the browser's name in the search box and choose Set default; that sends links, .html files and mail links to it.",
    tail: "",
};

pub(super) const BATTERY_HEALTH: PlatformSteps = PlatformSteps {
    mac: "On a Mac, hold Option and click the Apple menu > System Information > Power: the cycle count and the full charge capacity sit beside the design capacity, and under about 70% is a worn battery.",
    windows: "On Windows, open Command Prompt and run `powercfg /batteryreport` - the report gives the design capacity and what it holds now, and under about 70% is a worn battery.",
    tail: " A laptop battery is usually £40 to £90 and most are held in with screws, so it is worth replacing rather than buying a new machine.",
};

pub(super) const UNINSTALL_PROGRAM: PlatformSteps = PlatformSteps {
    mac: "On a Mac, open Finder > Applications, drag the app to the Bin and empty it - some leave a folder in Library, which is harmless to leave.",
    windows: "Windows: Settings > Apps > Installed apps, find it in the list and choose Uninstall.",
    tail: " A program that will not go usually has its own uninstall entry, under its maker's name, in the Start menu.",
};

pub(super) const SYMBOLS: &[(&[&str], &str)] = &[
    (&["pound", "£", "sterling"], "£ is Shift and 3"),
    (
        &["euro", "€"],
        "€ is AltGr and 4, or Ctrl and Alt and 4 where there is no AltGr key",
    ),
    (&["dollar", "$"], "$ is Shift and 4"),
    (
        &["at sign", "at symbol", "@", "email sign"],
        "@ is Shift and 2",
    ),
    (
        &["hash", "#"],
        "# is the key next to Enter, on the third row up - Shift and 3 gives # on a keyboard set to the US layout",
    ),
    (
        &["backslash", r#"\"#],
        "the backslash is the key directly above Enter, and Shift and that key gives the pipe |",
    ),
    (
        &["forward slash", "slash", "/"],
        "/ is the key next to the right Shift",
    ),
    (&["underscore", "_"], "_ is Shift and the minus key"),
    (
        &["tilde", "~"],
        "~ is Shift and the key left of the 1, or Option and N then space on a Mac",
    ),
    (
        &["degree", "°"],
        "there is no degree key: hold Alt and type 0176 on Windows, or Option and Shift and 8 on a Mac",
    ),
    (&["asterisk", "star sign", "*"], "* is Shift and 8"),
    (
        &["pipe", "|"],
        "| is Shift and the backslash key, above Enter",
    ),
];

pub(super) const TECH_WORDS: &[Definition] = &[
    Definition {
        keys: &["modem", "modems"],
        reply: "A modem turns the signal on the line coming into your home - fibre, cable or the phone socket - into a network connection, and the router is what shares that connection round the house. Most home boxes do both jobs in one, which is why people use the two words for the same box; the modem is the half that talks to the street.",
        avoid: &[
            "my modem",
            "not working",
            "flashing",
            "red light",
            "restart",
            "reset",
            "replace",
            "how do i",
        ],
    },
    Definition {
        keys: &["cookie", "cookies"],
        reply: "A cookie is a small file a website asks your browser to store: the note that keeps you signed in, the contents of your basket, or an ID that follows you between pages. It is not a program and it cannot read your files - the worst of them is tracking - and clearing them signs you out of most sites.",
        avoid: &[
            "recipe",
            "bake",
            "baking",
            "biscuit",
            "dough",
            "oven",
            "flour",
            "butter",
            "chocolate",
            "sugar",
            "bake",
            "tin",
            "tray",
            "my cookies",
            "accept",
            "banner",
            "how do i",
            "should i",
            "consent",
            "gdpr",
        ],
    },
    Definition {
        keys: &["ransomware"],
        reply: "Ransomware is a kind of malware that scrambles your files and then sells you the key: a message appears, the documents and photos will not open, and it wants money, usually in cryptocurrency. The one thing that reliably saves you is a backup you were not connected to when it struck, and paying often gets nothing back, so unplug the machine and get help before you do anything else.",
        avoid: &[
            "my", "i have", "infected", "hit by", "attack", "attacked", "caught", "paid", "pay",
            "removal", "remove", "fix", "recover", "how do i", "should i", "protect",
        ],
    },
    Definition {
        keys: &[
            "phishing",
            "phishing email",
            "phishing text",
            "phishing message",
        ],
        reply: "Phishing is a fake message dressed up as your bank, a courier or HMRC to get you to log in or pay: the link goes to a page that looks right and the details go to the thief. The tell is the address it really came from and the urgency, so do not use the link - open the company's app or type the address yourself - and report it as phishing.",
        avoid: &[
            "i got",
            "i clicked",
            "i have",
            "my",
            "what should i do",
            "how do i",
            "is this",
            "was it",
            "am i",
        ],
    },
    Definition {
        keys: &["password manager", "password managers"],
        reply: "A password manager is an app that keeps your passwords encrypted behind one master password or your face, and fills them in for you: Bitwarden, 1Password, the one built into Chrome, Safari and iCloud. It can invent a long random password for every site, which is the point - you only ever remember the one that opens the manager.",
        avoid: &[
            "my",
            "which",
            "best",
            "how do i",
            "should i",
            "set up",
            "forgot",
            "lost",
            "family",
            "free",
            "recommend",
        ],
    },
    Definition {
        keys: &["cloud storage", "cloud backup", "the cloud"],
        reply: "Cloud storage is your files kept on someone else's servers and copied to your devices: iCloud, Google Drive, OneDrive, Dropbox. It is a copy rather than a backup if it syncs the deletion too - keep one copy that is not attached, or the ransomware or the accidental delete reaches it as well.",
        avoid: &[
            "weather",
            "sky",
            "rain",
            "forecast",
            "full",
            "storage full",
            "upgrade",
            "plan",
            "price",
            "cost",
            "my",
            "which",
            "best",
            "how do i",
            "should i",
        ],
    },
];

pub(super) const RECOVER_IN_A_BIN: &str = "Check the Recycle Bin on Windows or the Bin on a Mac first, and in Word or Excel open File > Info > Manage Document > Recover Unsaved Documents, which holds copies for a few days even when you never saved. If the file lived in OneDrive or Google Drive, right-click it and open Version history, or look in the drive's own bin: both keep deleted files for 30 days.";

pub(super) const RECOVER_ON_A_DRIVE: &str = "Stop using the {thing} now, and do not save anything else to it: a deleted file is still there under the space a new one would take, and every file you add can cover it. Plug it into a computer and run a recovery tool that only reads the {thing} - Recuva on Windows, or PhotoRec on any system, both free - and run the deep scan, because a quick one misses the older files. If it was pulled out without ejecting, the files may be there and the index damaged, so run the computer's own check (chkdsk on Windows, First Aid on a Mac) before you give up: that fixes the view of the file, not the file itself.";

pub(super) const TV_CAST: &str = "Cast it rather than cable it: put the phone and the TV on the same wifi, open the app (YouTube, iPlayer, Netflix) and tap the Cast or AirPlay icon at the top of the video, then pick the TV. For the whole screen, iPhone use Screen Mirroring in Control Centre with an Apple TV or an AirPlay set, Android use Smart View or Cast in the quick settings. If the TV does not appear, it is the wifi band: both have to be on the same network, and a guest network or the 5GHz-only setting hides it. A Lightning or USB-C to HDMI adapter is the wired fallback and never drops.";

pub(super) const TV_CABLE: &str = "A cable is the reliable way: an HDMI lead from the laptop to the TV, then press Source or Input on the TV remote and pick the socket you used; a laptop with no HDMI port needs a USB-C to HDMI adapter, which does the same job. Once the picture is up, pick the TV as the sound output - the speaker icon in the Windows tray, or System Settings > Sound on a Mac - because the laptop keeps playing sound otherwise. No cable to hand and the app has a Cast icon: that works too, with both on the same wifi.";

pub(super) const MACHINE_BIN_WORDS: &[&str] = &[
    "desktop",
    "disk",
    "disks",
    "download",
    "downloads",
    "file",
    "files",
    "folder",
    "folders",
    "laptop",
    "mac",
    "macbook",
    "permanently",
    "recycle",
    "restore",
    "storage",
    "trash",
    "usb",
    "windows",
];
