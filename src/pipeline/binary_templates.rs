use std::path::PathBuf;

/// Get the bash script for the chosen binary
/// node_path: The path of the node modules
///
pub fn get_bash_script(node_path: Vec<String>, name_of_package: &str, package_json_binary_path:
&str) -> String {

    let node_path = node_path.join(":").replace("\\", "/");
    let path_to_bin = PathBuf::from(name_of_package).join(package_json_binary_path);
    let path_to_bin = path_to_bin.to_str().unwrap().replace("\\", "/");

    remove_indentation(&format!(r#"
    #!/bin/sh

    basedir=$(dirname "$(echo "$0" | sed -e 's,\\,/,g')")

    case `uname` in
        *CYGWIN*) basedir=`cygpath -w "$basedir"`;;
    esac

        if [ -z "$NODE_PATH" ]; then
          export NODE_PATH="{node_path}"
        else
          export NODE_PATH="{node_path}:$NODE_PATH"
        fi
        if [ -x "$basedir/node" ]; then
          exec "$basedir/node"  "$basedir/../{path_to_bin}" "$@"
        else
          exec node  "$basedir/../{path_to_bin}" "$@"
        fi
    "#, ))
}

fn remove_indentation(s: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        result.push_str(trimmed_line);
        result.push('\n');
    }
    result
}


pub fn get_cmd_script(node_path: Vec<String>, name_of_package: &str,
                      package_json_binary_path: &str) -> String {
    let node_path = node_path.join(";");
    let path_to_bin = PathBuf::from(name_of_package).join(package_json_binary_path);
    let path_to_bin = path_to_bin.into_os_string().into_string().unwrap().replace("/", "\\");


    remove_indentation(&format!(r#"
        @SETLOCAL
    @IF NOT DEFINED NODE_PATH (
      @SET "NODE_PATH={node_path}"
    ) ELSE (
      @SET "NODE_PATH={node_path};%NODE_PATH%"
    )
    @IF EXIST "%~dp0\node.exe" (
      "%~dp0\node.exe"  "%~dp0\..\{path_to_bin}" %*
    ) ELSE (
      @SET PATHEXT=%PATHEXT:;.JS;=;%
      node  "%~dp0\..\{path_to_bin}" %*
    )
    "#))
}


pub fn get_pwsh_script(node_path_orig: Vec<String>, name_of_package: &str,
                       package_json_binary_path: &str) -> String {
    let node_path = node_path_orig.join(";").replace("\\", "/");
    let old_node_path = node_path_orig.join(":").replace("/", "\\");
    let path_to_bin = PathBuf::from(name_of_package).join(package_json_binary_path);
    let path_to_bin = path_to_bin.into_os_string().into_string().unwrap();

    remove_indentation(&format!(r#"
#!/usr/bin/env pwsh
$basedir=Split-Path $MyInvocation.MyCommand.Definition -Parent

$exe=""
$pathsep=":"
$env_node_path=$env:NODE_PATH
$new_node_path="{old_node_path}"
if ($PSVersionTable.PSVersion -lt "6.0" -or $IsWindows) {{
  $exe=".exe"
  $pathsep=";"
}} else {{
  $new_node_path="{node_path}"
}}
if ([string]::IsNullOrEmpty($env_node_path)) {{
  $env:NODE_PATH=$new_node_path
}} else {{
  $env:NODE_PATH="$new_node_path$pathsep$env_node_path"
}}

$ret=0
if (Test-Path "$basedir/node$exe") {{
  if ($MyInvocation.ExpectingInput) {{
    $input | & "$basedir/node$exe"  "$basedir/../{path_to_bin}" $args
  }} else {{
    & "$basedir/node$exe"  "$basedir/../{path_to_bin}" $args
  }}
  $ret=$LASTEXITCODE
}} else {{
  if ($MyInvocation.ExpectingInput) {{
    $input | & "node$exe"  "$basedir/../{path_to_bin}" $args
  }} else {{
    & "node$exe"  "$basedir/../{path_to_bin}" $args
  }}
  $ret=$LASTEXITCODE
}}
$env:NODE_PATH=$env_node_path
exit $ret
    "#))
}