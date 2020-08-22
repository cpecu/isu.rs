use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  LoadDemo { path: String },
  SaveDemo { path: String },
  OpenImage { path: String, kind: ImageKind },
  AttachAudio,
  Section,
  Pace,
}

#[derive(Deserialize)]
pub enum ImageKind {
    Bg,
    Shell,
    CombinedBgShell,
    Insert,
    Asset,
}
