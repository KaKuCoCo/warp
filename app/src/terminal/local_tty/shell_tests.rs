use super::*;

#[test]
fn test_program_invalid_bash() {
    // This test assumes there is no bash binary at /some/weird/path/bash.
    let shell_path = "/some/weird/path/bash".to_owned();
    assert!(supported_shell_path_and_type(&shell_path).is_none());
}

#[test]
fn test_program_invalid_zsh() {
    // This test assumes there is no bash zsh at /some/weird/path/bash.
    let shell_path = "/some/weird/path/zsh".to_owned();
    assert!(supported_shell_path_and_type(&shell_path).is_none());
}

#[test]
fn test_program_unknown_shell() {
    let shell_path = "/some/weird/path/wtfsh".to_owned();
    assert!(supported_shell_path_and_type(&shell_path).is_none());
}

#[test]
fn test_trim_wsl_err_from_output() {
    assert_eq!(
        take_until_utf16_crlf(b"/bin/bash\n".to_vec()),
        b"/bin/bash\n".to_vec()
    );
    assert_eq!(
        take_until_utf16_crlf(b"/bin/bash\n\r\0\n\0W\0A\0R\0N\0I\0N\0G\0".to_vec()),
        b"/bin/bash\n".to_vec()
    );
}

#[cfg(windows)]
#[test]
fn test_powershell_args_bypass_execution_policy_before_command() {
    let args = arguments_for_session_spawning_command("pwsh.exe", ShellType::PowerShell);
    let args = args
        .iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    let execution_policy_index = args
        .iter()
        .position(|arg| arg == "-ExecutionPolicy")
        .expect("PowerShell should set a process-scoped execution policy");
    assert_eq!(
        args.get(execution_policy_index + 1),
        Some(&"Bypass".to_owned())
    );

    let command_index = args
        .iter()
        .position(|arg| arg == "-Command")
        .expect("PowerShell should still receive the init command");
    assert!(
        execution_policy_index < command_index,
        "-ExecutionPolicy must be before -Command because PowerShell treats everything after -Command as command text"
    );
}
