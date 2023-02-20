use js_sys::*;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::*;
use yew::prelude::*;

static VIDEO_CODEC: &str = "vp09.00.10.08";
const VIDEO_HEIGHT: i32 = 1280i32;
const VIDEO_WIDTH: i32 = 720i32;

#[function_component(Producer)]
fn producer() -> Html {
    // query, async
    wasm_bindgen_futures::spawn_local(async move {
        let navigator = window().unwrap().navigator();
        let media_devices = navigator.media_devices().unwrap();
        let video_element = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("webcam")
            .unwrap()
            .unchecked_into::<HtmlVideoElement>();
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&Boolean::from(true));
        let devices_query = media_devices
            .get_user_media_with_constraints(&constraints)
            .unwrap();
        let device = JsFuture::from(devices_query)
            .await
            .unwrap()
            .unchecked_into::<MediaStream>();
        video_element.set_src_object(Some(&device));
        let video_track = Box::new(
            // safely pass in Box
            device
                .get_video_tracks()
                .find(&mut |_: JsValue, _: u32, _: Array| true)
                .unchecked_into::<VideoTrack>(),
        );

        let error_handler = Closure::wrap(Box::new(move |e: JsValue| {
            console::log_1(&JsString::from("on error"));
            console::log_1(&e);
        }) as Box<dyn FnMut(JsValue)>);

        let output_handler = Closure::wrap(Box::new(move |chunk: JsValue| {
            // let video_chunk = chunk.unchecked_into::<EncodedVideoChunk>();
            console::log_1(&chunk);
        }) as Box<dyn FnMut(JsValue)>);
        let video_encoder_init = VideoEncoderInit::new(
            error_handler.as_ref().unchecked_ref(),
            output_handler.as_ref().unchecked_ref(),
        );
        let video_encoder = VideoEncoder::new(&video_encoder_init).unwrap();
        let video_encoder_config =
            VideoEncoderConfig::new(&VIDEO_CODEC, VIDEO_HEIGHT as u32, VIDEO_WIDTH as u32);
        video_encoder.configure(&video_encoder_config);
        let processor = MediaStreamTrackProcessor::new(&MediaStreamTrackProcessorInit::new(
            &video_track.unchecked_into::<MediaStreamTrack>(),
        ))
        .unwrap();

        // pull the frames out of the stream and send them to the encoder
        let reader = processor
            .readable()
            .get_reader()
            .unchecked_into::<ReadableStreamDefaultReader>();
        loop {
            let result = JsFuture::from(reader.read()).await.map_err(|e| {
                console::log_1(&e);
            });
            match result {
                Ok(js_frame) => {
                    let video_frame = Reflect::get(&js_frame, &JsString::from("value"))
                        .unwrap()
                        .unchecked_into::<VideoFrame>();
                    video_encoder.encode(&video_frame);
                    video_frame.close();
                }
                Err(_e) => {
                    console::log_1(&JsString::from("error"));
                }
            }
        }
    });
    // let mut constraints = MediaStreamConstraints::new();
    // constraints.video(&Boolean::from(true))

    html!(
        <div class="producer">
            <h3>{"Producer"}</h3>
            <video autoplay=true id="webcam"></video>
        </div>
    )
}

#[function_component(Consumer)]
fn consumer() -> Html {
    html!(
        <div class="consumer">
            <h3>{"Consumer"}</h3>
        </div>
    )
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <div class={"grid"}>
            <Producer/>
            <Consumer/>
        </div>
    )
}

fn main() {
    // yew::Renderer::<App>::new().render();
    yew::start_app::<App>();
}