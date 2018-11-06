
// NOTE: replicated from main serde source

macro_rules! feature_cache {
    ($enabled:block else $disabled:block) => {
        {
            $enabled
        }
    }
}
