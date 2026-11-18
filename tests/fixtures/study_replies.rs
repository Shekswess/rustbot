//! Original PyBot phrasebook and study-card results at the pinned revision.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub Option<bool>,
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "say-language",
        "how do you say toilet in portuguese",
        Some(true),
        Some(
            "'Toilet' in Portuguese is Onde fica a casa de banho? — say it roughly \"ON-dy FEE-ka a KA-za dy BAN-yo\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in greek",
        Some(true),
        Some(
            "'Toilet' in Greek is Poú eínai i toualéta? (πού είναι η τουαλέτα) — say it roughly \"poo EE-neh ee too-a-LEH-ta\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in turkish",
        Some(true),
        Some(
            "'Toilet' in Turkish is Tuvalet nerede? — say it roughly \"too-va-LET NEH-reh-deh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in welsh",
        Some(true),
        Some(
            "'Toilet' in Welsh is Ble mae'r toiled? — say it roughly \"blay my-re TOY-led\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in japanese",
        Some(true),
        Some(
            "'Toilet' in Japanese is Toire wa doko desu ka (トイレはどこですか) — say it roughly \"TOY-reh wah DOH-koh dess kah\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in chinese",
        Some(true),
        Some(
            "'Toilet' in Chinese is Xǐshǒujiān zài nǎr? (洗手间在哪儿) — say it roughly \"shee-show-jyen dzai nar\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in dutch",
        Some(true),
        Some(
            "'Toilet' in Dutch is Waar is het toilet? — say it roughly \"vaar is hut twa-LET\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in spanish",
        Some(true),
        Some(
            "'Toilet' in Spanish is ¿Dónde está el baño? — say it roughly \"DON-day es-TA el BAN-yo\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in french",
        Some(true),
        Some(
            "'Toilet' in French is Où sont les toilettes ? — say it roughly \"oo son lay twa-LET\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in italian",
        Some(true),
        Some(
            "'Toilet' in Italian is Dov'è il bagno? — say it roughly \"do-VEH eel BAN-yo\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in german",
        Some(true),
        Some(
            "'Toilet' in German is Wo ist die Toilette? — say it roughly \"vo ist dee toy-LET-tuh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say toilet in irish",
        Some(true),
        Some(
            "'Toilet' in Irish is Cá bhfuil an leithreas? — say it roughly \"kaw will an LEH-rass\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in portuguese",
        Some(true),
        Some(
            "'Thank you' in Portuguese is Obrigado (a woman says obrigada) — say it roughly \"oh-bree-GAH-doo\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in greek",
        Some(true),
        Some(
            "'Thank you' in Greek is Efcharistó (ευχαριστώ) — say it roughly \"ef-kha-ree-STOH\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in turkish",
        Some(true),
        Some(
            "'Thank you' in Turkish is Teşekkürler — say it roughly \"teh-shek-kewr-LER\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in welsh",
        Some(true),
        Some(
            "'Thank you' in Welsh is Diolch — say it roughly \"DEE-olch\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in japanese",
        Some(true),
        Some(
            "'Thank you' in Japanese is Arigatou (ありがとう) — say it roughly \"a-ree-GA-toh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in chinese",
        Some(true),
        Some(
            "'Thank you' in Chinese is Xièxie (谢谢) — say it roughly \"shyeh-shyeh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in dutch",
        Some(true),
        Some(
            "'Thank you' in Dutch is Dank je wel — say it roughly \"dank yuh vel\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in spanish",
        Some(true),
        Some(
            "'Thank you' in Spanish is Gracias — say it roughly \"GRA-thee-as\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in french",
        Some(true),
        Some(
            "'Thank you' in French is Merci — say it roughly \"mair-SEE\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in italian",
        Some(true),
        Some(
            "'Thank you' in Italian is Grazie — say it roughly \"GRAT-zee-eh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in german",
        Some(true),
        Some(
            "'Thank you' in German is Danke — say it roughly \"DAN-kuh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in irish",
        Some(true),
        Some(
            "'Thank you' in Irish is Go raibh maith agat — say it roughly \"guh rev MAH-ut uh-gut\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say thank you in gaelic",
        Some(true),
        Some(
            "'Thank you' in Scottish Gaelic is Tapadh leat — say it roughly \"TA-pah lat\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in portuguese",
        Some(true),
        Some(
            "'Please' in Portuguese is Por favor — say it roughly \"por fa-VOR\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in greek",
        Some(true),
        Some(
            "'Please' in Greek is Parakaló (παρακαλώ) — say it roughly \"pa-ra-ka-LOH\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in turkish",
        Some(true),
        Some(
            "'Please' in Turkish is Lütfen — say it roughly \"LEWT-fen\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in welsh",
        Some(true),
        Some(
            "'Please' in Welsh is Os gwelwch yn dda — say it roughly \"os GWEL-ookh un thah\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in japanese",
        Some(true),
        Some(
            "'Please' in Japanese is Onegaishimasu (お願いします) — say it roughly \"o-neh-GUY-shee-mas\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in chinese",
        Some(true),
        Some(
            "'Please' in Chinese is Qǐng (请) — say it roughly \"ching\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in dutch",
        Some(true),
        Some(
            "'Please' in Dutch is Alstublieft — say it roughly \"als-too-BLEEFT\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in spanish",
        Some(true),
        Some(
            "'Please' in Spanish is Por favor — say it roughly \"por fa-VOR\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in french",
        Some(true),
        Some(
            "'Please' in French is S'il vous plaît — say it roughly \"seel voo PLEH\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in italian",
        Some(true),
        Some(
            "'Please' in Italian is Per favore — say it roughly \"per fa-VO-reh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in german",
        Some(true),
        Some(
            "'Please' in German is Bitte — say it roughly \"BIT-tuh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in irish",
        Some(true),
        Some(
            "'Please' in Irish is Le do thoil — say it roughly \"leh duh HULL\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say please in gaelic",
        Some(true),
        Some(
            "'Please' in Scottish Gaelic is Mas e do thoil e — say it roughly \"mas eh daw HOLL eh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in portuguese",
        Some(true),
        Some(
            "'Hello' in Portuguese is Olá — say it roughly \"oh-LAH\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in greek",
        Some(true),
        Some(
            "'Hello' in Greek is Kaliméra (καλημέρα) — say it roughly \"ka-lee-MEH-ra\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in turkish",
        Some(true),
        Some(
            "'Hello' in Turkish is Merhaba — say it roughly \"mer-ha-BAH\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in welsh",
        Some(true),
        Some(
            "'Hello' in Welsh is Helo — say it roughly \"HEH-loh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in japanese",
        Some(true),
        Some(
            "'Hello' in Japanese is Konnichiwa (こんにちは) — say it roughly \"kon-nee-CHEE-wah\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in chinese",
        Some(true),
        Some(
            "'Hello' in Chinese is Nǐ hǎo (你好) — say it roughly \"nee HOW\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in dutch",
        Some(true),
        Some(
            "'Hello' in Dutch is Hallo — say it roughly \"HAL-oh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in spanish",
        Some(true),
        Some(
            "'Hello' in Spanish is Hola — say it roughly \"OH-la\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in french",
        Some(true),
        Some(
            "'Hello' in French is Bonjour — say it roughly \"bon-ZHOOR\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in italian",
        Some(true),
        Some(
            "'Hello' in Italian is Ciao — say it roughly \"chow\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in german",
        Some(true),
        Some(
            "'Hello' in German is Hallo — say it roughly \"HAL-oh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in irish",
        Some(true),
        Some(
            "'Hello' in Irish is Dia duit — say it roughly \"DEE-uh gwit\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say hello in gaelic",
        Some(true),
        Some(
            "'Hello' in Scottish Gaelic is Halò — say it roughly \"ha-LAW\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in portuguese",
        Some(true),
        Some(
            "'Goodbye' in Portuguese is Adeus — say it roughly \"a-DEHS\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in greek",
        Some(true),
        Some(
            "'Goodbye' in Greek is Antío (αντίο) — say it roughly \"an-DEE-oh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in turkish",
        Some(true),
        Some(
            "'Goodbye' in Turkish is Hoşça kalın — say it roughly \"hosh-cha ka-LUN\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in welsh",
        Some(true),
        Some(
            "'Goodbye' in Welsh is Hwyl fawr — say it roughly \"hoo-il vowr\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in japanese",
        Some(true),
        Some(
            "'Goodbye' in Japanese is Sayounara (さようなら) — say it roughly \"sa-YOH-na-ra\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in chinese",
        Some(true),
        Some(
            "'Goodbye' in Chinese is Zàijiàn (再见) — say it roughly \"dzai-jyen\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in dutch",
        Some(true),
        Some(
            "'Goodbye' in Dutch is Tot ziens — say it roughly \"tot zeens\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in spanish",
        Some(true),
        Some(
            "'Goodbye' in Spanish is Adiós — say it roughly \"a-DYOS\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in french",
        Some(true),
        Some(
            "'Goodbye' in French is Au revoir — say it roughly \"oh ruh-VWAR\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in italian",
        Some(true),
        Some(
            "'Goodbye' in Italian is Arrivederci — say it roughly \"a-ree-veh-DAIR-chee\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in german",
        Some(true),
        Some(
            "'Goodbye' in German is Auf Wiedersehen — say it roughly \"owf VEE-der-zayn\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in irish",
        Some(true),
        Some(
            "'Goodbye' in Irish is Slán — say it roughly \"slawn\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say goodbye in gaelic",
        Some(true),
        Some(
            "'Goodbye' in Scottish Gaelic is Mar sin leat — say it roughly \"mar shin lat\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in portuguese",
        Some(true),
        Some(
            "'How much' in Portuguese is Quanto custa? — say it roughly \"KWAN-too KOOSH-ta\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in greek",
        Some(true),
        Some(
            "'How much' in Greek is Póso kánei? (πόσο κάνει) — say it roughly \"POH-so KA-nee\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in turkish",
        Some(true),
        Some(
            "'How much' in Turkish is Ne kadar? — say it roughly \"neh ka-DAR\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in welsh",
        Some(true),
        Some(
            "'How much' in Welsh is Faint yw hyn? — say it roughly \"fynt yoo hin\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in japanese",
        Some(true),
        Some(
            "'How much' in Japanese is Ikura desu ka (いくらですか) — say it roughly \"ee-KOO-rah dess kah\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in chinese",
        Some(true),
        Some(
            "'How much' in Chinese is Duōshao qián? (多少钱) — say it roughly \"dwaw-shau chyen\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in dutch",
        Some(true),
        Some(
            "'How much' in Dutch is Hoeveel kost het? — say it roughly \"hoo-VAYL kost hut\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in spanish",
        Some(true),
        Some(
            "'How much' in Spanish is ¿Cuánto cuesta? — say it roughly \"KWAN-to KWES-ta\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in french",
        Some(true),
        Some(
            "'How much' in French is Combien ça coûte ? — say it roughly \"kom-BYAN sa KOOT\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in italian",
        Some(true),
        Some(
            "'How much' in Italian is Quanto costa? — say it roughly \"KWAN-to KOS-ta\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in german",
        Some(true),
        Some(
            "'How much' in German is Was kostet das? — say it roughly \"vas KOS-tet das\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say how much in irish",
        Some(true),
        Some(
            "'How much' in Irish is Cé mhéad? — say it roughly \"kay vayd\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say the bill in spanish",
        Some(true),
        Some(
            "'The bill' in Spanish is La cuenta, por favor — say it roughly \"la KWEN-ta por fa-VOR\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say the bill in french",
        Some(true),
        Some(
            "'The bill' in French is L'addition, s'il vous plaît — say it roughly \"la-dee-SYON seel voo PLEH\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say the bill in italian",
        Some(true),
        Some(
            "'The bill' in Italian is Il conto, per favore — say it roughly \"eel KON-to per fa-VO-reh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say the bill in german",
        Some(true),
        Some(
            "'The bill' in German is Die Rechnung, bitte — say it roughly \"dee REKH-noong BIT-tuh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say the bill in welsh",
        Some(true),
        Some(
            "'The bill' in Welsh is Y bil, os gwelwch yn dda — say it roughly \"uh bill os GWEL-ookh un thah\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say sorry in spanish",
        Some(true),
        Some(
            "'Sorry' in Spanish is Perdón — say it roughly \"per-DON\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say sorry in french",
        Some(true),
        Some(
            "'Sorry' in French is Pardon — say it roughly \"par-DON\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say sorry in italian",
        Some(true),
        Some(
            "'Sorry' in Italian is Scusi — say it roughly \"SKOO-zee\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say sorry in german",
        Some(true),
        Some(
            "'Sorry' in German is Entschuldigung — say it roughly \"ent-SHOOL-dee-goong\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say sorry in welsh",
        Some(true),
        Some(
            "'Sorry' in Welsh is Mae'n ddrwg gen i — say it roughly \"mine throog gen ee\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say sorry in irish",
        Some(true),
        Some(
            "'Sorry' in Irish is Tá brón orm — say it roughly \"taw brone or-um\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say i don't understand in spanish",
        Some(true),
        Some(
            "'I don't understand' in Spanish is No entiendo — say it roughly \"no en-TYEN-do\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say i don't understand in french",
        Some(true),
        Some(
            "'I don't understand' in French is Je ne comprends pas — say it roughly \"zhuh nuh kom-PRAN pa\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say i don't understand in italian",
        Some(true),
        Some(
            "'I don't understand' in Italian is Non capisco — say it roughly \"non ka-PEES-ko\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say i don't understand in german",
        Some(true),
        Some(
            "'I don't understand' in German is Ich verstehe nicht — say it roughly \"ikh fair-SHTAY-uh nikht\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say i don't understand in welsh",
        Some(true),
        Some(
            "'I don't understand' in Welsh is Dw i ddim yn deall — say it roughly \"doo ee thim un day-al\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say i don't understand in irish",
        Some(true),
        Some(
            "'I don't understand' in Irish is Ní thuigim — say it roughly \"nee HIG-im\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say do you speak english in spanish",
        Some(true),
        Some(
            "'Do you speak english' in Spanish is ¿Habla inglés? — say it roughly \"AB-la een-GLES\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say do you speak english in french",
        Some(true),
        Some(
            "'Do you speak english' in French is Parlez-vous anglais ? — say it roughly \"par-lay VOO on-GLEH\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say do you speak english in italian",
        Some(true),
        Some(
            "'Do you speak english' in Italian is Parla inglese? — say it roughly \"PAR-la een-GLEH-zeh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say do you speak english in german",
        Some(true),
        Some(
            "'Do you speak english' in German is Sprechen Sie Englisch? — say it roughly \"SHPREKH-en zee ENG-lish\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say do you speak english in welsh",
        Some(true),
        Some(
            "'Do you speak english' in Welsh is Ydych chi'n siarad Saesneg? — say it roughly \"UD-ikh kheen SHAR-ad SICE-neg\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say water in spanish",
        Some(true),
        Some(
            "'Water' in Spanish is Agua — say it roughly \"AH-gwa\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say water in french",
        Some(true),
        Some(
            "'Water' in French is De l'eau — say it roughly \"duh loh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say water in italian",
        Some(true),
        Some(
            "'Water' in Italian is Acqua — say it roughly \"AH-kwa\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say water in german",
        Some(true),
        Some(
            "'Water' in German is Wasser — say it roughly \"VAS-ser\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say water in welsh",
        Some(true),
        Some(
            "'Water' in Welsh is Dŵr — say it roughly \"door\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say water in irish",
        Some(true),
        Some(
            "'Water' in Irish is Uisce — say it roughly \"ISH-keh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say water in gaelic",
        Some(true),
        Some(
            "'Water' in Scottish Gaelic is Uisge — say it roughly \"OOSH-ka\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say help in spanish",
        Some(true),
        Some(
            "'Help' in Spanish is Socorro — say it roughly \"so-KO-ro\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say help in french",
        Some(true),
        Some(
            "'Help' in French is Au secours — say it roughly \"oh suh-KOOR\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say help in italian",
        Some(true),
        Some(
            "'Help' in Italian is Aiuto — say it roughly \"a-YOO-to\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say help in german",
        Some(true),
        Some(
            "'Help' in German is Hilfe — say it roughly \"HIL-fuh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say help in welsh",
        Some(true),
        Some(
            "'Help' in Welsh is Help! — say it roughly \"help\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say help in irish",
        Some(true),
        Some(
            "'Help' in Irish is Cabhair! — say it roughly \"KOW-ir\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say help in gaelic",
        Some(true),
        Some(
            "'Help' in Scottish Gaelic is Cuideachadh! — say it roughly \"KOO-jokh-ug\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good morning in spanish",
        Some(true),
        Some(
            "'Good morning' in Spanish is Buenos días — say it roughly \"BWEH-nos DEE-as\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good morning in french",
        Some(true),
        Some(
            "'Good morning' in French is Bonjour — say it roughly \"bon-ZHOOR\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good morning in italian",
        Some(true),
        Some(
            "'Good morning' in Italian is Buongiorno — say it roughly \"bwon-JOR-no\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good morning in german",
        Some(true),
        Some(
            "'Good morning' in German is Guten Morgen — say it roughly \"GOO-ten MOR-gen\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good morning in welsh",
        Some(true),
        Some(
            "'Good morning' in Welsh is Bore da — say it roughly \"BOR-eh da\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good morning in irish",
        Some(true),
        Some(
            "'Good morning' in Irish is Maidin mhaith — say it roughly \"MA-jin vah\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good morning in gaelic",
        Some(true),
        Some(
            "'Good morning' in Scottish Gaelic is Madainn mhath — say it roughly \"MA-din vah\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good night in spanish",
        Some(true),
        Some(
            "'Good night' in Spanish is Buenas noches — say it roughly \"BWEH-nas NO-chess\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good night in french",
        Some(true),
        Some(
            "'Good night' in French is Bonne nuit — say it roughly \"bon NWEE\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good night in italian",
        Some(true),
        Some(
            "'Good night' in Italian is Buonanotte — say it roughly \"bwo-na-NOT-teh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good night in german",
        Some(true),
        Some(
            "'Good night' in German is Gute Nacht — say it roughly \"GOO-tuh nakht\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good night in welsh",
        Some(true),
        Some(
            "'Good night' in Welsh is Nos da — say it roughly \"noss da\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say good night in irish",
        Some(true),
        Some(
            "'Good night' in Irish is Oíche mhaith — say it roughly \"EE-ha vah\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say cheers in spanish",
        Some(true),
        Some(
            "'Thank you' in Spanish is Gracias — say it roughly \"GRA-thee-as\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say cheers in french",
        Some(true),
        Some(
            "'Thank you' in French is Merci — say it roughly \"mair-SEE\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say cheers in italian",
        Some(true),
        Some(
            "'Thank you' in Italian is Grazie — say it roughly \"GRAT-zee-eh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say cheers in german",
        Some(true),
        Some(
            "'Thank you' in German is Danke — say it roughly \"DAN-kuh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say cheers in welsh",
        Some(true),
        Some(
            "'Thank you' in Welsh is Diolch — say it roughly \"DEE-olch\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say cheers in irish",
        Some(true),
        Some(
            "'Thank you' in Irish is Go raibh maith agat — say it roughly \"guh rev MAH-ut uh-gut\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "how do you say cheers in gaelic",
        Some(true),
        Some(
            "'Thank you' in Scottish Gaelic is Tapadh leat — say it roughly \"TA-pah lat\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language",
        "what is french food like",
        Some(false),
        None,
    ),
    Case(
        "say-language",
        "how do i say food in french",
        Some(true),
        Some(
            "Learn French as a set of whole phrases rather than word by word, and learn them out loud: 'hello', 'please', 'thank you', 'sorry, I don't understand', 'do you speak English?' and 'how much is that?' cover most of a short trip. Ask me any of those in one language and I will give you the phrase and how to say it.",
        ),
    ),
    Case(
        "say-language",
        "actually french",
        Some(true),
        Some(
            "Learn French as a set of whole phrases rather than word by word, and learn them out loud: 'hello', 'please', 'thank you', 'sorry, I don't understand', 'do you speak English?' and 'how much is that?' cover most of a short trip. Ask me any of those in one language and I will give you the phrase and how to say it.",
        ),
    ),
    Case(
        "say-language",
        "no let's learn spanish instead",
        Some(true),
        Some(
            "Learn Spanish as a set of whole phrases rather than word by word, and learn them out loud: 'hello', 'please', 'thank you', 'sorry, I don't understand', 'do you speak English?' and 'how much is that?' cover most of a short trip. Ask me any of those in one language and I will give you the phrase and how to say it.",
        ),
    ),
    Case(
        "say-language",
        "what was the french revolution",
        Some(false),
        Some(
            "Learn French as a set of whole phrases rather than word by word, and learn them out loud: 'hello', 'please', 'thank you', 'sorry, I don't understand', 'do you speak English?' and 'how much is that?' cover most of a short trip. Ask me any of those in one language and I will give you the phrase and how to say it.",
        ),
    ),
    Case("say-language", "It is in Maori", Some(false), None),
    Case(
        "say-language",
        "give me some Polish phrases",
        Some(false),
        None,
    ),
    Case("say-language", "do you speak english", Some(true), None),
    Case(
        "say-welsh-irish",
        "how do you say thanks in welsh",
        Some(true),
        Some(
            "'Thank you' in Welsh is Diolch — say it roughly \"DEE-olch\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-welsh-irish",
        "how do you say hello in welsh",
        Some(true),
        Some(
            "'Hello' in Welsh is Helo — say it roughly \"HEH-loh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-welsh-irish",
        "actually irish",
        Some(true),
        Some(
            "Learn Irish as a set of whole phrases rather than word by word, and learn them out loud: 'hello', 'please', 'thank you', 'sorry, I don't understand', 'do you speak English?' and 'how much is that?' cover most of a short trip. Ask me any of those in one language and I will give you the phrase and how to say it.",
        ),
    ),
    Case(
        "say-welsh-irish",
        "how do you say hello in gaelic",
        Some(true),
        Some(
            "'Hello' in Scottish Gaelic is Halò — say it roughly \"ha-LAW\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case("say-welsh-irish", "welcome in icelandic", Some(true), None),
    Case(
        "say-language-phrase",
        "how do you say hello in japanese",
        Some(true),
        Some(
            "'Hello' in Japanese is Konnichiwa (こんにちは) — say it roughly \"kon-nee-CHEE-wah\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "say-language-phrase",
        "translate good night into russian",
        Some(true),
        None,
    ),
    Case(
        "say-language-phrase",
        "what does faint mean in welsh",
        Some(true),
        Some(
            "Learn Welsh as a set of whole phrases rather than word by word, and learn them out loud: 'hello', 'please', 'thank you', 'sorry, I don't understand', 'do you speak English?' and 'how much is that?' cover most of a short trip. Ask me any of those in one language and I will give you the phrase and how to say it.",
        ),
    ),
    Case(
        "say-language-phrase",
        "whats hello in german",
        Some(true),
        Some(
            "'Hello' in German is Hallo — say it roughly \"HAL-oh\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does teşekkürler mean?",
        Some(true),
        Some(
            "That is Turkish for 'thank you'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does tesekkurler mean?",
        Some(true),
        Some(
            "That is Turkish for 'thank you'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does diolch mean?",
        Some(true),
        Some(
            "That is Welsh for 'thank you'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does gracias mean?",
        Some(true),
        Some(
            "That is Spanish for 'thank you'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does merci mean?",
        Some(true),
        Some(
            "That is French for 'thank you'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does grazie mean?",
        Some(true),
        Some(
            "That is Italian for 'thank you'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does danke mean?",
        Some(true),
        Some(
            "That is German for 'thank you'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does lütfen mean?",
        Some(true),
        Some(
            "That is Turkish for 'please'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does lutfen mean?",
        Some(true),
        Some(
            "That is Turkish for 'please'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does alstublieft mean?",
        Some(true),
        Some(
            "That is Dutch for 'please'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does bitte mean?",
        Some(true),
        Some(
            "That is German for 'please'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does olá mean?",
        Some(true),
        Some(
            "That is Portuguese for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does ola mean?",
        Some(true),
        Some(
            "That is Portuguese for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does merhaba mean?",
        Some(true),
        Some(
            "That is Turkish for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does helo mean?",
        Some(true),
        Some(
            "That is Welsh for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does hallo mean?",
        Some(true),
        Some(
            "That is Dutch for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does hola mean?",
        Some(true),
        Some(
            "That is Spanish for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does bonjour mean?",
        Some(true),
        Some(
            "That is French for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does ciao mean?",
        Some(true),
        Some(
            "That is Italian for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does halò mean?",
        Some(true),
        Some(
            "That is Scottish Gaelic for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does halo mean?",
        Some(true),
        Some(
            "That is Scottish Gaelic for 'hello'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does adeus mean?",
        Some(true),
        Some(
            "That is Portuguese for 'goodbye'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does adiós mean?",
        Some(true),
        Some(
            "That is Spanish for 'goodbye'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does adios mean?",
        Some(true),
        Some(
            "That is Spanish for 'goodbye'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does arrivederci mean?",
        Some(true),
        Some(
            "That is Italian for 'goodbye'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does slán mean?",
        Some(true),
        Some(
            "That is Irish for 'goodbye'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does slan mean?",
        Some(true),
        Some(
            "That is Irish for 'goodbye'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does perdón mean?",
        Some(true),
        Some(
            "That is Spanish for 'sorry'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does perdon mean?",
        Some(true),
        Some(
            "That is Spanish for 'sorry'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does pardon mean?",
        Some(true),
        Some(
            "That is French for 'sorry'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does scusi mean?",
        Some(true),
        Some(
            "That is Italian for 'sorry'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does entschuldigung mean?",
        Some(true),
        Some(
            "That is German for 'sorry'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does agua mean?",
        Some(true),
        Some(
            "That is Spanish for 'water'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does acqua mean?",
        Some(true),
        Some(
            "That is Italian for 'water'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does wasser mean?",
        Some(false),
        Some(
            "That is German for 'water'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does dŵr mean?",
        Some(true),
        Some(
            "That is Welsh for 'water'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does dwr mean?",
        Some(true),
        Some(
            "That is Welsh for 'water'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does uisce mean?",
        Some(true),
        Some(
            "That is Irish for 'water'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does uisge mean?",
        Some(true),
        Some(
            "That is Scottish Gaelic for 'water'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does socorro mean?",
        Some(true),
        Some(
            "That is Spanish for 'help'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does aiuto mean?",
        Some(true),
        Some(
            "That is Italian for 'help'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does hilfe mean?",
        Some(true),
        Some(
            "That is German for 'help'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does cabhair mean?",
        Some(true),
        Some(
            "That is Irish for 'help'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does cuideachadh mean?",
        Some(true),
        Some(
            "That is Scottish Gaelic for 'help'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does buongiorno mean?",
        Some(true),
        Some(
            "That is Italian for 'good morning'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does buonanotte mean?",
        Some(true),
        Some(
            "That is Italian for 'good night'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does salud mean?",
        Some(true),
        Some(
            "That is Spanish for 'cheers'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does santé mean?",
        Some(false),
        Some(
            "That is French for 'cheers'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does sante mean?",
        Some(false),
        Some(
            "That is French for 'cheers'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does salute mean?",
        Some(false),
        Some(
            "That is Italian for 'cheers'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does prost mean?",
        Some(true),
        Some(
            "That is German for 'cheers'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does sláinte mean?",
        Some(true),
        Some(
            "That is Irish for 'cheers'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what does slainte mean?",
        Some(true),
        Some(
            "That is Irish for 'cheers'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what should i do if i was sold a bad car",
        Some(false),
        None,
    ),
    Case(
        "what-does-that-mean",
        "what is a faint line",
        Some(false),
        None,
    ),
    Case(
        "what-does-that-mean",
        "what does non mean in french",
        Some(false),
        None,
    ),
    Case(
        "what-does-that-mean",
        "what does \"die\" mean",
        Some(false),
        None,
    ),
    Case(
        "what-does-that-mean",
        "diolch",
        Some(false),
        Some(
            "That is Welsh for 'thank you'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed.",
        ),
    ),
    Case(
        "what-does-that-mean",
        "what was apartheid",
        Some(false),
        None,
    ),
    Case(
        "past-papers",
        "how to work through exam past papers",
        None,
        Some(
            "Do them under timed conditions, then mark yourself against the real mark scheme - the marking is where most of the learning is. Keep a list of the question types you lose marks on and redo only those next time. If you are short of time, do the paper in your head and write full answers only for the weak sections.",
        ),
    ),
    Case(
        "past-papers",
        "when should i start using past papers",
        None,
        Some(
            "Start about six to eight weeks before the exam, one paper a week, then two a week in the last month, so the early marks still leave you time to fix what they show. Do each under timed conditions and mark it against the real mark scheme, and redo only the question types you lose marks on.",
        ),
    ),
    Case(
        "past-papers",
        "how many weeks early to start past papers",
        None,
        Some(
            "Start about six to eight weeks before the exam, one paper a week, then two a week in the last month, so the early marks still leave you time to fix what they show. Do each under timed conditions and mark it against the real mark scheme, and redo only the question types you lose marks on.",
        ),
    ),
    Case(
        "past-papers",
        "how to mark mock papers",
        None,
        Some(
            "Do them under timed conditions, then mark yourself against the real mark scheme - the marking is where most of the learning is. Keep a list of the question types you lose marks on and redo only those next time. If you are short of time, do the paper in your head and write full answers only for the weak sections.",
        ),
    ),
    Case(
        "past-papers",
        "should i time my past papers",
        None,
        Some(
            "Start about six to eight weeks before the exam, one paper a week, then two a week in the last month, so the early marks still leave you time to fix what they show. Do each under timed conditions and mark it against the real mark scheme, and redo only the question types you lose marks on.",
        ),
    ),
    Case(
        "fractions-sums",
        "divide two fractions",
        None,
        Some(
            "Flip the second fraction upside down and multiply: 2/3 divided by 3/5 is 2/3 x 5/3 = 10/9. Multiplying is top times top and bottom times bottom (2/3 x 3/5 = 6/15, which simplifies to 2/5), and adding or subtracting needs the same bottom number first, so 1/3 + 1/4 = 4/12 + 3/12 = 7/12.",
        ),
    ),
    Case(
        "fractions-sums",
        "how to add fractions",
        None,
        Some(
            "Get the same bottom number first: 1/3 + 1/4 becomes 4/12 + 3/12 = 7/12, and subtracting works the same way. Multiplying is top times top and bottom times bottom (2/3 x 3/5 = 6/15, which simplifies to 2/5); dividing flips the second one over and multiplies, so 2/3 divided by 3/5 is 2/3 x 5/3 = 10/9.",
        ),
    ),
    Case(
        "fractions-sums",
        "take away fractions",
        None,
        Some(
            "Top times top, bottom times bottom: 2/3 x 3/5 = 6/15, which simplifies to 2/5. Dividing is the same trick with the second one flipped over, so 2/3 divided by 3/5 is 2/3 x 5/3 = 10/9. Adding and subtracting need the same bottom number first: 1/3 + 1/4 = 4/12 + 3/12 = 7/12.",
        ),
    ),
    Case(
        "fractions-sums",
        "how do fractions multiply",
        None,
        Some(
            "Top times top, bottom times bottom: 2/3 x 3/5 = 6/15, which simplifies to 2/5. Dividing is the same trick with the second one flipped over, so 2/3 divided by 3/5 is 2/3 x 5/3 = 10/9. Adding and subtracting need the same bottom number first: 1/3 + 1/4 = 4/12 + 3/12 = 7/12.",
        ),
    ),
    Case(
        "fractions-sums",
        "turn fractions into decimals",
        None,
        Some(
            "Top times top, bottom times bottom: 2/3 x 3/5 = 6/15, which simplifies to 2/5. Dividing is the same trick with the second one flipped over, so 2/3 divided by 3/5 is 2/3 x 5/3 = 10/9. Adding and subtracting need the same bottom number first: 1/3 + 1/4 = 4/12 + 3/12 = 7/12.",
        ),
    ),
    Case(
        "fractions-sums",
        "what are fractions",
        None,
        Some(
            "Top times top, bottom times bottom: 2/3 x 3/5 = 6/15, which simplifies to 2/5. Dividing is the same trick with the second one flipped over, so 2/3 divided by 3/5 is 2/3 x 5/3 = 10/9. Adding and subtracting need the same bottom number first: 1/3 + 1/4 = 4/12 + 3/12 = 7/12.",
        ),
    ),
    Case(
        "memorise-faster",
        "how do i memorise things faster",
        None,
        Some(
            "Test yourself instead of rereading: cover the page, say or write what you remember, then check. That one change does more than any technique. Then space it out - the same thing tomorrow, in three days, in a week - because the forgetting and the effort of getting it back is what makes it stay. Say it out loud or write it by hand rather than only reading, link a new fact to one you already know, and sleep on it: a night's sleep is when it gets filed.",
        ),
    ),
    Case(
        "memorise-faster",
        "ways of remembering what i learnt",
        None,
        Some(
            "Test yourself instead of rereading: cover the page, say or write what you remember, then check. That one change does more than any technique. Then space it out - the same thing tomorrow, in three days, in a week - because the forgetting and the effort of getting it back is what makes it stay. Say it out loud or write it by hand rather than only reading, link a new fact to one you already know, and sleep on it: a night's sleep is when it gets filed.",
        ),
    ),
    Case("memorise-faster", "i forgot my password", None, None),
    Case(
        "memorise-faster",
        "i memorized this text",
        None,
        Some(
            "Test yourself instead of rereading: cover the page, say or write what you remember, then check. That one change does more than any technique. Then space it out - the same thing tomorrow, in three days, in a week - because the forgetting and the effort of getting it back is what makes it stay. Say it out loud or write it by hand rather than only reading, link a new fact to one you already know, and sleep on it: a night's sleep is when it gets filed.",
        ),
    ),
    Case(
        "memorise-faster",
        "remembering names more easily",
        None,
        Some(
            "Test yourself instead of rereading: cover the page, say or write what you remember, then check. That one change does more than any technique. Then space it out - the same thing tomorrow, in three days, in a week - because the forgetting and the effort of getting it back is what makes it stay. Say it out loud or write it by hand rather than only reading, link a new fact to one you already know, and sleep on it: a night's sleep is when it gets filed.",
        ),
    ),
    Case(
        "essay-conclusion",
        "how to write an essay conclusion",
        Some(true),
        None,
    ),
    Case(
        "essay-conclusion",
        "the last paragraph of my speech",
        Some(true),
        None,
    ),
    Case(
        "essay-conclusion",
        "how should i end an essay",
        Some(true),
        None,
    ),
    Case("essay-conclusion", "how do i end it", Some(false), None),
    Case(
        "essay-conclusion",
        "my dog ate my homework",
        Some(false),
        None,
    ),
    Case(
        "essay-conclusion",
        "how do i wrap a gift",
        Some(false),
        None,
    ),
];
