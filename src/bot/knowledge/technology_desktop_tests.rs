//! Direct callback and guard fixtures captured from PyBot 7d63c7f.
use super::*;

type Case = (
    Option<Reply>,
    &'static str,
    Option<&'static str>,
    Option<bool>,
);

#[test]
fn technology_desktop_callbacks_match_the_reference() {
    let cases: &[Case] = &[
        (
            Some(Reply::EmptyTrash),
            "How do I empty Trash on a MacBook?",
            Some(MAC_TRASH),
            Some(true),
        ),
        (
            Some(Reply::EmptyTrash),
            "How do I empty the Recycle Bin on Windows?",
            Some(WINDOWS_TRASH),
            Some(true),
        ),
        (
            Some(Reply::EmptyTrash),
            "How do I empty the bin on a computer?",
            Some(WINDOWS_TRASH),
            Some(true),
        ),
        (
            Some(Reply::EmptyTrash),
            "How do I empty the kitchen bin?",
            Some(WINDOWS_TRASH),
            Some(false),
        ),
        (
            Some(Reply::EmptyTrash),
            "The bin needs emptying",
            Some(WINDOWS_TRASH),
            Some(false),
        ),
        (
            Some(Reply::EmptyTrash),
            "How do I empty Trash?",
            Some(WINDOWS_TRASH),
            Some(true),
        ),
        (
            Some(Reply::EmptyTrash),
            "My app's bin is full",
            Some(WINDOWS_TRASH),
            Some(true),
        ),
        (
            Some(Reply::EmptyTrash),
            "How do I empty my desktop bins?",
            Some(WINDOWS_TRASH),
            Some(true),
        ),
        (
            Some(Reply::EmptyTrash),
            "How do I empty Trash in mac    book?",
            Some(MAC_TRASH),
            Some(true),
        ),
        (
            Some(Reply::EmptyTrash),
            "How should I empty the bin on my Chromebook?",
            Some(WINDOWS_TRASH),
            Some(false),
        ),
        (
            Some(Reply::ForceQuit),
            "How do I force quit an app on macOS?",
            Some(MAC_FORCE_QUIT),
            None,
        ),
        (
            Some(Reply::ForceQuit),
            "Force quit an application on Windows",
            Some(WINDOWS_FORCE_QUIT),
            None,
        ),
        (
            Some(Reply::ForceQuit),
            "How do I force an app to close?",
            Some(WINDOWS_FORCE_QUIT),
            None,
        ),
        (
            Some(Reply::ForceQuit),
            "Force close a frozen app on Windows and a Mac",
            Some(MAC_FORCE_QUIT),
            None,
        ),
        (
            Some(Reply::ForceQuit),
            "My Safari window won't close on OS X",
            Some(MAC_FORCE_QUIT),
            None,
        ),
        (
            Some(Reply::ForceQuit),
            "The zip on my jacket won't close",
            Some(WINDOWS_FORCE_QUIT),
            None,
        ),
        (
            Some(Reply::DefaultBrowser),
            "Change my default browser on a Mac",
            Some(MAC_BROWSER),
            None,
        ),
        (
            Some(Reply::DefaultBrowser),
            "Set Firefox as my default browser on Windows 11",
            Some(WINDOWS_BROWSER),
            None,
        ),
        (
            Some(Reply::DefaultBrowser),
            "Change default browser on my iPhone",
            Some(WINDOWS_BROWSER),
            None,
        ),
        (
            Some(Reply::DefaultBrowser),
            "Change the default browser in OSX",
            Some(MAC_BROWSER),
            None,
        ),
        (
            Some(Reply::DefaultBrowser),
            "Set a default browser on my PC and an Apple computer",
            Some(MAC_BROWSER),
            None,
        ),
        (
            Some(Reply::BatteryHealth),
            "How do I check battery health on my iMac?",
            Some(MAC_BATTERY),
            None,
        ),
        (
            Some(Reply::BatteryHealth),
            "How do I check my laptop battery health?",
            Some(WINDOWS_BATTERY),
            None,
        ),
        (
            Some(Reply::BatteryHealth),
            "Battery health on a Surface laptop",
            Some(WINDOWS_BATTERY),
            None,
        ),
        (
            Some(Reply::BatteryHealth),
            "Battery cycle count on a MacBook",
            Some(MAC_BATTERY),
            None,
        ),
        (
            Some(Reply::BatteryHealth),
            "Battery report for Windows or Mac",
            Some(MAC_BATTERY),
            None,
        ),
        (
            Some(Reply::Uninstall),
            "How do I uninstall a program on my MacBook?",
            Some(MAC_UNINSTALL),
            None,
        ),
        (
            Some(Reply::Uninstall),
            "How do I uninstall a program on Windows?",
            Some(WINDOWS_UNINSTALL),
            None,
        ),
        (
            Some(Reply::Uninstall),
            "How do I uninstall a program?",
            Some(WINDOWS_UNINSTALL),
            None,
        ),
        (
            Some(Reply::Uninstall),
            "Remove a program from an Apple computer",
            Some(MAC_UNINSTALL),
            None,
        ),
        (
            Some(Reply::Uninstall),
            "Uninstall a program on macbooks",
            Some(MAC_UNINSTALL),
            None,
        ),
        (
            Some(Reply::ToTv),
            "How do I connect my laptop to the TV?",
            Some(TV_CABLE),
            None,
        ),
        (
            Some(Reply::ToTv),
            "Can I cast my iPad to my TV?",
            Some(TV_CAST),
            None,
        ),
        (
            Some(Reply::ToTv),
            "How can I mirror my Android to my TV?",
            Some(TV_CAST),
            None,
        ),
        (
            Some(Reply::ToTv),
            "How can I cast my iPhone to my TV?",
            Some(TV_CAST),
            None,
        ),
        (
            Some(Reply::ToTv),
            "Connect my tablet to the TV",
            Some(TV_CAST),
            None,
        ),
        (
            Some(Reply::ToTv),
            "How do I connect my PC to my TV?",
            Some(TV_CABLE),
            None,
        ),
        (
            Some(Reply::ToTv),
            "How do I connect my laptop and headphones to my TV?",
            Some(TV_CAST),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type the pound sign?",
            Some(POUND),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type the euro sign?",
            Some(EURO),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type the dollar sign?",
            Some(DOLLAR),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type the at symbol?",
            Some(AT_SIGN),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type the hash sign?",
            Some(HASH_SIGN),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type a backslash?",
            Some(BACKSLASH),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type a forward slash?",
            Some(FORWARD_SLASH),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type an underscore?",
            Some(UNDERSCORE),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type a tilde?",
            Some(TILDE),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type a degree sign?",
            Some(DEGREE),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type an asterisk?",
            Some(ASTERISK),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type a pipe symbol?",
            Some(PIPE),
            None,
        ),
        (
            Some(Reply::Symbol),
            "Where is € on my keyboard?",
            Some(EURO),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type @ and #?",
            Some(AT_SIGN),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How can I type the euro and pound signs?",
            Some(POUND),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type an ampersand?",
            None,
            None,
        ),
        (
            Some(Reply::Symbol),
            "Where is the exclamation mark?",
            None,
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type \\?",
            Some(BACKSLASH),
            None,
        ),
        (
            Some(Reply::Symbol),
            "How do I type the sterling symbol?",
            Some(POUND),
            None,
        ),
        (
            Some(Reply::Symbol),
            "how do i type the email sign",
            Some(AT_SIGN),
            None,
        ),
        (
            Some(Reply::Recover),
            "How do I recover a deleted Word document?",
            Some(RECOVER_IN_BIN),
            None,
        ),
        (
            Some(Reply::Recover),
            "Recover deleted files from a USB stick",
            Some(RECOVER_STICK),
            None,
        ),
        (
            Some(Reply::Recover),
            "I deleted files off an SD card",
            Some(RECOVER_CARD),
            None,
        ),
        (
            Some(Reply::Recover),
            "Get a deleted file back on my external drive",
            Some(RECOVER_DRIVE),
            None,
        ),
        (
            Some(Reply::Recover),
            "Restore a deleted essay from an SD card",
            Some(RECOVER_IN_BIN),
            None,
        ),
        (
            Some(Reply::Recover),
            "Can I get back a deleted file in Google Drive?",
            Some(RECOVER_IN_BIN),
            None,
        ),
        (
            Some(Reply::Recover),
            "Can I recover a missing file on a flash drive?",
            Some(RECOVER_DRIVE),
            None,
        ),
        (
            Some(Reply::Recover),
            "Restore a deleted file from my USB drive",
            Some(RECOVER_DRIVE),
            None,
        ),
        (
            Some(Reply::Recover),
            "How do I restore a file I deleted from a folder on my hard drive?",
            Some(RECOVER_IN_BIN),
            None,
        ),
        (
            Some(Reply::Recover),
            "Can I recover deleted files from a USB stick and an SD card?",
            Some(RECOVER_CARD),
            None,
        ),
        (
            Some(Reply::Recover),
            "Recover a deleted file from a stick",
            Some(RECOVER_STICK),
            None,
        ),
        (
            Some(Reply::Definition),
            "What is a modem?",
            Some(MODEM),
            None,
        ),
        (
            Some(Reply::Definition),
            "Explain what cookies are",
            Some(COOKIE),
            None,
        ),
        (
            Some(Reply::Definition),
            "What is ransomware?",
            Some(RANSOMWARE),
            None,
        ),
        (
            Some(Reply::Definition),
            "What is phishing?",
            Some(PHISHING),
            None,
        ),
        (
            Some(Reply::Definition),
            "What is a phishing email?",
            Some(PHISHING),
            None,
        ),
        (
            Some(Reply::Definition),
            "What is a password manager?",
            Some(PASSWORD_MANAGER),
            None,
        ),
        (
            Some(Reply::Definition),
            "What does cloud backup mean?",
            Some(CLOUD),
            None,
        ),
        (
            Some(Reply::Definition),
            "What's the difference between a modem and a router?",
            Some(MODEM),
            None,
        ),
        (
            Some(Reply::Definition),
            "What is my modem doing flashing red?",
            None,
            None,
        ),
        (
            Some(Reply::Definition),
            "What is a cookie recipe?",
            None,
            None,
        ),
        (
            Some(Reply::Definition),
            "What is the cloud forecast?",
            None,
            None,
        ),
        (
            Some(Reply::Definition),
            "What is my password manager?",
            None,
            None,
        ),
        (
            Some(Reply::Definition),
            "I clicked on a phishing text",
            None,
            None,
        ),
        (
            Some(Reply::Definition),
            "What is ransomware removal?",
            None,
            None,
        ),
        (
            Some(Reply::Definition),
            "What is a modem and cloud storage?",
            Some(MODEM),
            None,
        ),
        (
            Some(Reply::Definition),
            "What is a cookie for baking or a password manager?",
            Some(PASSWORD_MANAGER),
            None,
        ),
        (Some(Reply::Definition), "What is a USB port?", None, None),
        (
            Some(Reply::Definition),
            "What is the cloud?",
            Some(CLOUD),
            None,
        ),
        (
            Some(Reply::Definition),
            "What is the best password manager?",
            None,
            None,
        ),
        (None, "How do I make the text bigger?", None, Some(true)),
        (
            None,
            "My shoes are too small, how do I fix them?",
            None,
            Some(false),
        ),
        (None, "How do I zoom in on my laptop?", None, Some(true)),
        (None, "How do I make it bigger?", None, Some(false)),
        (None, "How do I make the writing bigger?", None, Some(true)),
        (None, "Text size is too small in SETTINGS", None, Some(true)),
        (None, "An attachment is too big to send", None, Some(false)),
        (None, "I cannot read this tiny print", None, Some(true)),
        (None, "My application is too small", None, Some(false)),
        (None, "The monitor is too small", None, Some(false)),
    ];
    for &(form, line, expected, expected_guard) in cases {
        let reading = Reading::new(line);
        let offered = form.and_then(|form| reply(form, &reading));
        assert_eq!(offered.as_deref(), expected, "{form:?} for {line:?}");
        if let Some(expected) = expected_guard {
            let actual = match form {
                None => screen_named(&reading),
                Some(Reply::EmptyTrash) => computer_bin(&reading),
                other => panic!("unexpected guard for {other:?}"),
            };
            assert_eq!(actual, expected, "{form:?} guard for {line:?}");
        }
    }
}

