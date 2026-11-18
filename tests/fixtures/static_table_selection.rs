//! Exact source table scans, including overlap, fallback, and abstention.

pub(super) type Case = (
    &'static str,
    &'static str,
    &'static str,
    Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    (
        "cooking",
        "deseed-chilli",
        "how do i deseed a chilli",
        Some(
            "Cut the stalk off and halve it lengthways, then scrape the seeds and the pale pith out with the tip of a knife or a teaspoon - that pith is where most of the heat sits, so take it out too if you want the chilli mild. Slice it on a board, not in your hand, and keep your hands away from your eyes and your mouth. Wash the board, the knife and your hands in cold water and soap straight after: the oil that burns is not washed off by rinsing. If your hands do burn, rub them with oil then wash again, or soak them in milk, not water.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "how should i take the seeds out of a pomegranate",
        Some(
            "Cut the crown off and score the skin down the ridges, then break it open in a bowl of water and knock the seeds out with a wooden spoon: the seeds sink, the pith and the membrane float and you can skim them off. Doing it under water is what stops the juice reaching your shirt, and it is much quicker than picking them out.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "remove the seeds from a cucumber",
        Some(
            "Halve it lengthways and run a teaspoon down the middle to scoop the watery seeds out - a grapefruit spoon has the right edge for it. Salt the halves and leave them face down for fifteen minutes afterwards and the water comes out too, which is what a soggy sandwich is.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "how do i deseed a tomato",
        Some(
            "Halve it across the middle, not top to bottom, and squeeze or scoop the seeds out with a finger or a teaspoon. The jelly round them is the flavour, so keep it if you are cooking; for a sandwich or a salad, a sieve catches the juice and you can salt the halves and leave them upside down for ten minutes instead.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "how do i deseed a watermelon",
        Some(
            "Halve it and scoop the seeds out with a spoon, then rinse the last of the strings off under the tap. A watermelon is easier cut into wedges and the seeds flicked out with the tip of a knife, and if you want it neat, cut it in slices and stamp the seeds out with a small round cutter.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "how do i deseed a pumpkin",
        Some(
            "Halve it and scoop the strings and seeds out with a metal spoon, working round the edge towards the middle. Keep the seeds: rinse them, toss in a little oil and salt and roast them at 180C for 10 to 15 minutes.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "a courgette's seeds",
        Some(
            "Halve it lengthways and run a teaspoon down the middle to scoop the watery seeds out - a grapefruit spoon has the right edge for it. Salt the halves and leave them face down for fifteen minutes afterwards and the water comes out too, which is what a soggy sandwich is.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "how do i deseed tomatoes and squash",
        Some(
            "Halve it across the middle, not top to bottom, and squeeze or scoop the seeds out with a finger or a teaspoon. The jelly round them is the flavour, so keep it if you are cooking; for a sandwich or a salad, a sieve catches the juice and you can salt the halves and leave them upside down for ten minutes instead.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "how do i take the seeds out of chilli and tomatoes",
        Some(
            "Halve it across the middle, not top to bottom, and squeeze or scoop the seeds out with a finger or a teaspoon. The jelly round them is the flavour, so keep it if you are cooking; for a sandwich or a salad, a sieve catches the juice and you can salt the halves and leave them upside down for ten minutes instead.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "what about melons",
        Some(
            "Halve it and scoop the seeds out with a spoon, then rinse the last of the strings off under the tap. A watermelon is easier cut into wedges and the seeds flicked out with the tip of a knife, and if you want it neat, cut it in slices and stamp the seeds out with a small round cutter.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "can I deseed a big marrow",
        Some(
            "Halve it lengthways and run a teaspoon down the middle to scoop the watery seeds out - a grapefruit spoon has the right edge for it. Salt the halves and leave them face down for fifteen minutes afterwards and the water comes out too, which is what a soggy sandwich is.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "how do i take apart a banana",
        Some(
            "Cut the stalk off and halve it lengthways, then scrape the seeds and the pale pith out with the tip of a knife or a teaspoon - that pith is where most of the heat sits, so take it out too if you want the chilli mild. Slice it on a board, not in your hand, and keep your hands away from your eyes and your mouth. Wash the board, the knife and your hands in cold water and soap straight after: the oil that burns is not washed off by rinsing. If your hands do burn, rub them with oil then wash again, or soak them in milk, not water.",
        ),
    ),
    (
        "cooking",
        "deseed-chilli",
        "I have cherry tomatoes",
        Some(
            "Halve it across the middle, not top to bottom, and squeeze or scoop the seeds out with a finger or a teaspoon. The jelly round them is the flavour, so keep it if you are cooking; for a sandwich or a salad, a sieve catches the juice and you can salt the halves and leave them upside down for ten minutes instead.",
        ),
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what can i cook for a date night",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what should i cook for a picnic",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "food for people who ask",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "I'm going camping, what should I cook",
        Some(
            "One pot, no fridge: pasta with tinned tomatoes, tinned beans and a chopped chorizo or a tin of tuna, or a curry from a jar with rice. Take food that keeps without cooling - tins, dried pasta, noodles, porridge oats, bread, hard cheese, apples - and a small gas ring plus matches in a plastic bag, because a wet night in a tent is when the cooking has to be simple.",
        ),
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what can i make for bonfire night",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what should i cook for christmas",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "a packed lunch for a school trip",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what do i cook for a long drive",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what should i cook for a movie night",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what can i cook in a heatwave",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what can i make for pancake day",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what to cook for 6 people at short notice",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what can i cook for a picnic on a snowy day",
        None,
    ),
    (
        "cooking_gaps",
        "meal-for-the-situation",
        "what can i cook for dinner",
        None,
    ),
    ("cooking_gaps", "meal-for-the-situation", "hello", None),
    (
        "technology",
        "delete-social-account",
        "how do i delete my facebook account",
        Some(
            "On Facebook it is Settings > Account ownership and control > Deactivation and deletion, or the menu under your profile picture on a computer. Deactivating is reversible and keeps your data, deleting is not, and there is a 30-day window to change your mind.",
        ),
    ),
    (
        "technology",
        "delete-social-account",
        "how do i delete my instagram account",
        Some(
            "On Instagram it is Settings > Accounts Centre > Personal details > Account ownership and control > Delete account, and it will ask why. Deactivating is reversible and keeps your photos; deleting is not, and there is a 30-day window to change your mind.",
        ),
    ),
    (
        "technology",
        "delete-social-account",
        "how do i delete my tiktok account",
        Some(
            "On TikTok it is Profile > the three lines > Settings and privacy > Account > Deactivate or delete account. Deactivating is reversible; deleting is not, and there is a 30-day window to change your mind.",
        ),
    ),
    (
        "technology",
        "delete-social-account",
        "how do i delete my snapchat account",
        Some(
            "On Snapchat it is Settings > Account Actions > Delete Account, and it asks for your password. There is a 30-day window to log back in and cancel it.",
        ),
    ),
    (
        "technology",
        "delete-social-account",
        "how do i delete my twitter account",
        Some(
            "On X it is Settings > Your account > Deactivate your account. It is reversible for 30 days, then the account is permanently deleted.",
        ),
    ),
    (
        "technology",
        "delete-social-account",
        "how do i delete my account",
        Some(
            "Do it in the app, not by deleting the icon: every app hides it under Settings > Account, usually as Deactivate (reversible) or Delete (permanent). Deactivating keeps your posts and your name; deleting does not, and most apps give you 30 days to log back in and change your mind.",
        ),
    ),
    (
        "technology",
        "delete-social-account",
        "how do i delete facebook and instagram",
        Some(
            "On Facebook it is Settings > Account ownership and control > Deactivation and deletion, or the menu under your profile picture on a computer. Deactivating is reversible and keeps your data, deleting is not, and there is a 30-day window to change your mind.",
        ),
    ),
    (
        "technology",
        "delete-social-account",
        "hello",
        Some(
            "Do it in the app, not by deleting the icon: every app hides it under Settings > Account, usually as Deactivate (reversible) or Delete (permanent). Deactivating keeps your posts and your name; deleting does not, and most apps give you 30 days to log back in and change your mind.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my amazon account",
        Some(
            "On Amazon scroll to the bottom of Account & Lists > Your Account and use Close Your Amazon Account, which asks you to confirm by email. Orders and gift card balances go with it, so spend or download anything you have paid for first.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my ebay account",
        Some(
            "On eBay it is Account > Close account, at the bottom of the account settings page, and it will not let you close while you have a live listing, an unpaid fee or an open case. Withdraw any money in your eBay balance first, or it is kept.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my paypal account",
        Some(
            "On PayPal it is Settings > Account > Close your account, and it only lets you when the balance is zero, so move any money to your bank first. A limitation or an open dispute blocks it until that is settled.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my google account",
        Some(
            "On Google it is myaccount.google.com > Data & privacy > Delete a Google service to drop just Gmail or YouTube, or Delete your account for the lot. Download what you want first with takeout.google.com, and check the address is not the recovery email for other logins.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my apple account",
        Some(
            "On Apple it is privacy.apple.com > Delete your account, which needs to be done from a device signed in to the same Apple ID. Cancel any subscriptions and spend any credit first, because a deleted account cannot be restored.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my microsoft account",
        Some(
            "On Microsoft it is account.microsoft.com > Close your account, and any unused credit is lost, so use it first. Cancel Microsoft 365 and any Xbox subscriptions beforehand, or they keep trying to charge a card.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my netflix account",
        Some(
            "On Netflix the button is Cancel Membership on the account page, or netflix.com/cancelplan, and the account stays usable until the end of the month you have paid for. Download anything from the downloads list before then.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my spotify account",
        Some(
            "On Spotify it is the account page > Close account at the bottom, and the free tier goes with it. Cancel any Premium plan first, or the payment keeps going while the account still opens.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my icloud account",
        Some(
            "On iCloud an Apple ID is the account: delete it at privacy.apple.com and the mail, photos and backups go too. Download a copy of your data first with privacy.apple.com > Get a copy of your data.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "how do i close my dropbox account",
        Some(
            "On Dropbox it is Settings > Account > Delete account, and shared folders you own are deleted for everyone in them, so hand ownership over first. Download your files before you do.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "what's the last step to close an account",
        Some(
            "Close it from the service's own website in a browser rather than from the app: the button is usually the last item in your account or profile settings, called Delete or Close account. Spend or download anything you have paid for first - credit and files are lost - and shut down any subscription on the same account, or it keeps billing.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "I want to stop paying for spotified music",
        Some(
            "Close it from the service's own website in a browser rather than from the app: the button is usually the last item in your account or profile settings, called Delete or Close account. Spend or download anything you have paid for first - credit and files are lost - and shut down any subscription on the same account, or it keeps billing.",
        ),
    ),
    (
        "technology",
        "close-a-service-account",
        "hello",
        Some(
            "Close it from the service's own website in a browser rather than from the app: the button is usually the last item in your account or profile settings, called Delete or Close account. Spend or download anything you have paid for first - credit and files are lost - and shut down any subscription on the same account, or it keeps billing.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "which London airport is closest to the centre",
        Some(
            "London City is the closest airport to the centre: about 25 minutes to Bank on the DLR. Of the big four, Heathrow is the quickest into town - the Elizabeth line takes 30 minutes to Paddington for about 13 pounds.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "how do i get from Heathrow airport to London",
        Some(
            "From Heathrow the Elizabeth line is 30 minutes to Paddington for about 13 pounds, the Piccadilly line about an hour for about 5.60, and the Heathrow Express 15 minutes to Paddington for about 25. A black cab is the expensive option: the train beats it into the centre every time.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "how do i get from Gatwick to central London",
        Some(
            "From Gatwick the train to Victoria takes 30 to 40 minutes: the Gatwick Express is about 20 pounds, the ordinary Southern or Thameslink train about half that and only ten minutes slower. A black cab is the expensive option: the train beats it into the centre every time.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "how do i get from Stansted to town",
        Some(
            "From Stansted the Stansted Express runs to Liverpool Street in about 47 minutes for around 20 pounds, and a coach to Victoria is cheaper but slower. A black cab is the expensive option: the train beats it into the centre every time.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "how do i get from Luton to London",
        Some(
            "From Luton the shuttle bus runs to Luton Airport Parkway, then a fast train to St Pancras takes about 25 minutes - book the bundle and it is cheaper. A black cab is the expensive option: the train beats it into the centre every time.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "how do i get from London City to Bank",
        Some(
            "London City is the closest airport to the centre: about 25 minutes to Bank on the DLR. Of the big four, Heathrow is the quickest into town - the Elizabeth line takes 30 minutes to Paddington for about 13 pounds.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "train from Gatwick airport",
        Some(
            "From Gatwick the train to Victoria takes 30 to 40 minutes: the Gatwick Express is about 20 pounds, the ordinary Southern or Thameslink train about half that and only ten minutes slower. A black cab is the expensive option: the train beats it into the centre every time.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "how do I get from London Southend airport",
        Some(
            "London City is the closest airport to the centre: about 25 minutes to Bank on the DLR. Of the big four, Heathrow is the quickest into town - the Elizabeth line takes 30 minutes to Paddington for about 13 pounds.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "what time should we get to Heathrow airport for a 6am flight",
        Some(
            "From Heathrow the Elizabeth line is 30 minutes to Paddington for about 13 pounds, the Piccadilly line about an hour for about 5.60, and the Heathrow Express 15 minutes to Paddington for about 25. A black cab is the expensive option: the train beats it into the centre every time.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "how early for the flight",
        Some(
            "London City is the closest airport to the centre: about 25 minutes to Bank on the DLR. Of the big four, Heathrow is the quickest into town - the Elizabeth line takes 30 minutes to Paddington for about 13 pounds.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "what time is the London bus",
        None,
    ),
    (
        "travel",
        "london-airport",
        "travel by train from York to London",
        None,
    ),
    (
        "travel",
        "london-airport",
        "what is the nearest terminal",
        Some(
            "London City is the closest airport to the centre: about 25 minutes to Bank on the DLR. Of the big four, Heathrow is the quickest into town - the Elizabeth line takes 30 minutes to Paddington for about 13 pounds.",
        ),
    ),
    (
        "travel",
        "london-airport",
        "how do i get to Birmingham airport",
        Some(
            "London City is the closest airport to the centre: about 25 minutes to Bank on the DLR. Of the big four, Heathrow is the quickest into town - the Elizabeth line takes 30 minutes to Paddington for about 13 pounds.",
        ),
    ),
];
