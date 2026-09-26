
undefined4 __cdecl FUN_00619ad0(LPCSTR param_1)

{
  char cVar1;
  BOOL BVar2;
  DWORD DVar3;
  uint uVar4;
  undefined *puVar5;
  CHAR CStack_10c;
  undefined1 uStack_10b;
  undefined1 uStack_10a;
  undefined1 uStack_109;
  uint auStack_108 [66];
  
  BVar2 = SetCurrentDirectoryA(param_1);
  if (BVar2 != 0) {
    DVar3 = GetCurrentDirectoryA(0x105,(LPSTR)auStack_108);
    if (DVar3 != 0) {
      cVar1 = (char)auStack_108[0];
      if (((cVar1 == '\\') || (cVar1 == '/')) && (cVar1 == (char)(auStack_108[0] >> 8))) {
        return 0;
      }
      CStack_10c = '=';
      uVar4 = FUN_00620e00(auStack_108[0] & 0xff);
      uStack_10b = (undefined1)uVar4;
      uStack_10a = 0x3a;
      uStack_109 = 0;
      BVar2 = SetEnvironmentVariableA(&CStack_10c,(LPCSTR)auStack_108);
      if (BVar2 != 0) {
        return 0;
      }
    }
  }
  puVar5 = (undefined *)GetLastError();
  FUN_0061c1d0(puVar5);
  return 0xffffffff;
}

