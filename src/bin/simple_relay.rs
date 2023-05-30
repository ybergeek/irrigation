use irrigation::establish_pin1;
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
    loop{
        pin1.set_low();
        thread::sleep(Duration::from_millis(50));
        pin1.set_high();
        
    }
    
    
}