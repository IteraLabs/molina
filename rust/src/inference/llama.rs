/*
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

fn main() {
    // setup the parameters for loading the large language model file
    let model_params = ModelOptions {
        // the airoboros 7b model reference in this example can handle
        // a context size of 4096, but more VRAM constrained machines
        // might not be able to cope. Smaller context means less VRAM
        // used, and we won't need this much for this example.
        context_size: 2048,

        // this determines how many of the 'layers' in the LLM get sent
        // to the GPU for processing. specifying a large number just
        // ensures all layers are offloaded. if you want to run on the
        // CPU, comment this parameter out or set to 0.
        n_gpu_layers: 9999,

        ..Default::default()
    };

    // load the model we downloaded with the above parameters. it will need
    // to be `mut` because running Predict can update internal cached data.
    let llama_filepath_str = "airoboros-l2-7b-2.2.1.Q4_K_S.gguf".to_string();
    let mut llm_model = match LLama::new(llama_filepath_str, &model_params) {
        Ok(m) => m,
        Err(err) => panic!("Failed to load model: {err}"),
    };

    // setup the parameters that control the text prediction. only some
    // of the possibilities are shown here.
    let predict_options = PredictOptions {
        // predict at most 256 tokens, which will hard stop the prediction
        // at that point if the model doesn't finish earlier.
        tokens: 256,

        // this is how big the 'prompt' chunks should be for processing.
        // if using only CPU, you may wish to set this much lower, to
        // something like 8.
        batch: 512,

        // the following three options are 'sampler' settings to control
        // how the text is predicted. play with the possibilities here...
        temperature: 1.3,
        min_p: 0.05,
        penalty: 1.03,

        // we define our callback with a closure and just have it print
        // the generated tokens to stdout.
        token_callback: Some(std::sync::Arc::new(move |token| {
            print!("{}", token);
            let _ = std::io::Write::flush(&mut std::io::stdout());
            true // returning true means text prediction should continue
        })),
        ..Default::default()
    };

    // this is the 'prompt' to send the LLM that is basically telling it
    // what to write. different models have different formats and this
    // is the format for Airoboros 2.2.1.
    let prompt = "A chat\n\
        USER: Write the basic components of the scientific method\
        .\n\
        ASSISTANT:";

    // finally, call predict() to generate the text. the callback in
    // predict_options will get each token as it happens, but the final
    // result will be in the (String, LLamaPredictTimings) tuple of
    // the Result object returned.
    let _result = llm_model.predict(prompt.to_string(), &predict_options);
}
*/
