pub enum Text {
    Info,
    Shortcuts,
    BottomBar,
    AppTitle,
}

impl Text {
    pub fn text(&self) -> String {
        match self {
            Self::Info => "(c) Kamil Wdowicz".to_string(),
            Self::Shortcuts => "[F5] Refresh | [Esc] Quit".to_string(),
            Self::BottomBar => format!("{} | {}", Self::Info.text(), Self::Shortcuts.text()),
            Self::AppTitle => "DockerFace".to_string(),
        }
    }
}

pub enum Status {
    Ready,
    Refreshing,
    StoppingContainer,
    NoDocker,
}

impl Status {
    pub fn text(&self) -> String {
        match self {
            Self::Ready => "Ready".to_string(),
            Self::Refreshing => "Refreshing".to_string(),
            Self::StoppingContainer => "Stoping container".to_string(),
            Self::NoDocker => "Docker service unreachable".to_string(),
        }
    }
}
