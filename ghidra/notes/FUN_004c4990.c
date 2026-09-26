
ushort __cdecl FUN_004c4990(HGLOBAL param_1)

{
  ushort uVar1;
  HMODULE hModule;
  HRSRC hResInfo;
  int iVar2;
  ushort uVar3;
  ushort *puVar4;
  
  puVar4 = (ushort *)0x0;
  uVar3 = 0;
  hModule = (HMODULE)FUN_005fefd0(9);
  hResInfo = FindResourceA(hModule,(LPCSTR)((uint)param_1 & 0xffff),(LPCSTR)0xa);
  if ((hResInfo != (HRSRC)0x0) &&
     (param_1 = LoadResource(hModule,hResInfo), param_1 != (HGLOBAL)0x0)) {
    puVar4 = LockResource(param_1);
  }
  if (puVar4 != (ushort *)0x0) {
    uVar1 = *puVar4;
    if (uVar1 != 0) {
      if (1 < uVar1) {
        iVar2 = FUN_0041cd80((uint)uVar1);
        uVar3 = puVar4[iVar2 + 1];
        FreeResource(param_1);
        return uVar3;
      }
      uVar3 = puVar4[1];
    }
    FreeResource(param_1);
  }
  return uVar3;
}

