#![no_std]
#![no_main]

//extern crate libc;

use gdbstub::stub::state_machine::GdbStubStateMachine;
use gdbstub::stub::MultiThreadStopReason;
use gdbstub::stub::{DisconnectReason, GdbStubBuilder, GdbStubError};

mod conn;
mod gdb;
mod print_str;

use crate::print_str::print_str;

extern "C" { fn deadEnd( )->!;}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe
    {
        deadEnd();
    }
    loop {}
}



#[no_mangle]
pub extern "C"
fn rn_gdb_main() -> Result<(), i32>
{
    let mut target = gdb::DummyTarget::new();

    let conn = match conn::CdcConnection::new(0)
    {
        Ok(c) => c,
        Err(e) => {
            print_str("could not start CDC");
            print_str(e);
            return Err(-1);
        }
    };

    let mut buf = [0; 4096];
    let gdb = GdbStubBuilder::new(conn)
        .with_packet_buffer(&mut buf)
        .build()
        .map_err(|_| 1)?;

    print_str("Starting GDB session...");

    let mut gdb = gdb.run_state_machine(&mut target).map_err(|_| 1)?;

    let res = loop
    {
        gdb = match gdb
        {
            GdbStubStateMachine::Idle(mut gdb) =>
            {
                let byte = gdb.borrow_conn().read().map_err(|_| 1)?;
                match gdb.incoming_data(&mut target, byte) {
                    Ok(gdb) => gdb,
                    Err(e) => break Err(e),
                }
            }
            GdbStubStateMachine::Running(gdb) =>
            {
                match gdb.report_stop(&mut target, MultiThreadStopReason::DoneStep) {
                    Ok(gdb) => gdb,
                    Err(e) => break Err(e),
                }
            }
            GdbStubStateMachine::CtrlCInterrupt(gdb) =>
            {
                match gdb.interrupt_handled(&mut target, None::<MultiThreadStopReason<u32>>)
                {
                    Ok(gdb) => gdb,
                    Err(e) => break Err(e),
                }
            }
            GdbStubStateMachine::Disconnected(gdb) => break Ok(gdb.get_reason()),
        }
    };

    match res
    {
        Ok(disconnect_reason) => match disconnect_reason {
            DisconnectReason::Disconnect => print_str("GDB Disconnected"),
            DisconnectReason::TargetExited(_) => print_str("Target exited"),
            DisconnectReason::TargetTerminated(_) => print_str("Target halted"),
            DisconnectReason::Kill => print_str("GDB sent a kill command"),
        },
        Err(GdbStubError::TargetError(_e)) => {
            print_str("Target raised a fatal error");
        }
        Err(_e) => {
            print_str("gdbstub internal error");
        }
    }

    Ok(())
}
