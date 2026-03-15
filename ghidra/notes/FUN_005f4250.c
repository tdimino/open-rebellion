
bool __cdecl FUN_005f4250(HKEY param_1,LPCSTR param_2,LPCSTR param_3)

{
  HKEY hKey;
  LSTATUS LVar1;
  bool bVar2;
  
  bVar2 = false;
  hKey = (HKEY)FUN_005f4090((byte *)param_1);
  param_1 = (HKEY)0x0;
  if ((hKey != (HKEY)0x0) && (LVar1 = RegOpenKeyExA(hKey,param_2,0,0x20006,&param_1), LVar1 == 0)) {
    LVar1 = RegSetValueExA(param_1,param_3,0,4,&stack0x00000010,4);
    bVar2 = LVar1 == 0;
    RegCloseKey(param_1);
  }
  return bVar2;
}

