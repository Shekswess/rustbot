//! Direct subject/branch oracle from pinned PyBot.

pub(super) const CASES: &[(&str, &str, bool, Option<&str>)] = &[
    (
        "mass-and-weight",
        "what is weight",
        true,
        Some(
            "Weight is the pull of gravity on a mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit. Mass is how much stuff is in something, measured in kilograms, and it is 60 kg in all three places. The scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "what is mass",
        true,
        Some(
            "Mass is how much stuff is in something, measured in kilograms, and it does not change wherever you go. Weight is the pull of gravity on that mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit, while their mass is 60 kg in all three places. So the scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "difference between weight and mass",
        true,
        Some(
            "Mass is how much stuff is in something, measured in kilograms, and it does not change wherever you go. Weight is the pull of gravity on that mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit, while their mass is 60 kg in all three places. So the scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "mass and weight on the moon",
        true,
        Some(
            "Mass is how much stuff is in something, measured in kilograms, and it does not change wherever you go. Weight is the pull of gravity on that mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit, while their mass is 60 kg in all three places. So the scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "what weighs less in space",
        true,
        Some(
            "Mass is how much stuff is in something, measured in kilograms, and it does not change wherever you go. Weight is the pull of gravity on that mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit, while their mass is 60 kg in all three places. So the scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "what is a kilogram",
        true,
        Some(
            "Mass is how much stuff is in something, measured in kilograms, and it does not change wherever you go. Weight is the pull of gravity on that mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit, while their mass is 60 kg in all three places. So the scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "about a stone",
        false,
        Some(
            "Mass is how much stuff is in something, measured in kilograms, and it does not change wherever you go. Weight is the pull of gravity on that mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit, while their mass is 60 kg in all three places. So the scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "i want to lose a stone",
        false,
        Some(
            "Mass is how much stuff is in something, measured in kilograms, and it does not change wherever you go. Weight is the pull of gravity on that mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit, while their mass is 60 kg in all three places. So the scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "how many pounds in a stone",
        false,
        Some(
            "Mass is how much stuff is in something, measured in kilograms, and it does not change wherever you go. Weight is the pull of gravity on that mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit, while their mass is 60 kg in all three places. So the scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "the mass in church",
        true,
        Some(
            "Mass is how much stuff is in something, measured in kilograms, and it does not change wherever you go. Weight is the pull of gravity on that mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit, while their mass is 60 kg in all three places. So the scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "mass-and-weight",
        "weightless in orbit",
        true,
        Some(
            "Weight is the pull of gravity on a mass, measured in newtons: a 60 kg person weighs about 600 N on Earth, about 100 N on the Moon and nothing at all in orbit. Mass is how much stuff is in something, measured in kilograms, and it is 60 kg in all three places. The scales in your bathroom measure weight and are marked in kilograms for the mass that goes with it here.",
        ),
    ),
    (
        "kinetic-and-potential-energy",
        "what is potential energy",
        true,
        Some(
            "Potential energy is energy stored ready to be used: a ball held up high has potential energy, and it turns into kinetic energy as it falls, faster and faster. Kinetic energy is the energy something has because it is moving. Both are measured in joules. Potential is mass times gravity times height, so twice the height is twice the energy; kinetic is half the mass times the speed squared, so doubling the speed gives four times the energy.",
        ),
    ),
    (
        "kinetic-and-potential-energy",
        "what is kinetic energy",
        true,
        Some(
            "Kinetic energy is the energy something has because it is moving, and potential energy is energy stored ready to move: a ball held up high has potential energy, and it turns into kinetic energy as it falls, faster and faster. Both are measured in joules. Kinetic is half the mass times the speed squared, so doubling the speed gives four times the energy; potential is mass times gravity times height, so twice the height is twice the energy.",
        ),
    ),
    (
        "kinetic-and-potential-energy",
        "potential energy compared to kinetic energy",
        true,
        Some(
            "Potential energy is energy stored ready to be used: a ball held up high has potential energy, and it turns into kinetic energy as it falls, faster and faster. Kinetic energy is the energy something has because it is moving. Both are measured in joules. Potential is mass times gravity times height, so twice the height is twice the energy; kinetic is half the mass times the speed squared, so doubling the speed gives four times the energy.",
        ),
    ),
    (
        "kinetic-and-potential-energy",
        "kinetic energy compared to potential energy",
        true,
        Some(
            "Kinetic energy is the energy something has because it is moving, and potential energy is energy stored ready to move: a ball held up high has potential energy, and it turns into kinetic energy as it falls, faster and faster. Both are measured in joules. Kinetic is half the mass times the speed squared, so doubling the speed gives four times the energy; potential is mass times gravity times height, so twice the height is twice the energy.",
        ),
    ),
    (
        "kinetic-and-potential-energy",
        "why is a falling ball potential energy",
        true,
        Some(
            "Potential energy is energy stored ready to be used: a ball held up high has potential energy, and it turns into kinetic energy as it falls, faster and faster. Kinetic energy is the energy something has because it is moving. Both are measured in joules. Potential is mass times gravity times height, so twice the height is twice the energy; kinetic is half the mass times the speed squared, so doubling the speed gives four times the energy.",
        ),
    ),
    (
        "kinetic-and-potential-energy",
        "how much stored energy",
        true,
        Some(
            "Kinetic energy is the energy something has because it is moving, and potential energy is energy stored ready to move: a ball held up high has potential energy, and it turns into kinetic energy as it falls, faster and faster. Both are measured in joules. Kinetic is half the mass times the speed squared, so doubling the speed gives four times the energy; potential is mass times gravity times height, so twice the height is twice the energy.",
        ),
    ),
    (
        "kinetic-and-potential-energy",
        "explain potentiality and kinetic energy",
        true,
        Some(
            "Potential energy is energy stored ready to be used: a ball held up high has potential energy, and it turns into kinetic energy as it falls, faster and faster. Kinetic energy is the energy something has because it is moving. Both are measured in joules. Potential is mass times gravity times height, so twice the height is twice the energy; kinetic is half the mass times the speed squared, so doubling the speed gives four times the energy.",
        ),
    ),
    (
        "kinetic-and-potential-energy",
        "why does the potential energy turn into kinetic",
        true,
        Some(
            "Potential energy is energy stored ready to be used: a ball held up high has potential energy, and it turns into kinetic energy as it falls, faster and faster. Kinetic energy is the energy something has because it is moving. Both are measured in joules. Potential is mass times gravity times height, so twice the height is twice the energy; kinetic is half the mass times the speed squared, so doubling the speed gives four times the energy.",
        ),
    ),
    (
        "mitosis-and-meiosis",
        "what is meiosis",
        true,
        Some(
            "Meiosis makes four cells with half the chromosomes each, and that is how eggs and sperm are made, so that when two of them join the new cell has a full set again. Mitosis makes two identical cells, each with the same chromosomes as the parent, and that is how you grow and how a cut heals. Meiosis has two divisions in a row, mitosis one; the mnemonic that works is mitosis for maintenance, meiosis for making babies.",
        ),
    ),
    (
        "mitosis-and-meiosis",
        "what is mitosis",
        true,
        Some(
            "Mitosis makes two identical cells, each with the same chromosomes as the parent, and that is how you grow and how a cut heals. Meiosis makes four cells with half the chromosomes, and that is how eggs and sperm are made, so that when two of them join the new cell has a full set again. The mnemonic that works: mitosis for maintenance, meiosis for making babies. Mitosis has one division, meiosis has two in a row.",
        ),
    ),
    (
        "mitosis-and-meiosis",
        "what is a cell cycle",
        true,
        Some(
            "The cell cycle is the whole life of a cell from one division to the next: a growth stage where the cell gets bigger and copies its DNA, then the division itself, which is mitosis if it makes two identical cells. Cells that stop dividing leave the cycle and sit in a resting state instead. Meiosis is the different division that makes eggs and sperm, with half the chromosomes each.",
        ),
    ),
    (
        "mitosis-and-meiosis",
        "cell cycle and meiosis differences",
        true,
        Some(
            "The cell cycle is the whole life of a cell from one division to the next: a growth stage where the cell gets bigger and copies its DNA, then the division itself, which is mitosis if it makes two identical cells. Cells that stop dividing leave the cycle and sit in a resting state instead. Meiosis is the different division that makes eggs and sperm, with half the chromosomes each.",
        ),
    ),
    (
        "mitosis-and-meiosis",
        "compare meiosis and mitosis",
        true,
        Some(
            "Meiosis makes four cells with half the chromosomes each, and that is how eggs and sperm are made, so that when two of them join the new cell has a full set again. Mitosis makes two identical cells, each with the same chromosomes as the parent, and that is how you grow and how a cut heals. Meiosis has two divisions in a row, mitosis one; the mnemonic that works is mitosis for maintenance, meiosis for making babies.",
        ),
    ),
    (
        "mitosis-and-meiosis",
        "compare mitosis with meiosis",
        true,
        Some(
            "Mitosis makes two identical cells, each with the same chromosomes as the parent, and that is how you grow and how a cut heals. Meiosis makes four cells with half the chromosomes, and that is how eggs and sperm are made, so that when two of them join the new cell has a full set again. The mnemonic that works: mitosis for maintenance, meiosis for making babies. Mitosis has one division, meiosis has two in a row.",
        ),
    ),
    (
        "mitosis-and-meiosis",
        "how many chromosomes are in meiosis",
        true,
        Some(
            "Meiosis makes four cells with half the chromosomes each, and that is how eggs and sperm are made, so that when two of them join the new cell has a full set again. Mitosis makes two identical cells, each with the same chromosomes as the parent, and that is how you grow and how a cut heals. Meiosis has two divisions in a row, mitosis one; the mnemonic that works is mitosis for maintenance, meiosis for making babies.",
        ),
    ),
    (
        "mitosis-and-meiosis",
        "whats cell division",
        true,
        Some(
            "Mitosis makes two identical cells, each with the same chromosomes as the parent, and that is how you grow and how a cut heals. Meiosis makes four cells with half the chromosomes, and that is how eggs and sperm are made, so that when two of them join the new cell has a full set again. The mnemonic that works: mitosis for maintenance, meiosis for making babies. Mitosis has one division, meiosis has two in a row.",
        ),
    ),
    (
        "frequency",
        "what is wavelength",
        true,
        Some(
            "Wavelength is the length of one whole wave, from one peak to the next, measured in metres. Frequency is how many of those waves pass a point each second, in hertz, and the two are locked together by the speed of the wave: wavelength is the speed divided by the frequency, so a 340 m/s sound at 50 Hz has a wavelength of 6.8 m. That is why a low note travels further round a corner than a high one.",
        ),
    ),
    (
        "frequency",
        "what is amplitude",
        true,
        Some(
            "Amplitude is how big the wave is - how far it swings from the middle - and it is what you hear as volume or see as brightness: a bigger amplitude is a louder sound or a brighter light. Frequency is the different thing, how many waves pass each second, measured in hertz, and that is the pitch. So a loud low note is big amplitude and small frequency, and turning the volume up changes the amplitude only.",
        ),
    ),
    (
        "frequency",
        "what is frequency",
        true,
        Some(
            "Frequency is how many times something happens in a second, measured in hertz: 50 Hz is fifty times a second. A sound's frequency is its pitch - 440 Hz is the A above middle C, and the higher the number the higher the note - while a radio station's 100 MHz is a hundred million waves a second. It is the reciprocal of the period, so a wave that repeats every 0.02 seconds has a frequency of 50 Hz, and the wavelength is the speed divided by the frequency.",
        ),
    ),
    (
        "frequency",
        "how do you calculate wavelength and frequency",
        true,
        Some(
            "Wavelength is the length of one whole wave, from one peak to the next, measured in metres. Frequency is how many of those waves pass a point each second, in hertz, and the two are locked together by the speed of the wave: wavelength is the speed divided by the frequency, so a 340 m/s sound at 50 Hz has a wavelength of 6.8 m. That is why a low note travels further round a corner than a high one.",
        ),
    ),
    (
        "frequency",
        "explain frequency and amplitude",
        true,
        Some(
            "Frequency is how many times something happens in a second, measured in hertz: 50 Hz is fifty times a second. A sound's frequency is its pitch - 440 Hz is the A above middle C, and the higher the number the higher the note - while a radio station's 100 MHz is a hundred million waves a second. It is the reciprocal of the period, so a wave that repeats every 0.02 seconds has a frequency of 50 Hz, and the wavelength is the speed divided by the frequency.",
        ),
    ),
    (
        "frequency",
        "what is the amplitude of a wave",
        true,
        Some(
            "Amplitude is how big the wave is - how far it swings from the middle - and it is what you hear as volume or see as brightness: a bigger amplitude is a louder sound or a brighter light. Frequency is the different thing, how many waves pass each second, measured in hertz, and that is the pitch. So a loud low note is big amplitude and small frequency, and turning the volume up changes the amplitude only.",
        ),
    ),
    (
        "frequency",
        "what is hertz",
        true,
        Some(
            "Frequency is how many times something happens in a second, measured in hertz: 50 Hz is fifty times a second. A sound's frequency is its pitch - 440 Hz is the A above middle C, and the higher the number the higher the note - while a radio station's 100 MHz is a hundred million waves a second. It is the reciprocal of the period, so a wave that repeats every 0.02 seconds has a frequency of 50 Hz, and the wavelength is the speed divided by the frequency.",
        ),
    ),
    (
        "frequency",
        "what is the length of a wave",
        true,
        Some(
            "Frequency is how many times something happens in a second, measured in hertz: 50 Hz is fifty times a second. A sound's frequency is its pitch - 440 Hz is the A above middle C, and the higher the number the higher the note - while a radio station's 100 MHz is a hundred million waves a second. It is the reciprocal of the period, so a wave that repeats every 0.02 seconds has a frequency of 50 Hz, and the wavelength is the speed divided by the frequency.",
        ),
    ),
    (
        "newtons-laws",
        "what is newtons first law",
        true,
        Some(
            "The first law says a thing keeps still, or keeps going in a straight line at the same speed, unless a force acts on it - which is why a passenger is thrown forward when a bus brakes, and why a seatbelt is the force that stops them. The second law is the useful one: force equals mass times acceleration, F = ma, so the same push moves a light thing quickly and a heavy one slowly, and a 1,000 kg car pushed with 2,000 N accelerates at 2 m/s each second. The third law says every force has an equal force back the other way: you push the ground and it pushes you forward, a rocket throws gas down and the gas throws the rocket up.",
        ),
    ),
    (
        "newtons-laws",
        "explain the second law of motion",
        true,
        Some(
            "The second law is the useful one: force equals mass times acceleration, F = ma, so the same push moves a light thing quickly and a heavy one slowly, and a 1,000 kg car pushed with 2,000 N accelerates at 2 m/s each second. The first law says a thing keeps still, or keeps going in a straight line at the same speed, unless a force acts on it - which is why a passenger is thrown forward when a bus brakes, and why a seatbelt is the force that stops them. The third law says every force has an equal force back the other way: you push the ground and it pushes you forward, a rocket throws gas down and the gas throws the rocket up.",
        ),
    ),
    (
        "newtons-laws",
        "what is newton’s third law",
        true,
        Some(
            "The third law says every force has an equal force back the other way: you push the ground and it pushes you forward, a rocket throws gas down and the gas throws the rocket up. The second law is the useful one: force equals mass times acceleration, F = ma, so the same push moves a light thing quickly and a heavy one slowly, and a 1,000 kg car pushed with 2,000 N accelerates at 2 m/s each second. The first law says a thing keeps still, or keeps going in a straight line at the same speed, unless a force acts on it - which is why a passenger is thrown forward when a bus brakes, and why a seatbelt is the force that stops them.",
        ),
    ),
    (
        "newtons-laws",
        "explain the law number 1st",
        true,
        Some(
            "The first law says a thing keeps still, or keeps going in a straight line at the same speed, unless a force acts on it - which is why a passenger is thrown forward when a bus brakes, and why a seatbelt is the force that stops them. The second law is the useful one: force equals mass times acceleration, F = ma, so the same push moves a light thing quickly and a heavy one slowly, and a 1,000 kg car pushed with 2,000 N accelerates at 2 m/s each second. The third law says every force has an equal force back the other way: you push the ground and it pushes you forward, a rocket throws gas down and the gas throws the rocket up.",
        ),
    ),
    (
        "newtons-laws",
        "first and third laws of motion",
        true,
        Some(
            "The first law says a thing keeps still, or keeps going in a straight line at the same speed, unless a force acts on it - which is why a passenger is thrown forward when a bus brakes, and why a seatbelt is the force that stops them. The second law is the useful one: force equals mass times acceleration, F = ma, so the same push moves a light thing quickly and a heavy one slowly, and a 1,000 kg car pushed with 2,000 N accelerates at 2 m/s each second. The third law says every force has an equal force back the other way: you push the ground and it pushes you forward, a rocket throws gas down and the gas throws the rocket up.",
        ),
    ),
    (
        "newtons-laws",
        "law of motion number third",
        true,
        Some(
            "The third law says every force has an equal force back the other way: you push the ground and it pushes you forward, a rocket throws gas down and the gas throws the rocket up. The second law is the useful one: force equals mass times acceleration, F = ma, so the same push moves a light thing quickly and a heavy one slowly, and a 1,000 kg car pushed with 2,000 N accelerates at 2 m/s each second. The first law says a thing keeps still, or keeps going in a straight line at the same speed, unless a force acts on it - which is why a passenger is thrown forward when a bus brakes, and why a seatbelt is the force that stops them.",
        ),
    ),
    (
        "newtons-laws",
        "how does f ma work",
        true,
        Some(
            "The second law is the useful one: force equals mass times acceleration, F = ma, so the same push moves a light thing quickly and a heavy one slowly, and a 1,000 kg car pushed with 2,000 N accelerates at 2 m/s each second. The first law says a thing keeps still, or keeps going in a straight line at the same speed, unless a force acts on it - which is why a passenger is thrown forward when a bus brakes, and why a seatbelt is the force that stops them. The third law says every force has an equal force back the other way: you push the ground and it pushes you forward, a rocket throws gas down and the gas throws the rocket up.",
        ),
    ),
    (
        "newtons-laws",
        "newtons laws",
        true,
        Some(
            "The second law is the useful one: force equals mass times acceleration, F = ma, so the same push moves a light thing quickly and a heavy one slowly, and a 1,000 kg car pushed with 2,000 N accelerates at 2 m/s each second. The first law says a thing keeps still, or keeps going in a straight line at the same speed, unless a force acts on it - which is why a passenger is thrown forward when a bus brakes, and why a seatbelt is the force that stops them. The third law says every force has an equal force back the other way: you push the ground and it pushes you forward, a rocket throws gas down and the gas throws the rocket up.",
        ),
    ),
    (
        "newtons-laws",
        "why is the third law useful",
        true,
        Some(
            "The third law says every force has an equal force back the other way: you push the ground and it pushes you forward, a rocket throws gas down and the gas throws the rocket up. The second law is the useful one: force equals mass times acceleration, F = ma, so the same push moves a light thing quickly and a heavy one slowly, and a 1,000 kg car pushed with 2,000 N accelerates at 2 m/s each second. The first law says a thing keeps still, or keeps going in a straight line at the same speed, unless a force acts on it - which is why a passenger is thrown forward when a bus brakes, and why a seatbelt is the force that stops them.",
        ),
    ),
    (
        "speed-and-velocity",
        "what is velocity",
        true,
        Some(
            "Velocity is how fast you are going and which way: 50 mph north is a velocity, and 50 mph on its own is a speed. So a car going round a roundabout at a steady 30 mph has a constant speed and a changing velocity, because the direction keeps changing. Speed is distance over time, and average speed is the whole trip's distance over the whole time, which is why a two-hour journey with a stop does not average the speed you had on the motorway.",
        ),
    ),
    (
        "speed-and-velocity",
        "what is speed",
        true,
        Some(
            "Speed is how fast you are going, distance over time: 100 miles in two hours is 50 mph. Velocity is that speed with a direction, so a car going round a roundabout at a steady 30 mph has a constant speed and a changing velocity. Average speed is the whole trip's distance over the whole time, which is why a two-hour journey with a stop does not average the speed you had on the motorway.",
        ),
    ),
    (
        "speed-and-velocity",
        "what is the difference between speed and velocity",
        true,
        Some(
            "Speed is how fast you are going, distance over time: 100 miles in two hours is 50 mph. Velocity is that speed with a direction, so a car going round a roundabout at a steady 30 mph has a constant speed and a changing velocity. Average speed is the whole trip's distance over the whole time, which is why a two-hour journey with a stop does not average the speed you had on the motorway.",
        ),
    ),
    (
        "speed-and-velocity",
        "velocity and speed",
        true,
        Some(
            "Speed is how fast you are going, distance over time: 100 miles in two hours is 50 mph. Velocity is that speed with a direction, so a car going round a roundabout at a steady 30 mph has a constant speed and a changing velocity. Average speed is the whole trip's distance over the whole time, which is why a two-hour journey with a stop does not average the speed you had on the motorway.",
        ),
    ),
    (
        "speed-and-velocity",
        "how do you calculate average speed",
        true,
        Some(
            "Speed is how fast you are going, distance over time: 100 miles in two hours is 50 mph. Velocity is that speed with a direction, so a car going round a roundabout at a steady 30 mph has a constant speed and a changing velocity. Average speed is the whole trip's distance over the whole time, which is why a two-hour journey with a stop does not average the speed you had on the motorway.",
        ),
    ),
    (
        "speed-and-velocity",
        "which way does velocity point",
        true,
        Some(
            "Velocity is how fast you are going and which way: 50 mph north is a velocity, and 50 mph on its own is a speed. So a car going round a roundabout at a steady 30 mph has a constant speed and a changing velocity, because the direction keeps changing. Speed is distance over time, and average speed is the whole trip's distance over the whole time, which is why a two-hour journey with a stop does not average the speed you had on the motorway.",
        ),
    ),
    (
        "speed-and-velocity",
        "why is the velocity changing",
        true,
        Some(
            "Velocity is how fast you are going and which way: 50 mph north is a velocity, and 50 mph on its own is a speed. So a car going round a roundabout at a steady 30 mph has a constant speed and a changing velocity, because the direction keeps changing. Speed is distance over time, and average speed is the whole trip's distance over the whole time, which is why a two-hour journey with a stop does not average the speed you had on the motorway.",
        ),
    ),
    (
        "water-freezing",
        "at what temperature does water freeze",
        true,
        Some(
            "Water freezes at 0C (32F). Salt lowers that, which is why roads are gritted, and pressure lowers it too, which is why ice skates glide on a thin film of water.",
        ),
    ),
    (
        "water-freezing",
        "why is ice slippery",
        true,
        Some(
            "Ice is slippery because a skate or a boot melts a thin film of water under it, so you are sliding on water rather than on ice, and the film keeps refreezing as you go over it. Water freezes at 0C (32F); salt lowers that, which is why roads are gritted.",
        ),
    ),
    (
        "water-freezing",
        "why do my skates glide",
        true,
        Some(
            "Ice is slippery because a skate or a boot melts a thin film of water under it, so you are sliding on water rather than on ice, and the film keeps refreezing as you go over it. Water freezes at 0C (32F); salt lowers that, which is why roads are gritted.",
        ),
    ),
    (
        "water-freezing",
        "why is the pond frozen",
        true,
        Some(
            "Water freezes at 0C (32F). Salt lowers that, which is why roads are gritted, and pressure lowers it too, which is why ice skates glide on a thin film of water.",
        ),
    ),
    (
        "water-freezing",
        "why does ice make you slip",
        true,
        Some(
            "Water freezes at 0C (32F). Salt lowers that, which is why roads are gritted, and pressure lowers it too, which is why ice skates glide on a thin film of water.",
        ),
    ),
    (
        "water-freezing",
        "what does salt do to ice",
        true,
        Some(
            "Water freezes at 0C (32F). Salt lowers that, which is why roads are gritted, and pressure lowers it too, which is why ice skates glide on a thin film of water.",
        ),
    ),
    (
        "water-freezing",
        "what happens when ice slips",
        true,
        Some(
            "Water freezes at 0C (32F). Salt lowers that, which is why roads are gritted, and pressure lowers it too, which is why ice skates glide on a thin film of water.",
        ),
    ),
    (
        "water-freezing",
        "water is frozen",
        true,
        Some(
            "Water freezes at 0C (32F). Salt lowers that, which is why roads are gritted, and pressure lowers it too, which is why ice skates glide on a thin film of water.",
        ),
    ),
    (
        "water-freezing",
        "skating on a frozen pond",
        true,
        Some(
            "Ice is slippery because a skate or a boot melts a thin film of water under it, so you are sliding on water rather than on ice, and the film keeps refreezing as you go over it. Water freezes at 0C (32F); salt lowers that, which is why roads are gritted.",
        ),
    ),
    (
        "tall-things-stand-up",
        "why do tall buildings not topple over",
        true,
        Some(
            "A tall building stays up because its weight runs straight down a braced frame into deep foundations, so wind bends it a little instead of tipping it, and the tallest carry a heavy damper near the top to soak up the sway. Pisa leans without falling because its weight still sits over its base: it sank slowly into soft ground, and soil taken out from the high side is what steadied it.",
        ),
    ),
    (
        "tall-things-stand-up",
        "how do cranes stay up",
        true,
        Some(
            "A tower crane stays up because its mast is bolted to a concrete base and tied into the building as that rises, and a counterweight on the short arm balances the load on the long one. It is built to lift a set weight at a set reach and no more, so it trips out before it can tip.",
        ),
    ),
    (
        "tall-things-stand-up",
        "how can a bridge stand up",
        true,
        Some(
            "A bridge gets its load to the ground in one of three ways: a beam rests the span on a support at each end, an arch pushes the weight outwards into solid abutments, and a suspension or cable-stayed bridge hangs the deck from cables over towers anchored into the ground. Whichever it is, the span is never left to carry itself.",
        ),
    ),
    (
        "tall-things-stand-up",
        "why are chimneys so tall and do not fall",
        true,
        Some(
            "A chimney stays up because each stack of bricks is bonded into the wall it stands on and strapped to the roof, and every joint is staggered so the bricks cannot slide on each other - a straight stack of bricks with a flue down the middle is strong only while the mortar and the flaunching at the top are sound. That is why a chimney comes down in a storm, and why it comes down as one piece: the mortar in the joints has gone soft, or smoke and rain have washed it out from under the flaunching, so the whole stack rocks on its base.",
        ),
    ),
    (
        "tall-things-stand-up",
        "why do trees stay upright",
        true,
        Some(
            "A tree stays up because its roots spread about as wide as its branches and the trunk tapers as it rises, so most of the weight is low down and the crown is light. Wind is what tests it: a tree that bends sheds the force, and the one that comes down is the one whose roots were cut on one side or left standing in waterlogged soil.",
        ),
    ),
    (
        "tall-things-stand-up",
        "why do sunflowers fall over",
        true,
        Some(
            "A plant in the ground or a grow bag goes over for one of four reasons, and the fix is the one that matches: it grew tall and soft reaching for light, so stake it loosely with a cane and a soft tie in a figure of eight and give it more sun; it was sown too thickly and the stems are thin, so thin them next time; the roots have been eaten or rotted, so lift the sick one and look; or wind and rain bent it, so a windbreak or a row of sturdier plants on that side is what stops it next year. Seedlings that flop at soil level are damping off from cold wet compost rather than anything above it.",
        ),
    ),
    (
        "tall-things-stand-up",
        "why do houseplants fall over",
        true,
        Some(
            "A houseplant flops over for one of three reasons: it has grown tall and thin reaching for light, the pot has become top-heavy for its roots, or the roots have rotted in soggy compost and let go. Move it somewhere brighter, put a cane or a hoop in and tie the stems loosely, and check the drainage holes before you water again - a pot standing in a full saucer is the usual cause.",
        ),
    ),
    (
        "tall-things-stand-up",
        "how do bridges and cranes not fall over",
        true,
        Some(
            "A tower crane stays up because its mast is bolted to a concrete base and tied into the building as that rises, and a counterweight on the short arm balances the load on the long one. It is built to lift a set weight at a set reach and no more, so it trips out before it can tip.",
        ),
    ),
    (
        "tall-things-stand-up",
        "what stops the tall wooden thing toppling",
        true,
        Some(
            "A tall building stays up because its weight runs straight down a braced frame into deep foundations, so wind bends it a little instead of tipping it, and the tallest carry a heavy damper near the top to soak up the sway. Pisa leans without falling because its weight still sits over its base: it sank slowly into soft ground, and soil taken out from the high side is what steadied it.",
        ),
    ),
    (
        "planet-size",
        "how big is mars",
        true,
        Some(
            "Mars is about 6,779 km across, a bit over half the Earth's width, so nearly two Marses would fit across the Earth's face.",
        ),
    ),
    (
        "planet-size",
        "how big is jupiter compared to earth",
        true,
        Some(
            "Jupiter is about 143,000 km across, eleven Earths side by side, and about 1,300 Earths would fit inside it: it is the biggest planet by a wide margin.",
        ),
    ),
    (
        "planet-size",
        "how big is earth compared to jupiter",
        true,
        None,
    ),
    (
        "planet-size",
        "how big is the sun compared to neptune",
        true,
        None,
    ),
    (
        "planet-size",
        "how big is pluto",
        true,
        Some(
            "Pluto is about 2,377 km across: about a fifth of the Earth's width, and smaller than our Moon.",
        ),
    ),
    (
        "planet-size",
        "whats the size of uranus",
        true,
        Some(
            "Uranus is about 50,724 km across, four Earths side by side, and about 63 Earths would fit inside it.",
        ),
    ),
    (
        "planet-size",
        "is venus bigger than mercury",
        true,
        Some(
            "Venus is about 12,104 km across, almost exactly the Earth's width - about 95% of it - and a little hotter than a pizza oven all over its surface.",
        ),
    ),
    (
        "planet-size",
        "how big is the moon next to saturn",
        true,
        None,
    ),
    (
        "planet-size",
        "what size is neptune4",
        true,
        Some(
            "Neptune is about 49,244 km across, nearly four Earths side by side, and about 58 Earths would fit inside it - a little smaller than Uranus but heavier.",
        ),
    ),
    ("planet-size", "is supermars bigger than earth", true, None),
    ("planet-size", "how big is it", true, None),
    ("what-water-is", "what is water", true, None),
    ("what-water-is", "what is water made of", true, None),
    ("what-water-is", "what is water retention", false, None),
    ("what-water-is", "what is water flossing", false, None),
    (
        "what-water-is",
        "what is the chemical formula of water",
        true,
        None,
    ),
    ("what-water-is", "what is water in simple terms", true, None),
    ("what-water-is", "is water a substance", true, None),
    ("what-water-is", "what is water heating", false, None),
    ("what-water-is", "what is h2o", true, None),
    ("what-water-is", "where is water near me", false, None),
];
