use tauri::{plugin::Plugin, window::Window, Runtime};

pub struct TransparentWryPlugin;
impl<R: Runtime> Plugin<R> for TransparentWryPlugin {
    fn name(&self) -> &'static str {
        "transparent-wry"
    }

    fn created(&mut self, window: Window<R>) {
        let _ = window.with_webview(|webview| {
            // #[cfg(target_os = "linux")]
            // {
            //   // see https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/struct.WebView.html
            //   // and https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/trait.WebViewExt.html
            //   use webkit2gtk::traits::WebViewExt;
            //   webview.inner().set_zoom_level(4.);
            // }

            #[cfg(windows)]
            {
                // see https://docs.rs/webview2-com/0.19.1/webview2_com/Microsoft/Web/WebView2/Win32/struct.ICoreWebView2Controller.html
                use webview2_com::Microsoft::Web::WebView2::Win32::*;
                use windows_core::ComInterface;
                let controller = webview.controller();
                let controller2: ICoreWebView2Controller2 = controller.cast().unwrap();
                unsafe {
                    let _ = controller2
                        .SetDefaultBackgroundColor(COREWEBVIEW2_COLOR {
                            R: 0,
                            G: 0,
                            B: 0,
                            A: 0,
                        })
                        .map_err(webview2_com::Error::WindowsError);
                }
            }

            // #[cfg(target_os = "macos")]
            // unsafe {
            //   let () = msg_send![webview.inner(), setPageZoom: 4.];
            //   let () = msg_send![webview.controller(), removeAllUserScripts];
            //   let bg_color: cocoa::base::id = msg_send![class!(NSColor), colorWithDeviceRed:0.5 green:0.2 blue:0.4 alpha:1.];
            //   let () = msg_send![webview.ns_window(), setBackgroundColor: bg_color];
            // }
        });
    }
}

pub fn init() -> TransparentWryPlugin {
    TransparentWryPlugin
}
