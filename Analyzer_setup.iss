[Setup]
; Basic Information
AppName=File Path Analyzer
AppVersion=1.0
DefaultDirName={commonpf}\File_Path_Analyzer
DefaultGroupName=Path Searcher
AllowNoIcons=yes
OutputDir=output
OutputBaseFilename=PathAnalyzerInstaller
Compression=lzma
SolidCompression=yes
UninstallDisplayIcon={app}\file_path_analyzer.exe

[Files]
; Files to Install
Source: "D:\RustProjects\file_path_analyzer\target\release\file_path_analyzer.exe"; DestDir: "{app}"; Flags: ignoreversion
[UninstallDelete]
; Delete Custom Files on Uninstall
Type: files; Name: "{app}\file_path_analyzer.exe"

[Run]
; Run Application After Installation
Filename: "{app}\file_path_analyzer.exe"; Description: "Launch Path Analyzer"; Flags: nowait postinstall skipifsilent

[Registry]
; Add Uninstall Information to Control Panel
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\PathSearcher"; Flags: uninsdeletekeyifempty
Root: HKCU; Subkey: "Software\PathSearcher"; ValueType: string; ValueName: "InstallPath"; ValueData: "{app}"; Flags: uninsdeletevalue

; Add Right-Click Context Menu Entry for Right-Click on a Folder
Root: HKCR; Subkey: "Directory\shell\PathSearcher"; ValueType: string; ValueData: "Run Path Analyzer"
Root: HKCR; Subkey: "Directory\shell\PathSearcher\command"; ValueType: string; ValueData: """cmd.exe"" /C """"{app}\file_path_analyzer.exe"" ""%1"""""

; Add Right-Click Context Menu Entry for Right-Click Inside a Directory
Root: HKCR; Subkey: "Directory\Background\shell\PathSearcher"; ValueType: string; ValueData: "Run Path Analyzer"
Root: HKCR; Subkey: "Directory\Background\shell\PathSearcher\command"; ValueType: string; ValueData: """cmd.exe"" /C """"{app}\file_path_analyzer.exe"" ""%V"""""

[Tasks]
; No optional tasks (no start menu shortcut or desktop icon)
