use crate::common::*;
use crate::wayland_clipboard::WaylandClipboardContext;
use crate::x11_clipboard::{Clipboard, X11ClipboardContext};
use crate::Result;

enum LinuxContext {
    Wayland(WaylandClipboardContext),
    X11(X11ClipboardContext),
}

pub struct LinuxClipboardContext {
    context: LinuxContext,
}

impl ClipboardProvider for LinuxClipboardContext {
    fn new() -> Result<LinuxClipboardContext> {
        match WaylandClipboardContext::new() {
            Ok(context) => Ok(LinuxClipboardContext {
                context: LinuxContext::Wayland(context),
            }),
            Err(_) => match X11ClipboardContext::<Clipboard>::new() {
                Ok(context) => Ok(LinuxClipboardContext {
                    context: LinuxContext::X11(context),
                }),
                Err(err) => Err(err),
            },
        }
    }

    fn get_contents(&mut self) -> Result<String> {
        match &mut self.context {
            LinuxContext::Wayland(context) => context.get_contents(),
            LinuxContext::X11(context) => context.get_contents(),
        }
    }

    fn set_contents(&mut self, content: String) -> Result<()> {
        match &mut self.context {
            LinuxContext::Wayland(context) => context.set_contents(content),
            LinuxContext::X11(context) => context.set_contents(content),
        }
    }

    fn clear(&mut self) -> Result<()> {
        match &mut self.context {
            LinuxContext::Wayland(context) => context.clear(),
            LinuxContext::X11(context) => context.clear(),
        }
    }
}
