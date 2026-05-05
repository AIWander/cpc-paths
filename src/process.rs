//! Process-lifetime helpers for CPC MCP servers.

/// Register this process in a Windows Job Object with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`.
///
/// When the parent process (e.g. Claude Desktop) closes the handle to this process,
/// the OS guarantees that this process **and all its descendants** are terminated.
/// This prevents orphan MCP server processes from surviving across restarts.
///
/// On non-Windows platforms this is a no-op.
#[cfg(windows)]
pub fn ensure_kill_on_parent_death() -> anyhow::Result<()> {
    use windows::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject,
        JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    };
    use windows::Win32::System::Threading::GetCurrentProcess;

    unsafe {
        let job = CreateJobObjectW(None, None)?;
        let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            &info as *const _ as *const _,
            std::mem::size_of_val(&info) as u32,
        )?;
        AssignProcessToJobObject(job, GetCurrentProcess())?;
        // HANDLE is Copy with no Drop — stays open until process exit.
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn ensure_kill_on_parent_death() -> anyhow::Result<()> {
    Ok(())
}
