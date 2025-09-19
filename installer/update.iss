#define AppName        "Avrix Launcher Updater"
#define EnvVer         GetEnv("NEW_VERSION")
#if EnvVer != ""
  #define AppVersion   EnvVer
#else
  #define AppVersion   "0.0.0"
#endif
#define AppPublisher   "Avrix"
#define AppFolderName  "Avrix"
#define AppExeName     "AvrixLauncher.exe"

[Setup]
AppId={{30E3A2D1-8F7C-4D61-9A9E-6B7E0A4DA3E1}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
; Non-interactive for directory selection
DisableDirPage=yes
DisableProgramGroupPage=yes
CreateAppDir=yes
AllowUNCPath=no
UsePreviousAppDir=no
Uninstallable=no
DefaultDirName={code:GetDefaultPZDir}
CloseApplications=force
OutputBaseFilename=Avrix-Update-{#AppVersion}
Compression=lzma2
SolidCompression=yes
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
WizardStyle=modern
SetupIconFile=payload\assets\icon.ico

[Languages]
Name: "fr"; MessagesFile: "compiler:Languages\French.isl"

[Files]
; Update only the launcher binary in the Avrix subfolder under the previously installed root
Source: "payload\build\AvrixLauncher.exe"; DestDir: "{app}\{#AppFolderName}"; Flags: ignoreversion

[Registry]
; Keep InstallDir registry value aligned for other tools
Root: HKCU; Subkey: "Software\Avrix"; ValueType: string; ValueName: "InstallDir"; ValueData: "{app}\{#AppFolderName}"; Flags: uninsdeletevalue

[Code]
function GetPrevInstallDir(var Path: string): Boolean;
var
  v: string;
begin
  Result := False;
  // Try NSIS key written by Tauri installer (current user)
  if RegQueryStringValue(HKCU, 'Software\\Avrix\\Avrix Launcher', '', v) then begin
    Path := v;
    Result := True;
    exit;
  end;
  // Try NSIS key (per-machine)
  if RegQueryStringValue(HKLM, 'Software\\Avrix\\Avrix Launcher', '', v) then begin
    Path := v;
    Result := True;
    exit;
  end;
  // Fallback: legacy Inno key storing full Avrix install dir
  if RegQueryStringValue(HKCU, 'Software\\Avrix', 'InstallDir', v) then begin
    Path := v;
    Result := True;
    exit;
  end;
end;

function GuessSteamPath(var SteamPath: string): Boolean;
begin
  Result := RegQueryStringValue(HKLM, 'SOFTWARE\\WOW6432Node\\Valve\\Steam', 'InstallPath', SteamPath);
  if (not Result) then
    Result := RegQueryStringValue(HKCU, 'Software\\Valve\\Steam', 'SteamPath', SteamPath);
end;

function GetDefaultPZDir(Param: string): string;
var
  prev, parent, SteamPath, Candidate: string;
begin
  // Prefer the parent of the previous Avrix install dir if available
  if GetPrevInstallDir(prev) then begin
    parent := ExtractFileDir(prev);
    if DirExists(parent) then begin
      Result := parent;
      exit;
    end;
  end;
  // Fallback to Steam default
  if GuessSteamPath(SteamPath) then begin
    Candidate := SteamPath + '\\steamapps\\common\\ProjectZomboid';
    if DirExists(Candidate) then begin
      Result := Candidate;
      exit;
    end;
  end;
  Result := 'C:\\Program Files (x86)\\Steam\\steamapps\\common\\ProjectZomboid';
end;

procedure InitializeWizard;
begin
  // Ensure the app root is set based on previous install
  WizardForm.DirEdit.Text := GetDefaultPZDir('');
end;
