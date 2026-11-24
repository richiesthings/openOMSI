use super::*;

/// The MileZero switch and its volume come off by default at 80 %, are read from the file,
/// written once each, and kept within 0..1.
#[test]
fn milezero_settings_round_trip_through_the_launcher() {
    let fresh = settings_from_text(None);
    assert_eq!(fresh["milezero"], false);
    assert_eq!(fresh["milezero_volume"], 0.8);
    let old = "version=4\nmilezero=1\nmilezero_volume=0.35\n";
    let mut v = settings_from_text(Some(old));
    assert_eq!(v["milezero"], true);
    assert_eq!(v["milezero_volume"], 0.35);
    v["milezero_volume"] = json!(1.7);
    let text = settings_to_text(&v, Some(old));
    assert_eq!(
        text.lines()
            .filter(|l| l.starts_with("milezero="))
            .collect::<Vec<_>>(),
        ["milezero=1"]
    );
    assert_eq!(
        text.lines()
            .filter(|l| l.starts_with("milezero_volume="))
            .collect::<Vec<_>>(),
        ["milezero_volume=1"]
    );
    assert_eq!(
        settings_from_text(Some("milezero_volume=nonsense\n"))["milezero_volume"],
        0.8
    );
}
