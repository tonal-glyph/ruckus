// use alsa::*;
use jack::*;
// use pulse_simple::*;
// use pulse_simple::Playback;
use std::f64::consts::PI;
const RATE: u32 = 48000;
// pub fn main() {
//     // jack
//     // Create client
//     let (client, _status) =
//         jack::Client::new("rust_jack_simple", jack::ClientOptions::NO_START_SERVER).unwrap();
//     // Register ports. They will be used in a callback that will be
//     // called when new data is available.
//     let in_a = client
//         .register_port("rust_in_l", jack::AudioIn::default())
//         .unwrap();
//     let in_b = client
//         .register_port("rust_in_r", jack::AudioIn::default())
//         .unwrap();
//     let mut out_a = client
//         .register_port("rust_out_l", jack::AudioOut::default())
//         .unwrap();
//     let mut out_b = client
//         .register_port("rust_out_r", jack::AudioOut::default())
//         .unwrap();
//     let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
//         let out_a_p = out_a.as_mut_slice(ps);
//         let out_b_p = out_b.as_mut_slice(ps);
//         let in_a_p = in_a.as_slice(ps);
//         let in_b_p = in_b.as_slice(ps);
//         out_a_p.clone_from_slice(&in_a_p);
//         out_b_p.clone_from_slice(&in_b_p);
//         jack::Control::Continue
//     };
//     let process = jack::ClosureProcessHandler::new(process_callback);
//     // Activate the client, which starts the processing.
//     let active_client = client.activate_async(Notifications, process).unwrap();
//     // Wait for user input to quit
//     println!("Press enter/return to quit...");
//     let mut user_input = String::new();
//     io::stdin().read_line(&mut user_input).ok();
//     active_client.deactivate().unwrap();

//     // pulseaudio
//     let p = Playback::new("Example", "Playback", None, RATE);
//     // Generate 1s of sound
//     let mut data = Vec::with_capacity(RATE as usize);
//     for i in 0..RATE {
//         let t = i as f64 / RATE as f64;
//         let make_freq = |f: f64| ((std::i16::MAX as f64) * (f * 2.0 * PI * t).sin()) as i16;
//         data.push([make_freq(440.0), make_freq(330.0)]);
//     }
//     // Play in a loop
//     loop {
//         p.write(&data[..]);
//     }
// }
