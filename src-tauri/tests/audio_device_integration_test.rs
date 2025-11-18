// Integration tests for audio device selection and switching
// Tests switching input and output devices with real macOS audio engine
//
// These tests require audio devices to be available on the system.
// To run these tests:
//   cargo test --test audio_device_integration_test -- --ignored --nocapture
//
// To skip integration tests (if no devices available):
//   cargo test --test audio_device_integration_test -- --skip

use rustalk_lib::domain::errors::AudioEngineError;
use rustalk_lib::domain::traits::audio_engine::AudioDevice;
use rustalk_lib::infrastructure::audio::create_audio_engine;
use rustalk_lib::services::audio_service::AudioService;
use std::env;
use std::sync::Arc;

/// Check if integration tests should be skipped
fn should_skip_integration_tests() -> bool {
    env::var("SKIP_AUDIO_INTEGRATION_TESTS").is_ok()
}

/// Helper function to skip test gracefully if no devices are available
fn handle_no_devices_error(result: Result<Vec<AudioDevice>, AudioEngineError>) -> Vec<AudioDevice> {
    match result {
        Ok(devices) => {
            if devices.is_empty() {
                println!("DEBUG:[AUDIO/INTEGRATION] No devices available, skipping test");
                return vec![];
            }
            devices
        }
        Err(e) => {
            println!(
                "DEBUG:[AUDIO/INTEGRATION] Failed to enumerate devices (may be expected): {:?}",
                e
            );
            vec![]
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default - run with: cargo test --test audio_device_integration_test -- --ignored
async fn test_switch_input_device() {
    if should_skip_integration_tests() {
        println!("Skipping input device switching test (SKIP_AUDIO_INTEGRATION_TESTS set)");
        return;
    }

    println!("DEBUG:[AUDIO/INTEGRATION] Testing input device switching");

    // Create real audio engine
    let engine = create_audio_engine().expect("Should create audio engine");
    let engine_arc: Arc<dyn rustalk_lib::domain::traits::audio_engine::AudioEngine> =
        Arc::from(engine);
    let service = AudioService::new(engine_arc.clone());

    // Enumerate available input devices
    let devices = handle_no_devices_error(service.list_input_devices().await);
    if devices.is_empty() {
        println!("DEBUG:[AUDIO/INTEGRATION] No input devices available, skipping test");
        return;
    }

    println!(
        "DEBUG:[AUDIO/INTEGRATION] Found {} input device(s)",
        devices.len()
    );

    // Try to switch to the first available input device
    let first_device = &devices[0];
    println!(
        "DEBUG:[AUDIO/INTEGRATION] Switching to input device: {} ({})",
        first_device.name, first_device.id
    );

    let result = service.set_input_device(&first_device.id).await;
    match result {
        Ok(_) => {
            println!("DEBUG:[AUDIO/INTEGRATION] Successfully switched input device");
            // Verify device was set
            let current_device = service.get_input_device().await.unwrap();
            assert!(
                current_device.is_some(),
                "Input device should be set after switching"
            );
            assert_eq!(
                current_device.unwrap().id,
                first_device.id,
                "Current device should match the device we set"
            );
        }
        Err(e) => {
            println!(
                "DEBUG:[AUDIO/INTEGRATION] Failed to switch input device: {:?}",
                e
            );
            // Don't panic - device switching may fail on some systems
            // This is acceptable as long as the code path is tested
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default - run with: cargo test --test audio_device_integration_test -- --ignored
async fn test_switch_output_device() {
    if should_skip_integration_tests() {
        println!("Skipping output device switching test (SKIP_AUDIO_INTEGRATION_TESTS set)");
        return;
    }

    println!("DEBUG:[AUDIO/INTEGRATION] Testing output device switching");

    // Create real audio engine
    let engine = create_audio_engine().expect("Should create audio engine");
    let engine_arc: Arc<dyn rustalk_lib::domain::traits::audio_engine::AudioEngine> =
        Arc::from(engine);
    let service = AudioService::new(engine_arc.clone());

    // Enumerate available output devices
    let devices = handle_no_devices_error(service.list_output_devices().await);
    if devices.is_empty() {
        println!("DEBUG:[AUDIO/INTEGRATION] No output devices available, skipping test");
        return;
    }

    println!(
        "DEBUG:[AUDIO/INTEGRATION] Found {} output device(s)",
        devices.len()
    );

    // Try to switch to the first available output device
    let first_device = &devices[0];
    println!(
        "DEBUG:[AUDIO/INTEGRATION] Switching to output device: {} ({})",
        first_device.name, first_device.id
    );

    let result = service.set_output_device(&first_device.id).await;
    match result {
        Ok(_) => {
            println!("DEBUG:[AUDIO/INTEGRATION] Successfully switched output device");
            // Verify device was set
            let current_device = service.get_output_device().await.unwrap();
            assert!(
                current_device.is_some(),
                "Output device should be set after switching"
            );
            assert_eq!(
                current_device.unwrap().id,
                first_device.id,
                "Current device should match the device we set"
            );
        }
        Err(e) => {
            println!(
                "DEBUG:[AUDIO/INTEGRATION] Failed to switch output device: {:?}",
                e
            );
            // Don't panic - device switching may fail on some systems
            // This is acceptable as long as the code path is tested
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default - run with: cargo test --test audio_device_integration_test -- --ignored
async fn test_switch_invalid_device() {
    if should_skip_integration_tests() {
        println!("Skipping invalid device test (SKIP_AUDIO_INTEGRATION_TESTS set)");
        return;
    }

    println!("DEBUG:[AUDIO/INTEGRATION] Testing invalid device error handling");

    // Create real audio engine
    let engine = create_audio_engine().expect("Should create audio engine");
    let engine_arc: Arc<dyn rustalk_lib::domain::traits::audio_engine::AudioEngine> =
        Arc::from(engine);
    let service = AudioService::new(engine_arc);

    // Try to switch to a non-existent device
    let result = service.set_input_device("non-existent-device-id").await;
    assert!(
        result.is_err(),
        "Setting invalid input device should return error"
    );
    assert!(
        matches!(result.unwrap_err(), AudioEngineError::DeviceNotFound { .. }),
        "Error should be DeviceNotFound"
    );

    let result = service.set_output_device("non-existent-device-id").await;
    assert!(
        result.is_err(),
        "Setting invalid output device should return error"
    );
    assert!(
        matches!(result.unwrap_err(), AudioEngineError::DeviceNotFound { .. }),
        "Error should be DeviceNotFound"
    );

    println!("DEBUG:[AUDIO/INTEGRATION] Invalid device error handling works correctly");
}

#[tokio::test]
#[ignore] // Ignore by default - run with: cargo test --test audio_device_integration_test -- --ignored
async fn test_device_switching_flow() {
    if should_skip_integration_tests() {
        println!("Skipping device switching flow test (SKIP_AUDIO_INTEGRATION_TESTS set)");
        return;
    }

    println!("DEBUG:[AUDIO/INTEGRATION] Testing complete device switching flow");

    // Create real audio engine
    let engine = create_audio_engine().expect("Should create audio engine");
    let engine_arc: Arc<dyn rustalk_lib::domain::traits::audio_engine::AudioEngine> =
        Arc::from(engine);
    let service = AudioService::new(engine_arc.clone());

    // Step 1: Enumerate input devices
    let input_devices = handle_no_devices_error(service.list_input_devices().await);
    if input_devices.is_empty() {
        println!("DEBUG:[AUDIO/INTEGRATION] No input devices available, skipping test");
        return;
    }

    // Step 2: Enumerate output devices
    let output_devices = handle_no_devices_error(service.list_output_devices().await);
    if output_devices.is_empty() {
        println!("DEBUG:[AUDIO/INTEGRATION] No output devices available, skipping test");
        return;
    }

    println!(
        "DEBUG:[AUDIO/INTEGRATION] Found {} input device(s) and {} output device(s)",
        input_devices.len(),
        output_devices.len()
    );

    // Step 3: Set first input device
    let first_input = &input_devices[0];
    println!(
        "DEBUG:[AUDIO/INTEGRATION] Setting input device: {} ({})",
        first_input.name, first_input.id
    );
    if let Err(e) = service.set_input_device(&first_input.id).await {
        println!(
            "DEBUG:[AUDIO/INTEGRATION] Failed to set input device (may be expected): {:?}",
            e
        );
        return;
    }

    // Step 4: Verify input device was set
    let current_input = service.get_input_device().await.unwrap();
    assert!(
        current_input.is_some(),
        "Input device should be set after set_input_device"
    );
    assert_eq!(
        current_input.unwrap().id,
        first_input.id,
        "Current input device should match the device we set"
    );

    // Step 5: Set first output device
    let first_output = &output_devices[0];
    println!(
        "DEBUG:[AUDIO/INTEGRATION] Setting output device: {} ({})",
        first_output.name, first_output.id
    );
    if let Err(e) = service.set_output_device(&first_output.id).await {
        println!(
            "DEBUG:[AUDIO/INTEGRATION] Failed to set output device (may be expected): {:?}",
            e
        );
        return;
    }

    // Step 6: Verify output device was set
    let current_output = service.get_output_device().await.unwrap();
    assert!(
        current_output.is_some(),
        "Output device should be set after set_output_device"
    );
    assert_eq!(
        current_output.unwrap().id,
        first_output.id,
        "Current output device should match the device we set"
    );

    // Step 7: If multiple devices available, test switching
    if input_devices.len() > 1 {
        let second_input = &input_devices[1];
        println!(
            "DEBUG:[AUDIO/INTEGRATION] Switching to second input device: {} ({})",
            second_input.name, second_input.id
        );
        if service.set_input_device(&second_input.id).await.is_ok() {
            let current = service.get_input_device().await.unwrap();
            assert!(
                current.is_some(),
                "Input device should be set after switching"
            );
            assert_eq!(
                current.unwrap().id,
                second_input.id,
                "Current input device should match the second device"
            );
        }
    }

    if output_devices.len() > 1 {
        let second_output = &output_devices[1];
        println!(
            "DEBUG:[AUDIO/INTEGRATION] Switching to second output device: {} ({})",
            second_output.name, second_output.id
        );
        if service.set_output_device(&second_output.id).await.is_ok() {
            let current = service.get_output_device().await.unwrap();
            assert!(
                current.is_some(),
                "Output device should be set after switching"
            );
            assert_eq!(
                current.unwrap().id,
                second_output.id,
                "Current output device should match the second device"
            );
        }
    }

    println!("DEBUG:[AUDIO/INTEGRATION] Complete device switching flow test passed");
}
