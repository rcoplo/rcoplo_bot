
#[macro_export]
macro_rules! plugin_settings {
    ($struct_name:ident,$name:expr,$usage:expr,$default:expr) =>{
        pub struct $struct_name ;
        impl crate::PluginSettings for $struct_name{
            type Config = ();
            fn name(&self) -> &'static str {
                $name
            }
            fn usage(&self) -> &'static str {
                $usage
            }
            fn default(&self) -> bool {
                $default
            }
            fn plugin_configs(&self) -> Option<Self::Config> {
                None
            }
    }
    };
    ($struct_name:ident,$name:expr,$usage:expr,$default:expr,$config_name:ident{$($config_:ident:$ty:ty => $data:expr),*}) =>{
        pub struct $struct_name;

        #[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
        pub struct $config_name {
            $($config_:$ty),*
        }

        impl std::default::Default for $config_name{
            fn default() -> Self {
                Self{
                    $($config_:$data),*
                }
            }
        }

        impl crate::PluginSettings for $struct_name {
            type Config = $config_name;
            fn name(&self) -> &'static str {
                $name
            }
            fn usage(&self) -> &'static str {
                $usage
            }
            fn default(&self) -> bool {
                $default
            }
            fn plugin_configs(&self) -> Option<Self::Config> {
                Some($config_name::default())
            }

        }
    };
}
pub trait PluginSettings{
    type Config: for<'a> serde::Deserialize<'a> + serde::Serialize + Send+ Sync + 'static;
    fn name(&self) -> &'static str;
    fn usage(&self) -> &'static str;
    fn default(&self) -> bool;
    fn plugin_configs(&self) -> Option<Self::Config>;
}

#[macro_export]
macro_rules! plugin_sqlite {
    () => {};
}


#[macro_export]
macro_rules! resource_path {
    ($($path:literal),*) => {{
        // let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // path.push("resources");
        let mut path = String::new();
        path.push_str("./resources");
        $(
        path.push_str(&format!("/{}",$path));
        )*
        // path.to_string_lossy().to_string()
        path
    }};
}

#[macro_export]
macro_rules! resource_tmp_path {
    ($name:literal) => {
        resource_tmp_path!(=>$name);
    };
    ($($path:literal),* => $name:literal) => {{
        let mut path = String::new();
        path.push_str("./resources");
        // let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // path.push("resources");
        $(
        path.push_str(&format!("/{}",$path));
        )*

        path.push_str(format!("/{}_{}", rbatis::rbdc::uuid::Uuid::new().0,$name));
        // path.to_string_lossy().to_string()
        path
    }};

}
#[macro_export]
macro_rules! plugin_config {
    ($($name:ident:$ty:ident),*) =>{
        pub static PLUGIN_CONFIG:once_cell::sync::Lazy<PluginConfig> = once_cell::sync::Lazy::new(||{
            PluginConfig::default()
        });
        pub struct PluginConfig {
            $(pub $name:$ty),*
        }

        impl Default for PluginConfig {
            fn default() -> Self {
                Self{
                    $($name:$ty::default()),*
                }
            }
        }
    }
}
#[macro_export]
macro_rules! base_plugins_config {
    ($($name:ident:$ty:ident),*) =>{

        pub struct BasePluginConfig {
            $(pub $name:$ty),*
        }
        impl Default for  BasePluginConfig {
            fn default() -> Self {
                Self{
                    $($name:$ty::default()),*
                }
            }
        }
    }
}
#[macro_export]
macro_rules! bot_context {
    ($($sql_name:ident => $name:expr),*) => {
        pub static BOT_CONTEXT: once_cell::sync::Lazy<BotContext> = once_cell::sync::Lazy::new(||{
            BotContext::default()
        });

        pub struct BotContext{
            rb:rbatis::Rbatis,
            plugin: crate::plugins::PluginConfig,
            base_plugins: crate::base_plugins::BasePluginConfig,
        }

        impl Default for BotContext{
            fn default() -> Self {
                Self{
                    rb: rbatis::Rbatis::new(),
                    plugin: crate::plugins::PluginConfig::default(),
                    base_plugins: crate::base_plugins::BasePluginConfig::default(),
                }
            }
        }

        impl BotContext {
            pub async fn init_pool(&self) {
                let path = resource_path!("data","bot.db");
                println!("db path: {}",&path);
                self.rb.init(rbdc_sqlite::driver::SqliteDriver {}, path.as_str()).unwrap();
                let mut s = rbatis::table_sync::SqliteTableSync::default();
                s.sql_id = " PRIMARY KEY AUTOINCREMENT NOT NULL ".to_string();
                use rbatis::table_sync::TableSync;
                $(
                 s.sync(self.rb.acquire().await.unwrap(), rbs::to_value!($sql_name{
                    id:Some(0),
                    ..Default::default()
                    }), $name)
                    .await
                    .unwrap();
                )*
            }
        }
    };
}
#[cfg(test)]
mod test {
    pub static BOT_CONTEXT: once_cell::sync::Lazy<BotContext> = once_cell::sync::Lazy::new(||{
        BotContext::default()
    });

    pub struct BotContext{
        rb:rbatis::Rbatis,
        plugin: crate::plugins::PluginConfig,
        base_plugins: crate::base_plugins::BasePluginConfig,
    }

    impl Default for BotContext{
        fn default() -> Self {
            Self{
                rb: rbatis::Rbatis::new(),
                plugin: crate::plugins::PluginConfig::default(),
                base_plugins: crate::base_plugins::BasePluginConfig::default(),
            }
        }
    }

    impl BotContext {
        pub async fn init_pool(&self) {
            let path = resource_path!("data","bot.db");
            println!("db path: {}",&path);
            self.rb.init(rbdc_sqlite::driver::SqliteDriver {}, path.as_str()).unwrap();
            let mut s = rbatis::table_sync::SqliteTableSync::default();
            s.sql_id = " PRIMARY KEY AUTOINCREMENT NOT NULL ".to_string();
            use rbatis::table_sync::TableSync;
            s.sync(self.rb.acquire().await.unwrap(), rbs::to_value!(BiliPush{
                id:Some(0),
                ..Default::default()
        }), )
                .await
                .unwrap();
        }
    }
    fn path(){
        let mut path = String::new();

    }
}