const MAC_TRASH: &str = "On a Mac, open the Bin from the Dock and press Empty in the top right, or press Command-Shift-Delete to do it from the keyboard; holding Option while you press Empty skips the question. Nothing is freed until the Bin is emptied, so a full disk is often just this. On Windows, right-click the Recycle Bin on the desktop and choose Empty Recycle Bin, or open it and press Empty Recycle Bin at the top; the files are only really gone once you do, so a full disk is often just this.";

const WINDOWS_TRASH: &str = "On Windows, right-click the Recycle Bin on the desktop and choose Empty Recycle Bin, or open it and press Empty Recycle Bin at the top; the files are only really gone once you do, so a full disk is often just this. On a Mac, open the Bin from the Dock and press Empty in the top right, or press Command-Shift-Delete to do it from the keyboard; holding Option while you press Empty skips the question. Nothing is freed until the Bin is emptied, so a full disk is often just this.";

const MAC_FORCE_QUIT: &str = "On a Mac press Command-Option-Esc, pick the app in the list and press Force Quit: that closes the one app and leaves everything else open, and if the whole Mac is stuck, hold the power button for ten seconds instead. On Windows press Ctrl-Shift-Esc for Task Manager, pick the app and choose End task: that closes the one app and leaves everything else open, and if the whole machine is stuck, hold the power button for ten seconds instead.";

