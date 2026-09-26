
int __cdecl FUN_006267e0(undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  HMODULE hModule;
  int iVar1;
  
  iVar1 = 0;
  if (DAT_006bec1c != (FARPROC)0x0) {
LAB_00626830:
    if (DAT_006bec20 != (FARPROC)0x0) {
      iVar1 = (*DAT_006bec20)();
    }
    if ((iVar1 != 0) && (DAT_006bec24 != (FARPROC)0x0)) {
      iVar1 = (*DAT_006bec24)(iVar1);
    }
    iVar1 = (*DAT_006bec1c)(iVar1,param_1,param_2,param_3);
    return iVar1;
  }
  hModule = LoadLibraryA("user32.dll");
  if (hModule != (HMODULE)0x0) {
    DAT_006bec1c = GetProcAddress(hModule,"MessageBoxA");
    if (DAT_006bec1c != (FARPROC)0x0) {
      DAT_006bec20 = GetProcAddress(hModule,"GetActiveWindow");
      DAT_006bec24 = GetProcAddress(hModule,"GetLastActivePopup");
      goto LAB_00626830;
    }
  }
  return 0;
}

