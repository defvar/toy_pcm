use toy_pcm::{hz, Playback};

#[test]
fn next_phase() {
    let mut p = hz::rate(44_100.0).const_hz(440.0).phase();
    println!("{:?}", p.next_phase());
    println!("{:?}", p.next_phase());
    println!("{:?}", p.next_phase());
    println!("{:?}", p.next_phase());
}

#[test]
fn test_play() {
    let hz = hz::rate(44_100.0).const_hz(440.0);
    let p = Playback::new(hz.clone().phase().sine(), 3);
    let p2 = Playback::new(hz.clone().phase().saw(), 3);
    let mut p3 = p.chain(p2);
    println!("1:{:?}", p3.next());
    println!("2:{:?}", p3.next());
    println!("3:{:?}", p3.next());
    println!("4:{:?}", p3.next());
    println!("5:{:?}", p3.next());
    println!("6:{:?}", p3.next());
    println!("7:{:?}", p3.next());
}
