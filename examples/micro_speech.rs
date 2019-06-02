#![no_main]
#![no_std]

extern crate panic_halt;

use tensorflow_lite_sys::tflite::{
    ops::micro::AllOpsResolver, ErrorReporter, FeatureProvider, GetModel, LatestAudioTimestamp,
    MicroErrorReporter, MicroInterpreter, Model, RecognizeCommands, RespondToCommand,
    SimpleTensorAllocator, TfLiteStatus, TfLiteTensor,
};

use cortex_m_rt::entry;
use stm32f1xx_hal::{delay::Delay, pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc9.into_push_pull_output(&mut gpioc.crh);

    let mut delay = Delay::new(cp.SYST, clocks);

    // // Set up logging.
    // tflite::MicroErrorReporter micro_error_reporter;
    // tflite::ErrorReporter* error_reporter = &micro_error_reporter;

    // // Map the model into a usable data structure. This doesn't involve any
    // // copying or parsing, it's a very lightweight operation.
    // const tflite::Model* model =
    //     ::tflite::GetModel(g_tiny_conv_micro_features_model_data);
    // if (model->version() != TFLITE_SCHEMA_VERSION) {
    //   error_reporter->Report(
    //       "Model provided is schema version %d not equal "
    //       "to supported version %d.\n",
    //       model->version(), TFLITE_SCHEMA_VERSION);
    //   return 1;
    // }

    // // This pulls in all the operation implementations we need.
    // tflite::ops::micro::AllOpsResolver resolver;

    // // Create an area of memory to use for input, output, and intermediate arrays.
    // // The size of this will depend on the model you're using, and may need to be
    // // determined by experimentation.
    // const int tensor_arena_size = 10 * 1024;
    // uint8_t tensor_arena[tensor_arena_size];
    // tflite::SimpleTensorAllocator tensor_allocator(tensor_arena,
    //                                                tensor_arena_size);

    // // Build an interpreter to run the model with.
    // tflite::MicroInterpreter interpreter(model, resolver, &tensor_allocator,
    //                                      error_reporter);

    // // Get information about the memory area to use for the model's input.
    // TfLiteTensor* model_input = interpreter.input(0);
    // if ((model_input->dims->size != 4) || (model_input->dims->data[0] != 1) ||
    //     (model_input->dims->data[1] != kFeatureSliceCount) ||
    //     (model_input->dims->data[2] != kFeatureSliceSize) ||
    //     (model_input->type != kTfLiteUInt8)) {
    //   error_reporter->Report("Bad input tensor parameters in model");
    //   return 1;
    // }

    // // Prepare to access the audio spectrograms from a microphone or other source
    // // that will provide the inputs to the neural network.
    // FeatureProvider feature_provider(kFeatureElementCount,
    //                                  model_input->data.uint8);

    // RecognizeCommands recognizer(error_reporter);

    // int32_t previous_time = 0;
    // // Keep reading and analysing audio data in an infinite loop.
    // while (true) {
    //   // Fetch the spectrogram for the current time.
    //   const int32_t current_time = LatestAudioTimestamp();
    //   int how_many_new_slices = 0;
    //   TfLiteStatus feature_status = feature_provider.PopulateFeatureData(
    //       error_reporter, previous_time, current_time, &how_many_new_slices);
    //   if (feature_status != kTfLiteOk) {
    //     error_reporter->Report("Feature generation failed");
    //     return 1;
    //   }
    //   previous_time = current_time;
    //   // If no new audio samples have been received since last time, don't bother
    //   // running the network model.
    //   if (how_many_new_slices == 0) {
    //     continue;
    //   }

    //   // Run the model on the spectrogram input and make sure it succeeds.
    //   TfLiteStatus invoke_status = interpreter.Invoke();
    //   if (invoke_status != kTfLiteOk) {
    //     error_reporter->Report("Invoke failed");
    //     return 1;
    //   }

    //   // The output from the model is a vector containing the scores for each
    //   // kind of prediction, so figure out what the highest scoring category was.
    //   TfLiteTensor* output = interpreter.output(0);
    //   uint8_t top_category_score = 0;
    //   for (int category_index = 0; category_index < kCategoryCount;
    //        ++category_index) {
    //     const uint8_t category_score = output->data.uint8[category_index];
    //     if (category_score > top_category_score) {
    //       top_category_score = category_score;
    //     }
    //   }

    //   const char* found_command = nullptr;
    //   uint8_t score = 0;
    //   bool is_new_command = false;
    //   TfLiteStatus process_status = recognizer.ProcessLatestResults(
    //       output, current_time, &found_command, &score, &is_new_command);
    //   if (process_status != kTfLiteOk) {
    //     error_reporter->Report(
    //         "RecognizeCommands::ProcessLatestResults() failed");
    //     return 1;
    //   }
    //   // Do something based on the recognized command. The default implementation
    //   // just prints to the error console, but you should replace this with your
    //   // own function for a real application.
    //   RespondToCommand(error_reporter, current_time, found_command, score,
    //                    is_new_command);
    // }

    loop {
        led.set_high();
        delay.delay_ms(1_000_u16);
        led.set_low();
        delay.delay_ms(1_000_u16);
    }
}
