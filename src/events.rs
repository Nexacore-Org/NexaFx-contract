use soroban_sdk::Env;

use crate::conversion::ConversionEvent;

/// Publishes a conversion event to the environment
pub fn publish(env: &Env, event: ConversionEvent) {
    env.events().publish(("conversion",), event);
}
