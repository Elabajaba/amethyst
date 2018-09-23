use amethyst_assets::{AssetStorage, Loader, SimpleFormat};
use format::{FontAsset, FontHandle, TtfFormat};
use font::systemfont::default_system_font;
use font_kit::handle::Handle as FontKitHandle;

/// Get the system default fonts.
/// If unable to, gets the local square.ttf font.
pub fn get_default_font(loader: &Loader, storage: &AssetStorage<FontAsset>) -> FontHandle {
	let system_font = default_system_font();

	match system_font {
		Ok(handle) => match handle {
			FontKitHandle::Path{..} => unimplemented!(),
			FontKitHandle::Memory{bytes, ..} => {
				let font_data = TtfFormat.import(bytes.to_vec(), ());
				match font_data {
					Ok(data) => return loader.load_from_data(data, (), storage),
					Err(e) => warn!("Failed to load default system font from bytes. Falling back to built-in.\nError: {:?}", e),
				}
			}
		}
		Err(e) => warn!("Failed to find suitable default system font. Falling back to built-in.\nError: {:?}", e),
	}

	return loader.load_from_data(TtfFormat.import(include_bytes!("./square.ttf").to_vec(), ()).unwrap(), (), storage);
} 