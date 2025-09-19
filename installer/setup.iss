#define AppName        "Avrix Launcher"
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
AppId={{E6E1BBA2-4D5A-44C3-9A3B-08F9E8D5A16F}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
; On n'installe PAS sous Program Files par défaut : on vise le dossier du jeu
DefaultDirName={code:GetDefaultPZDir}
DisableProgramGroupPage=yes
CreateAppDir=yes
AllowUNCPath=no
UsePreviousAppDir=no
OutputBaseFilename={#AppName}-Setup-{#AppVersion}
Compression=lzma2
SolidCompression=yes
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
WizardStyle=modern
SetupIconFile=payload\assets\icon.ico

[Languages]
Name: "fr"; MessagesFile: "compiler:Languages\French.isl"

[Tasks]
Name: "desktopicon"; Description: "Créer un raccourci sur le Bureau"; GroupDescription: "Raccourcis:"; Flags: unchecked

[Files]
; ----- EXE du launcher (dans {app}\Avrix)
Source: "payload\build\AvrixLauncher.exe"; DestDir: "{app}\{#AppFolderName}"; Flags: ignoreversion

[Icons]
; Raccourci (facultatif) sur le Bureau
Name: "{commondesktop}\{#AppName}"; Filename: "{app}\{#AppFolderName}\{#AppExeName}"; WorkingDir: "{app}\{#AppFolderName}"; Tasks: desktopicon

[Run]
; Proposer de lancer à la fin
Filename: "{app}\{#AppFolderName}\{#AppExeName}"; Description: "Lancer {#AppName}"; Flags: nowait postinstall skipifsilent

[Registry]
; Enregistre le répertoire d'installation pour que l'app (ou d'autres installateurs) puisse le retrouver
Root: HKCU; Subkey: "Software\Avrix"; ValueType: string; ValueName: "InstallDir"; ValueData: "{app}\{#AppFolderName}"; Flags: uninsdeletevalue

[Code]
procedure InitializeWizard;
var
  Lbl: TNewStaticText;
begin
  // Ajoute une note sur la page de sélection du dossier pour recommander l'installation à la racine du jeu
  Lbl := TNewStaticText.Create(WizardForm);
  Lbl.Parent := WizardForm.SelectDirPage;
  Lbl.Left := 0;
  Lbl.Top := WizardForm.DirEdit.Top + WizardForm.DirEdit.Height + ScaleY(8);
  Lbl.Width := WizardForm.SelectDirPage.ClientWidth;
  Lbl.AutoSize := False;
  Lbl.WordWrap := True;
  Lbl.Caption :=
    'Recommandé : installez Avrix à la racine de Project Zomboid'
end;

function GuessSteamPath(var SteamPath: string): Boolean;
begin
  // Steam 64-bit (WOW6432Node est courant pour Steam)
  Result := RegQueryStringValue(HKLM, 'SOFTWARE\WOW6432Node\Valve\Steam', 'InstallPath', SteamPath);
  if (not Result) then
    Result := RegQueryStringValue(HKCU, 'Software\Valve\Steam', 'SteamPath', SteamPath);
end;

function GetDefaultPZDir(Param: string): string;
var
  SteamPath: string;
  Candidate: string;
begin
  if GuessSteamPath(SteamPath) then begin
    Candidate := SteamPath + '\steamapps\common\ProjectZomboid';
    if DirExists(Candidate) then begin
      Result := Candidate;
      exit;
    end;
  end;
  // Fallback : chemin Steam standard
  Result := 'C:\Program Files (x86)\Steam\steamapps\common\ProjectZomboid';
end;

// Vérification simple : avertir si le dossier choisi ne ressemble pas à PZ
function NextButtonClick(CurPageID: Integer): Boolean;
var
  exe1, exe2: string;
begin
  Result := True;
  if CurPageID = wpSelectDir then begin
    exe1 := AddBackslash(WizardDirValue) + 'ProjectZomboid64.exe';
    exe2 := AddBackslash(WizardDirValue) + 'ProjectZomboid64.bat';
    if (not FileExists(exe1)) and (not FileExists(exe2)) then begin
      if MsgBox('Le dossier sélectionné ne semble pas être celui de Project Zomboid.'#13#10+
                'Continuer quand même ?', mbConfirmation, MB_YESNO) = IDNO then
      begin
        Result := False;
      end;
    end;
  end;
end;
