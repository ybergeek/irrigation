use irrigation::{establish_pin1,establish_pin2,establish_pin3,establish_pin4};
use std::error::Error;
use std::thread;
use std::time::Duration;
//use tokio::signal;
/* 
async fn shutdown_signal() { // (1)
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    establish_pin1().set_high();
} */

fn main()-> Result<(), Box<dyn Error>>{
    let mut pin1 = establish_pin1();
    let mut pin2 = establish_pin2();
    let mut pin3 = establish_pin3();
    let mut pin4 = establish_pin4();
   
    loop{
        pin1.set_low();
        thread::sleep(Duration::from_millis(500));
        pin1.set_high();
        pin2.set_low();
        thread::sleep(Duration::from_millis(500));
        pin2.set_high();
        pin3.set_low();
        thread::sleep(Duration::from_millis(500));
        pin3.set_high();
        pin4.set_low();
        thread::sleep(Duration::from_millis(500));
        pin4.set_high();
        
    }
    
    
}