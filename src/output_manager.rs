use crate::output_handler::SfOutputHandler;
use wlroots::*;

#[derive(Debug)]
pub struct OutputManager;

impl OutputManager {
    pub fn new() -> OutputManager {
        OutputManager
    }
}

impl OutputManagerHandler for OutputManager {
    fn output_added<'output>(
        &mut self,
        _compositor: CompositorHandle,
        output: OutputBuilder<'output>,
    ) -> Option<OutputBuilderResult<'output>> {
        let name = output.handle().run(|out| out.name()).unwrap();
        info!("Output added! {}", name);
        Some(output.build_best_mode(SfOutputHandler::new()))
    }
}