const WINDOWS_FORCE_QUIT: &str = "On Windows press Ctrl-Shift-Esc for Task Manager, pick the app and choose End task: that closes the one app and leaves everything else open, and if the whole machine is stuck, hold the power button for ten seconds instead. On a Mac press Command-Option-Esc, pick the app in the list and press Force Quit: that closes the one app and leaves everything else open, and if the whole Mac is stuck, hold the power button for ten seconds instead.";

const MAC_BROWSER: &str = "On a Mac, open System Settings > Desktop & Dock and scroll to Default web browser near the bottom (on older macOS it is Safari > Preferences > General > Default web browser). Pick the one you want from the menu and every link opens there from then on. On Windows, open Settings > Apps > Default apps, type the browser's name in the search box and choose Set default; that sends links, .html files and mail links to it.";

const WINDOWS_BROWSER: &str = "On Windows, open Settings > Apps > Default apps, type the browser's name in the search box and choose Set default; that sends links, .html files and mail links to it. On a Mac, open System Settings > Desktop & Dock and scroll to Default web browser near the bottom (on older macOS it is Safari > Preferences > General > Default web browser). Pick the one you want from the menu and every link opens there from then on.";

const MAC_BATTERY: &str = "On a Mac, hold Option and click the Apple menu > System Information > Power: the cycle count and the full charge capacity sit beside the design capacity, and under about 70% is a worn battery. On Windows, open Command Prompt and run `powercfg /batteryreport` - the report gives the design capacity and what it holds now, and under about 70% is a worn battery. A laptop battery is usually £40 to £90 and most are held in with screws, so it is worth replacing rather than buying a new machine.";

