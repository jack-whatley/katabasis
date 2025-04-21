use tokio::sync::OnceCell;

static KATABASIS_EVENT: OnceCell<KatabasisEventHandler> = OnceCell::const_new();

pub struct KatabasisEventHandler {}
