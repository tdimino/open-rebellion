
undefined4 __cdecl FUN_00442790(uint param_1,int param_2,undefined2 *param_3)

{
  HMODULE hModule;
  HRSRC hResInfo;
  undefined2 *puVar1;
  undefined4 uVar2;
  HGLOBAL hResData;
  
  hModule = (HMODULE)FUN_006037f0(param_2);
  uVar2 = 0;
  hResData = (HGLOBAL)0x0;
  if (hModule != (HMODULE)0x0) {
    hResInfo = FindResourceA(hModule,(LPCSTR)(param_1 & 0xffff),(LPCSTR)0xa);
    if (hResInfo == (HRSRC)0x0) {
      uVar2 = 1;
    }
    else {
      hResData = LoadResource(hModule,hResInfo);
    }
  }
  if ((hResData != (HGLOBAL)0x0) && (param_3 != (undefined2 *)0x0)) {
    puVar1 = LockResource(hResData);
    *param_3 = *puVar1;
    param_3[1] = puVar1[1];
    param_3[2] = puVar1[2];
    param_3[3] = puVar1[3];
    param_3[4] = puVar1[4];
    param_3[5] = puVar1[5];
    param_3[6] = puVar1[6];
    param_3[7] = puVar1[7];
    param_3[8] = puVar1[8];
    param_3[9] = puVar1[9];
    param_3[10] = puVar1[10];
    param_3[0xb] = puVar1[0xb];
    param_3[0xc] = puVar1[0xc];
  }
  FreeResource(hResData);
  return uVar2;
}

