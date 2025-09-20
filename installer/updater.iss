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
; IMPORTANT: Use the SAME AppId as the main installer so Windows treats this as the same product
AppId={{E6E1BBA2-4D5A-44C3-9A3B-08F9E8D5A16F}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
; We don't create {app}, we push directly to the resolved installation dir
CreateAppDir=no
AllowUNCPath=no
UsePreviousAppDir=no
; Render a small, quiet wizard – intended to run silently via CI delivery
DisableDirPage=yes
DisableProgramGroupPage=yes
DisableReadyMemo=yes
DisableReadyPage=yes
DisableFinishedPage=yes
WizardStyle=modern
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
Compression=lzma2
SolidCompression=yes
; Ensure we can write under Program Files / Steam paths and auto-close running launcher
PrivilegesRequired=admin
CloseApplications=yes
CloseApplicationsFilter={#AppExeName}
; Output
OutputDir=Output
OutputBaseFilename=Avrix-Update-{#AppVersion}
SetupIconFile=payload\assets\icon.ico

[Languages]
Name: "fr"; MessagesFile: "compiler:Languages\French.isl"

[Files]
; Replace only the launcher EXE in the existing install folder
Source: "payload\build\AvrixLauncher.exe"; DestDir: "{code:GetInstalledAvrixDir}"; Flags: ignoreversion

[Code]
function GetInstalledAvrixDir(Param: string): string;
var
  InstallDir: string;
begin
  { Read the same registry key written by setup.iss: HKCU\Software\Avrix\Avrix Launcher, default value }
  if RegQueryStringValue(HKCU, 'Software\Avrix\Avrix Launcher', '', InstallDir) then
  begin
    Result := InstallDir;
  end
  else
  begin
    { If not found, fail fast with a clear message. The updater is not meant to pick a path. }
    MsgBox('Impossible de trouver le dossier d\'installation d\'Avrix dans le registre.'#13#10 +
           'Clé attendue: HKCU\\Software\\Avrix\\Avrix Launcher (valeur par défaut).', mbCriticalError, MB_OK);
    Result := '';
  end;
end;

function InitializeSetup(): Boolean;
begin
  { Abort if we cannot resolve the install directory }
  Result := GetInstalledAvrixDir('') <> '';
end;
