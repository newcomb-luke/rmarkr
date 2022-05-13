#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::RMarkrApp;
use theme::Theme;

mod app;
mod theme;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.transparent = true;
    options.vsync = true;
    options.maximized = true;

    let theme = Theme::Dark;

    eframe::run_native(
        "Test",
        options,
        Box::new(move |cc| Box::new(RMarkrApp::new(cc, theme))),
    );
}

pub const TEST_SRC: &'static str = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Nulla consectetur neque id urna semper ullamcorper.
In non tellus ac est interdum vestibulum sit amet eu metus.
Donec justo purus, aliquet vitae pulvinar aliquet, laoreet et nibh.
Praesent accumsan augue vehicula tellus eleifend, eget molestie leo interdum.
Donec fringilla massa vitae magna elementum mollis.
Donec purus mauris, gravida sit amet aliquam vitae, lacinia quis dolor.
Ut quis rutrum erat. In hac habitasse platea dictumst.
Aenean eget magna vulputate, semper sem sit amet, tempus mi. 
Nullam tempus tristique felis vel pulvinar. Vestibulum nec posuere dui. 
Nunc varius dui quis purus facilisis, sed pellentesque ex efficitur.
Cras sollicitudin bibendum elit a semper.
Suspendisse rhoncus, urna vel gravida tincidunt, felis nibh vestibulum urna, et dapibus quam nibh a neque.

Morbi et purus erat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Sed finibus, nisl at suscipit lobortis, arcu lacus dictum sem, non vestibulum leo mauris id risus. Maecenas at lorem quis justo blandit finibus nec non nisi. Fusce quis felis faucibus, lacinia nisl sit amet, gravida risus. Interdum et malesuada fames ac ante ipsum primis in faucibus. Aenean facilisis tortor et gravida venenatis. Praesent in vestibulum purus. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Ut dictum eleifend vulputate. Etiam facilisis purus vel eleifend dapibus. Aliquam sed venenatis dolor. Duis quam nunc, interdum feugiat egestas quis, mattis eget arcu.

Fusce lacinia mi lectus. Proin lectus nulla, dapibus id nibh id, molestie condimentum sem. Nullam urna turpis, congue quis velit eu, venenatis fermentum nisl. Mauris porttitor ultricies tortor, vitae posuere mi finibus et. Ut vitae placerat augue. Phasellus varius massa elit, nec pulvinar nulla consectetur eget. Integer auctor ac ligula eu suscipit. Nam auctor velit in purus elementum cursus. Etiam consectetur blandit aliquam. Suspendisse mattis venenatis justo eu efficitur. Mauris venenatis eros magna, sit amet facilisis eros luctus commodo. Proin viverra lacus sit amet est semper, quis condimentum libero tempus. Mauris tincidunt orci vitae nulla varius posuere. In gravida massa sed dolor finibus, vel malesuada massa eleifend.

Sed rutrum vehicula metus, sed mattis ipsum tincidunt sed. Integer in aliquam massa, et sodales turpis. Vivamus faucibus, nisi eget sagittis pretium, nunc felis tristique lacus, eget dictum mauris est eget lorem. Aenean sed enim euismod, ultrices diam id, lacinia enim. Aenean faucibus rutrum egestas. Vivamus sed tortor pellentesque, imperdiet velit a, tempus mauris. Morbi molestie nulla et lacinia tempor. Phasellus laoreet eleifend erat, ut mollis lorem porttitor eu. Donec dolor lorem, euismod in nisi in, finibus cursus sem. Aliquam erat volutpat. Nulla ultricies enim id neque venenatis sagittis. Ut vestibulum pulvinar molestie.

Aliquam a pulvinar mauris, molestie malesuada libero. Mauris mattis augue ante, at vestibulum leo mattis sit amet. Etiam efficitur aliquet tincidunt. Nullam sodales massa odio, ut cursus orci commodo nec. Duis a nisl eget nisi sodales convallis. Sed dictum fermentum risus, sit amet facilisis odio dictum sed. Donec nec ornare lacus. Nulla facilisi. Mauris imperdiet odio nec lobortis mattis. Mauris malesuada pharetra suscipit. Nam mollis diam eu mi aliquet, et interdum enim laoreet. Praesent interdum consequat justo eget vestibulum. Integer sit amet quam ex. Sed consequat luctus turpis ut porta.
"#;
