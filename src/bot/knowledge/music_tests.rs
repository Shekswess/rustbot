//! Pinned direct responses and guards from PyBot 7d63c7f.
use super::*;

#[test]
fn music_callbacks_and_guards_match_the_reference() {
    let cases = [
        (
            Some(Reply::ListenWhen),
            "what music should i listen to on a long drive",
            Some(
                "For a long drive: *Rumours* by Fleetwood Mac and *Graceland* by Paul Simon are both easy to sing along to, and *The Rest Is History* if you would rather have talking than music. Make one playlist before you go, because changing it at 70mph is the annoying part.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "a podcast about cooking",
            None,
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "what songs for a kids party",
            Some(
                "For a children's party the music wants to be louder and shorter than you think: a 60 to 90 minute playlist of songs they already know - *The Greatest Showman*, *Frozen*, Taylor Swift, whatever the school run plays - from a phone on a speaker away from the food table, started the moment they arrive. Cue musical statues about halfway through: it costs nothing and it resets a room that is getting wild.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "what music should i walk down the aisle to",
            Some(
                "For the aisle pick the entrance, not a favourite song: *Canon in D* (Pachelbel) is the default for good reason, *Here Comes the Bride* (Bridal Chorus) is the one everybody knows, *A Thousand Years* (Christina Perri) or *Sweet Disposition* (Temper Trap) for something modern, or a string version of a song you love if the room is small. Two minutes is plenty, and play it again as you leave.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "what song is good for a funeral",
            Some(
                "Three songs is the usual shape: one going in, one for the reflection or the photos, one to walk out to. *Somewhere Over the Rainbow* (the Israel Kamakawiwo'ole version) or *Ave Maria* suit a church, *Fix You* and *Everybody Hurts* suit a room that wants to cry, *My Way* (Sinatra) or *Thank You for the Days* (the Kinks) walk people out smiling. Give the funeral director titles, artists and exact versions a week ahead.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "what is a good first dance song",
            Some(
                "The song you both already sing badly in the car beats any list, but if you want somewhere to start: *At Last* by Etta James, *You Are the Best Thing* by Ray LaMontagne, *Can't Help Falling in Love* by Elvis, *Tenerife Sea* by Ed Sheeran, *The Book of Love* by Peter Gabriel for something quieter, or *You're My Best Friend* by Queen for a room that will sing along. Play each one all the way through and pick the one you can sway to for three minutes without counting the beats, then give the DJ that exact version - a remix has ruined more first dances than bad dancing.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "songs for signing the register",
            Some(
                "For the register, something the room knows and can hum while they wait: *A Thousand Years* (Christina Perri), *Make You Feel My Love* (Adele), *Jesu, Joy of Man's Desiring* if it is a church, or *Better Together* (Jack Johnson). Two songs is right - one while the first people sign, one for the second - because signing takes longer than anyone plans for.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "what are good songs for a wedding",
            Some(
                "Three moments, three songs: walk in - *Canon in D*, or the *Trumpet Voluntary* if it is grand; sign the register - *A Thousand Years* or *Make You Feel My Love*; walk out - *Signed, Sealed, Delivered* or *This Will Be*. For the evening, the songs everyone already knows (ABBA, Queen, *Mr Brightside*) fill a floor better than anything tasteful. Ask the couple which three songs they would dance to in an empty kitchen.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "what should i play at a reception",
            Some(
                "Split the day, because a wedding is two different rooms: for the ceremony and the meal, something quiet and familiar that nobody has to listen to - a solo piano or string version of songs the couple likes - and for the evening, the songs everyone already knows so the dance floor fills. Ask the couple for five songs they both love and build the evening around those, keep a do-not-play list, and have the first dance track loaded and cued rather than searched for on the night.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "what should i put on at a dinner party",
            Some(
                "For a room with people talking in it, the music is wallpaper, not the show: something familiar and mid-tempo that nobody has to listen to - a Motown or soul compilation, *Rumours*, or any 'dinner party' playlist in the 90 to 110 beats-per-minute range. Start it thirty minutes before people arrive, keep it quiet enough to talk over while you eat, and turn it up once the plates are cleared.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "music for a rainy day",
            Some(
                "For a rainy day: *Rumours* by Fleetwood Mac, *Blue* by Joni Mitchell if you want something quieter, or *Graceland* by Paul Simon for a kitchen that needs waking up. Rain is the one day a whole album beats a playlist, so put one on and leave it alone.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "music while i cook",
            Some(
                "While you cook: *Kind of Blue* by Miles Davis, *The Köln Concert* by Keith Jarrett, or any Motown compilation. All of them have no words to follow, so you can still hear the timer.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "playlist for running",
            Some(
                "For a run: something around 160 to 170 beats per minute keeps the pace steady, so *Mr Brightside*, *Titanium* and *Don't Stop Me Now* all work, or a Daft Punk album run straight through if you would rather not think about it.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "what should i listen to on the commute",
            Some(
                "For a commute: *The Rest Is History* or *Desert Island Discs* if you want talking, *Rumours* if you want music. Podcasts beat music for a commute because the time passes without you counting it.",
            ),
            None,
        ),
        (
            Some(Reply::ListenWhen),
            "recommend an album for later",
            Some(
                "For a long listen: *Rumours* by Fleetwood Mac, *Graceland* by Paul Simon, or *The Rest Is History* if you would rather hear talking. Tell me what the journey is and I'll make it specific.",
            ),
            None,
        ),
        (
            Some(Reply::GenreStart),
            "where do i start with jazz",
            Some(
                "*Kind of Blue* by Miles Davis is the one to start with: quiet, unhurried, and every tune is a simple theme with the solos built on it. Then *Blue Train* by John Coltrane for something with more muscle, and *Ella and Louis* if you would rather hear a voice first. Play one album right through twice before you decide - jazz gives up its tune on the second listen.",
            ),
            None,
        ),
        (
            Some(Reply::GenreStart),
            "where to start with hip hop",
            Some(
                "*Illmatic* by Nas for words and storytelling, *The Miseducation of Lauryn Hill* for singing and soul, and *good kid, m.A.A.d city* by Kendrick Lamar for a whole album that tells one story. Read the words once on paper - rap is the one pop form where the writing rewards a second pass.",
            ),
            None,
        ),
        (
            Some(Reply::GenreStart),
            "what is the difference between house and techno",
            Some(
                "Start with the compilations rather than a single artist: a *Fabric* or *Global Underground* mix, then Jeff Mills' *The Occurrence* for how stripped back it can be. Techno is about the build over eight minutes, so give it time.",
            ),
            None,
        ),
        (
            Some(Reply::GenreStart),
            "recommend a house track",
            Some(
                "*Discovery* by Daft Punk and a *Defected* house compilation cover the friendly end, then Frankie Knuckles' *Best of* for where the sound started. House is warm and repetitive on purpose - let it run and it works.",
            ),
            None,
        ),
        (
            Some(Reply::GenreStart),
            "what is better a house or a flat",
            None,
            None,
        ),
        (
            Some(Reply::GenreStart),
            "where should i start with a walk in the country",
            None,
            None,
        ),
        (
            Some(Reply::GenreStart),
            "is soul or motown different",
            Some(
                "Start with the singles: *The Ultimate Collection* by The Supremes or a Motown greatest-hits record, then *What's Going On* by Marvin Gaye and Stevie Wonder's *Songs in the Key of Life* for what the hit factory turned into.",
            ),
            None,
        ),
        (
            Some(Reply::GenreStart),
            "some music i can start with",
            None,
            None,
        ),
        (
            Some(Reply::InTune),
            "how do i tune a piano",
            Some(
                "An upright or grand is a job for a tuner, not for you: an annual visit, and twice a year if it is in a room that gets hot in summer. A digital piano needs nothing. If one note has gone badly wrong on an acoustic, stop playing it and get the tuner in, because it is usually a string or a hammer rather than the tuning.",
            ),
            None,
        ),
        (
            Some(Reply::InTune),
            "how to tune a keyboard",
            Some(
                "An upright or grand is a job for a tuner, not for you: an annual visit, and twice a year if it is in a room that gets hot in summer. A digital piano needs nothing. If one note has gone badly wrong on an acoustic, stop playing it and get the tuner in, because it is usually a string or a hammer rather than the tuning.",
            ),
            None,
        ),
        (
            Some(Reply::InTune),
            "my guitar wont stay in tune",
            Some(
                "Clip a tuner on the headstock and take the string order low to high: E, A, D, G, B, E - the sixth string is the thick one, and the top E is the thin one. Turn the peg slowly and always come up to the note from below, because slackening down to it leaves the string ready to slip. Ten seconds a string, and tune every time you pick it up; guitars move with the room.",
            ),
            None,
        ),
        (
            Some(Reply::InTune),
            "how do i tune a bass",
            Some(
                "A clip-on tuner and the string order low to high: E, A, D, G. Turn the peg slowly and come up to each note from below rather than down onto it. A bass needs tuning less often than a guitar but goes out if the room changes temperature.",
            ),
            None,
        ),
        (
            Some(Reply::InTune),
            "my cello wont stay in tune",
            Some(
                "A clip-on tuner and the order G, D, A, E from the lowest string up - and the pegs are for the big jumps while the fine tuners at the tailpiece do the last bit. Turn the peg with a little inward pressure so it grips, or it will slip as soon as you let go.",
            ),
            None,
        ),
        (
            Some(Reply::InTune),
            "how to tune a banjo",
            Some(
                "Clip a tuner on the headstock and take the string order low to high: E, A, D, G, B, E - the sixth string is the thick one, and the top E is the thin one. Turn the peg slowly and always come up to the note from below, because slackening down to it leaves the string ready to slip. Ten seconds a string, and tune every time you pick it up; guitars move with the room.",
            ),
            None,
        ),
        (Some(Reply::InTune), "how do i tune a toy", None, None),
        (
            Some(Reply::Term),
            "what is a tempo",
            Some(
                "Tempo is how fast the music goes, counted in beats per minute: 60 is one a second and a ballad, 120 is two a second and a dance track. A metronome clicks it for you.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Term),
            "what is a time signature",
            Some(
                "The two numbers at the start of the music. The top one is how many beats are in a bar and the bottom one which note gets the beat, so 4/4 is four quarter notes to the bar (most pop) and 3/4 is a waltz.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Term),
            "whats a beat",
            Some(
                "The beat is the steady pulse you clap or tap along to; the rhythm is the pattern played on top of it. Four beats to the bar is most pop, three is a waltz, and a drummer keeps the beat while everyone else plays around it.",
            ),
            Some(false),
        ),
        (
            Some(Reply::Term),
            "what is a beat in music",
            Some(
                "The beat is the steady pulse you clap or tap along to; the rhythm is the pattern played on top of it. Four beats to the bar is most pop, three is a waltz, and a drummer keeps the beat while everyone else plays around it.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Term),
            "wheres my keys",
            Some(
                "A chord is three or more notes together, the key is the home note a song keeps returning to, and an octave is the same note higher or lower. The time signature says how many beats are in a bar. Ask about one and I'll give you its answer.",
            ),
            Some(false),
        ),
        (
            Some(Reply::Term),
            "wheres my keys and what key is this song in",
            Some(
                "The key is the home note a piece is built around and comes back to: a song in C keeps landing on C. The notes it uses are that key's scale, and a key change lifts the whole song.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Term),
            "what is your key",
            Some(
                "The key is the home note a piece is built around and comes back to: a song in C keeps landing on C. The notes it uses are that key's scale, and a key change lifts the whole song.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Term),
            "i lost your key",
            Some(
                "The key is the home note a piece is built around and comes back to: a song in C keeps landing on C. The notes it uses are that key's scale, and a key change lifts the whole song.",
            ),
            Some(false),
        ),
        (
            Some(Reply::Term),
            "what does a music producer do",
            Some(
                "The producer is the person who shapes the recording: the arrangement, the sounds, how the parts fit, and the mix at the end. They are not the songwriter and not the engineer. In film the word means the person who runs the money and the schedule, a different job.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Term),
            "what does a film producer do",
            Some(
                "A chord is three or more notes together, the key is the home note a song keeps returning to, and an octave is the same note higher or lower. The time signature says how many beats are in a bar. Ask about one and I'll give you its answer.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Term),
            "what is music theory",
            Some(
                "A chord is three or more notes together, the key is the home note a song keeps returning to, and an octave is the same note higher or lower. The time signature says how many beats are in a bar. Ask about one and I'll give you its answer.",
            ),
            Some(true),
        ),
        (
            Some(Reply::Term),
            "what are scales",
            Some(
                "A chord is three or more notes together, the key is the home note a song keeps returning to, and an octave is the same note higher or lower. The time signature says how many beats are in a bar. Ask about one and I'll give you its answer.",
            ),
            Some(true),
        ),
        (None, "what is a bridge in a song?", None, Some(false)),
        (None, "what is a bridge", None, Some(false)),
        (
            None,
            "what is the musical bridge in this track",
            None,
            Some(true),
        ),
        (None, "bridge of the song", None, Some(true)),
        (None, "dental bridge advice", None, Some(false)),
        (
            Some(Reply::Count),
            "how many strings does a bass guitar have",
            Some(
                "A guitar has six strings, E, A, D, G, B and E from the thickest up. A 12-string guitar has twelve, in six pairs.",
            ),
            None,
        ),
        (
            Some(Reply::Count),
            "how many strings on a violin",
            Some("A violin has four strings, tuned G, D, A and E from the lowest up."),
            None,
        ),
        (
            Some(Reply::Count),
            "how many strings on a double bass",
            Some("A double bass has four strings, E, A, D and G, and some have a fifth."),
            None,
        ),
        (
            Some(Reply::Count),
            "how many keys on a keyboard",
            Some(
                "A full-size piano keyboard has 88 keys; a smaller synth or starter keyboard usually has 61 or 49.",
            ),
            None,
        ),
        (
            Some(Reply::Count),
            "how many buttons",
            Some(
                "A guitar has six strings, a violin four, a bass four and a piano 88 keys. Ask me about the one you have and I'll tell you the count and the tuning.",
            ),
            None,
        ),
        (
            Some(Reply::Practice),
            "how do i practise piano",
            Some(
                "Fifteen to twenty minutes a day beats two hours on Sunday, and start with the hardest four bars rather than the beginning: play them slowly enough to get them right, four times in a row, then put them back in the piece. Use a metronome at a speed you can play cleanly and nudge it up two beats a minute, because sloppy and fast teaches sloppy. Scales and one technical exercise first warm the hands up, and finish on something you enjoy so you want to come back tomorrow.",
            ),
            None,
        ),
        (
            Some(Reply::Practice),
            "how do i practice scales",
            Some(
                "Fifteen to twenty minutes a day beats two hours on Sunday, and start with the hardest four bars rather than the beginning: play them slowly enough to get them right, four times in a row, then put them back in the piece. Use a metronome at a speed you can play cleanly and nudge it up two beats a minute, because sloppy and fast teaches sloppy. Scales and one technical exercise first warm the hands up, and finish on something you enjoy so you want to come back tomorrow.",
            ),
            None,
        ),
        (
            Some(Reply::Practice),
            "how do i learn to read music",
            Some(
                "Start with the rhythm, not the notes: clap a line of the music while a metronome clicks, because the note names are the easy half. Then learn the five lines of the treble staff with a mnemonic you make up, ten minutes a day, and read one new short piece slowly each day rather than the same one until it is perfect - reading is the skill of playing what you have not seen before. Fifteen to twenty minutes a day beats two hours on Sunday, and start with the hardest four bars rather than the beginning: play them slowly enough to get them right, four times in a row, then put them back in the piece. Use a metronome at a speed you can play cleanly and nudge it up two beats a minute, because sloppy and fast teaches sloppy. Scales and one technical exercise first warm the hands up, and finish on something you enjoy so you want to come back tomorrow.",
            ),
            None,
        ),
        (
            Some(Reply::Practice),
            "how can i get better at sight reading",
            Some(
                "Start with the rhythm, not the notes: clap a line of the music while a metronome clicks, because the note names are the easy half. Then learn the five lines of the treble staff with a mnemonic you make up, ten minutes a day, and read one new short piece slowly each day rather than the same one until it is perfect - reading is the skill of playing what you have not seen before. Fifteen to twenty minutes a day beats two hours on Sunday, and start with the hardest four bars rather than the beginning: play them slowly enough to get them right, four times in a row, then put them back in the piece. Use a metronome at a speed you can play cleanly and nudge it up two beats a minute, because sloppy and fast teaches sloppy. Scales and one technical exercise first warm the hands up, and finish on something you enjoy so you want to come back tomorrow.",
            ),
            None,
        ),
        (
            Some(Reply::Practice),
            "sheet music practice",
            Some(
                "Start with the rhythm, not the notes: clap a line of the music while a metronome clicks, because the note names are the easy half. Then learn the five lines of the treble staff with a mnemonic you make up, ten minutes a day, and read one new short piece slowly each day rather than the same one until it is perfect - reading is the skill of playing what you have not seen before. Fifteen to twenty minutes a day beats two hours on Sunday, and start with the hardest four bars rather than the beginning: play them slowly enough to get them right, four times in a row, then put them back in the piece. Use a metronome at a speed you can play cleanly and nudge it up two beats a minute, because sloppy and fast teaches sloppy. Scales and one technical exercise first warm the hands up, and finish on something you enjoy so you want to come back tomorrow.",
            ),
            None,
        ),
    ];
    let mut differences = Vec::new();
    for (kind, line, expected, guard) in cases {
        let reading = Reading::new(line);
        let result = kind.and_then(|kind| reply(kind, &reading));
        if result.as_deref() != expected {
            differences.push(format!(
                "{kind:?} {line}: {result:?}, reference {expected:?}"
            ));
        }
        if let Some(expected) = guard {
            let actual = if kind.is_some() {
                term_context(&reading)
            } else {
                bridge_context(&reading)
            };
            if actual != expected {
                differences.push(format!(
                    "{kind:?} {line}: guard {actual}, reference {expected}"
                ));
            }
        }
    }
    assert!(
        differences.is_empty(),
        "mismatches\n{}",
        differences.join("\n")
    );
}
