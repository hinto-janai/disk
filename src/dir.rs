//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Types of User Dirs
/// The different types of OS directories, provided by [`directories`](https://docs.rs/directories)
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
pub enum Dir {
	/// |Platform | Value                                                                 | Example                                             |
	/// | ------- | --------------------------------------------------------------------- | --------------------------------------------------- |
	/// | Linux   | `$XDG_CACHE_HOME`/`_project_path_` or `$HOME`/.cache/`_project_path_` | /home/alice/.cache/barapp                           |
	/// | macOS   | `$HOME`/Library/Caches/`_project_path_`                               | /Users/Alice/Library/Caches/com.Foo-Corp.Bar-App    |
	/// | Windows | `{FOLDERID_LocalAppData}`\\`_project_path_`\\cache                    | C:\Users\Alice\AppData\Local\Foo Corp\Bar App\cache |
	Project,

	/// |Platform | Value                                                                 | Example                                             |
	/// | ------- | --------------------------------------------------------------------- | --------------------------------------------------- |
	/// | Linux   | `$XDG_CACHE_HOME`/`_project_path_` or `$HOME`/.cache/`_project_path_` | /home/alice/.cache/barapp                           |
	/// | macOS   | `$HOME`/Library/Caches/`_project_path_`                               | /Users/Alice/Library/Caches/com.Foo-Corp.Bar-App    |
	/// | Windows | `{FOLDERID_LocalAppData}`\\`_project_path_`\\cache                    | C:\Users\Alice\AppData\Local\Foo Corp\Bar App\cache |
	Cache,

	/// |Platform | Value                                                                   | Example                                                        |
	/// | ------- | ----------------------------------------------------------------------- | -------------------------------------------------------------- |
	/// | Linux   | `$XDG_CONFIG_HOME`/`_project_path_` or `$HOME`/.config/`_project_path_` | /home/alice/.config/barapp                                     |
	/// | macOS   | `$HOME`/Library/Application Support/`_project_path_`                    | /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App  |
	/// | Windows | `{FOLDERID_RoamingAppData}`\\`_project_path_`\\config                   | C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config         |
	Config,

	#[default]
	/// This is the default value.
	///
	/// |Platform | Value                                                                      | Example                                                       |
	/// | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------- |
	/// | Linux   | `$XDG_DATA_HOME`/`_project_path_` or `$HOME`/.local/share/`_project_path_` | /home/alice/.local/share/barapp                               |
	/// | macOS   | `$HOME`/Library/Application Support/`_project_path_`                       | /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App |
	/// | Windows | `{FOLDERID_RoamingAppData}`\\`_project_path_`\\data                        | C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\data          |
	Data,

	/// |Platform | Value                                                                      | Example                                                       |
	/// | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------- |
	/// | Linux   | `$XDG_DATA_HOME`/`_project_path_` or `$HOME`/.local/share/`_project_path_` | /home/alice/.local/share/barapp                               |
	/// | macOS   | `$HOME`/Library/Application Support/`_project_path_`                       | /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App |
	/// | Windows | `{FOLDERID_LocalAppData}`\\`_project_path_`\\data                          | C:\Users\Alice\AppData\Local\Foo Corp\Bar App\data            |
	DataLocal,

	/// |Platform | Value                                                                   | Example                                                |
	/// | ------- | ----------------------------------------------------------------------- | ------------------------------------------------------ |
	/// | Linux   | `$XDG_CONFIG_HOME`/`_project_path_` or `$HOME`/.config/`_project_path_` | /home/alice/.config/barapp                             |
	/// | macOS   | `$HOME`/Library/Preferences/`_project_path_`                            | /Users/Alice/Library/Preferences/com.Foo-Corp.Bar-App  |
	/// | Windows | `{FOLDERID_RoamingAppData}`\\`_project_path_`\\config                   | C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config |
	Preference,
}