const WINDOWS_BATTERY: &str = "On Windows, open Command Prompt and run `powercfg /batteryreport` - the report gives the design capacity and what it holds now, and under about 70% is a worn battery. On a Mac, hold Option and click the Apple menu > System Information > Power: the cycle count and the full charge capacity sit beside the design capacity, and under about 70% is a worn battery. A laptop battery is usually £40 to £90 and most are held in with screws, so it is worth replacing rather than buying a new machine.";

const MAC_UNINSTALL: &str = "On a Mac, open Finder > Applications, drag the app to the Bin and empty it - some leave a folder in Library, which is harmless to leave. Windows: Settings > Apps > Installed apps, find it in the list and choose Uninstall. A program that will not go usually has its own uninstall entry, under its maker's name, in the Start menu.";

const WINDOWS_UNINSTALL: &str = "Windows: Settings > Apps > Installed apps, find it in the list and choose Uninstall. On a Mac, open Finder > Applications, drag the app to the Bin and empty it - some leave a folder in Library, which is harmless to leave. A program that will not go usually has its own uninstall entry, under its maker's name, in the Start menu.";

const TV_CABLE: &str = "A cable is the reliable way: an HDMI lead from the laptop to the TV, then press Source or Input on the TV remote and pick the socket you used; a laptop with no HDMI port needs a USB-C to HDMI adapter, which does the same job. Once the picture is up, pick the TV as the sound output - the speaker icon in the Windows tray, or System Settings > Sound on a Mac - because the laptop keeps playing sound otherwise. No cable to hand and the app has a Cast icon: that works too, with both on the same wifi.";

const TV_CAST: &str = "Cast it rather than cable it: put the phone and the TV on the same wifi, open the app (YouTube, iPlayer, Netflix) and tap the Cast or AirPlay icon at the top of the video, then pick the TV. For the whole screen, iPhone use Screen Mirroring in Control Centre with an Apple TV or an AirPlay set, Android use Smart View or Cast in the quick settings. If the TV does not appear, it is the wifi band: both have to be on the same network, and a guest network or the 5GHz-only setting hides it. A Lightning or USB-C to HDMI adapter is the wired fallback and never drops.";

