
undefined4 __cdecl FUN_005f41b0(LPCSTR param_1,LPCSTR param_2)

{
  LSTATUS LVar1;
  HKEY local_10;
  DWORD local_c;
  undefined4 local_8;
  DWORD local_4;
  
  local_10 = (HKEY)0x0;
  local_8 = 0;
  local_4 = 4;
  local_c = 0;
  LVar1 = RegOpenKeyExA((HKEY)0x80000002,param_1,0,0x20019,&local_10);
  if (LVar1 == 0) {
    LVar1 = RegQueryValueExA(local_10,param_2,(LPDWORD)0x0,&local_4,(LPBYTE)0x0,&local_c);
    if ((LVar1 == 0) && (local_c == 4)) {
      local_8 = 0;
      RegQueryValueExA(local_10,param_2,(LPDWORD)0x0,&local_4,(LPBYTE)&local_8,&local_c);
    }
    RegCloseKey(local_10);
  }
  return local_8;
}

