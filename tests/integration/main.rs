use envsafe::*;

#[test]
fn app_env() {
    EnvSafeConfig::<1, 2>::new()
        .add_envfile("common.env")
        .app_env_config(
            AppEnvConfig::new("APP_ENV")
                .add_app_env(
                    "DEV",
                    EnvConfig::new()
                        .add_envfile_override("dev.env")
                        .sequence(EnvSequence::EnvThenFiles)
                        .errors(ErrorReturn::All),
                )
                .add_app_env(
                    "PROD",
                    EnvConfig::new()
                        .add_envfile("prod.env")
                        .sequence(EnvSequence::EnvThenFiles)
                        .errors(ErrorReturn::None),
                ),
        )
        .load()
        .unwrap();
}

#[test]
fn default_configure() {
    EnvSafeConfig::<1>::default().load().unwrap();
}

#[test]
fn default() {
    load().unwrap();
}
