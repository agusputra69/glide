//! What to do with a running export when the editor is about to go away.

/// The way the editor is being left.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leaving {
    Quit,
    CloseProject,
    OpenProject,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportGuard {
    /// No export is running: carry on.
    Proceed,
    /// An export is running and nobody can be asked: cancel it and carry on.
    ProceedCancelling,
    /// An export is running: ask first, and cancel it only on a yes.
    Ask { prompt: &'static str },
}

pub fn export_guard(export_running: bool, scripted: bool, leaving: Leaving) -> ExportGuard {
    match (export_running, scripted) {
        (false, _) => ExportGuard::Proceed,
        (true, true) => ExportGuard::ProceedCancelling,
        (true, false) => ExportGuard::Ask {
            prompt: match leaving {
                Leaving::Quit => "An export is running. Cancel it and quit?",
                Leaving::CloseProject => "An export is running. Cancel it and close this recording?",
                Leaving::OpenProject => "An export is running. Cancel it and open another recording?",
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Leaving; 3] = [Leaving::Quit, Leaving::CloseProject, Leaving::OpenProject];

    #[test]
    fn nothing_running_never_asks() {
        for scripted in [false, true] {
            for leaving in ALL {
                assert_eq!(export_guard(false, scripted, leaving), ExportGuard::Proceed);
            }
        }
    }

    #[test]
    fn scripted_runs_cancel_without_asking() {
        for leaving in ALL {
            assert_eq!(export_guard(true, true, leaving), ExportGuard::ProceedCancelling);
        }
    }

    #[test]
    fn an_interactive_run_asks_in_words_that_match_the_action() {
        let prompt = |leaving| match export_guard(true, false, leaving) {
            ExportGuard::Ask { prompt } => prompt,
            other => panic!("expected a question, got {other:?}"),
        };
        assert_eq!(prompt(Leaving::Quit), "An export is running. Cancel it and quit?");
        assert_eq!(prompt(Leaving::CloseProject), "An export is running. Cancel it and close this recording?");
        assert_eq!(prompt(Leaving::OpenProject), "An export is running. Cancel it and open another recording?");
    }
}