const POUND: &str = "On a UK keyboard £ is Shift and 3. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const EURO: &str = "On a UK keyboard € is AltGr and 4, or Ctrl and Alt and 4 where there is no AltGr key. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const DOLLAR: &str = "On a UK keyboard $ is Shift and 4. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const AT_SIGN: &str = "On a UK keyboard @ is Shift and 2. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const HASH_SIGN: &str = "On a UK keyboard # is the key next to Enter, on the third row up - Shift and 3 gives # on a keyboard set to the US layout. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const BACKSLASH: &str = "On a UK keyboard the backslash is the key directly above Enter, and Shift and that key gives the pipe |. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const FORWARD_SLASH: &str = "On a UK keyboard / is the key next to the right Shift. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const UNDERSCORE: &str = "On a UK keyboard _ is Shift and the minus key. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const TILDE: &str = "On a UK keyboard ~ is Shift and the key left of the 1, or Option and N then space on a Mac. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const DEGREE: &str = "On a UK keyboard there is no degree key: hold Alt and type 0176 on Windows, or Option and Shift and 8 on a Mac. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const ASTERISK: &str = "On a UK keyboard * is Shift and 8. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const PIPE: &str = "On a UK keyboard | is Shift and the backslash key, above Enter. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac.";

const RECOVER_IN_BIN: &str = "Check the Recycle Bin on Windows or the Bin on a Mac first, and in Word or Excel open File > Info > Manage Document > Recover Unsaved Documents, which holds copies for a few days even when you never saved. If the file lived in OneDrive or Google Drive, right-click it and open Version history, or look in the drive's own bin: both keep deleted files for 30 days.";

const RECOVER_STICK: &str = "Stop using the stick now, and do not save anything else to it: a deleted file is still there under the space a new one would take, and every file you add can cover it. Plug it into a computer and run a recovery tool that only reads the stick - Recuva on Windows, or PhotoRec on any system, both free - and run the deep scan, because a quick one misses the older files. If it was pulled out without ejecting, the files may be there and the index damaged, so run the computer's own check (chkdsk on Windows, First Aid on a Mac) before you give up: that fixes the view of the file, not the file itself.";

const RECOVER_CARD: &str = "Stop using the card now, and do not save anything else to it: a deleted file is still there under the space a new one would take, and every file you add can cover it. Plug it into a computer and run a recovery tool that only reads the card - Recuva on Windows, or PhotoRec on any system, both free - and run the deep scan, because a quick one misses the older files. If it was pulled out without ejecting, the files may be there and the index damaged, so run the computer's own check (chkdsk on Windows, First Aid on a Mac) before you give up: that fixes the view of the file, not the file itself.";

const RECOVER_DRIVE: &str = "Stop using the drive now, and do not save anything else to it: a deleted file is still there under the space a new one would take, and every file you add can cover it. Plug it into a computer and run a recovery tool that only reads the drive - Recuva on Windows, or PhotoRec on any system, both free - and run the deep scan, because a quick one misses the older files. If it was pulled out without ejecting, the files may be there and the index damaged, so run the computer's own check (chkdsk on Windows, First Aid on a Mac) before you give up: that fixes the view of the file, not the file itself.";

const MODEM: &str = "A modem turns the signal on the line coming into your home - fibre, cable or the phone socket - into a network connection, and the router is what shares that connection round the house. Most home boxes do both jobs in one, which is why people use the two words for the same box; the modem is the half that talks to the street.";

const COOKIE: &str = "A cookie is a small file a website asks your browser to store: the note that keeps you signed in, the contents of your basket, or an ID that follows you between pages. It is not a program and it cannot read your files - the worst of them is tracking - and clearing them signs you out of most sites.";

const RANSOMWARE: &str = "Ransomware is a kind of malware that scrambles your files and then sells you the key: a message appears, the documents and photos will not open, and it wants money, usually in cryptocurrency. The one thing that reliably saves you is a backup you were not connected to when it struck, and paying often gets nothing back, so unplug the machine and get help before you do anything else.";

const PHISHING: &str = "Phishing is a fake message dressed up as your bank, a courier or HMRC to get you to log in or pay: the link goes to a page that looks right and the details go to the thief. The tell is the address it really came from and the urgency, so do not use the link - open the company's app or type the address yourself - and report it as phishing.";

const PASSWORD_MANAGER: &str = "A password manager is an app that keeps your passwords encrypted behind one master password or your face, and fills them in for you: Bitwarden, 1Password, the one built into Chrome, Safari and iCloud. It can invent a long random password for every site, which is the point - you only ever remember the one that opens the manager.";

const CLOUD: &str = "Cloud storage is your files kept on someone else's servers and copied to your devices: iCloud, Google Drive, OneDrive, Dropbox. It is a copy rather than a backup if it syncs the deletion too - keep one copy that is not attached, or the ransomware or the accidental delete reaches it as well.";
