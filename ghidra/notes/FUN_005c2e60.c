
void __fastcall FUN_005c2e60(int param_1)

{
  ushort uVar1;
  HMODULE hModule;
  HRSRC hResInfo;
  HGLOBAL hResData;
  LPVOID pvVar2;
  undefined1 *puVar3;
  undefined1 *puVar4;
  int iVar5;
  
  hModule = (HMODULE)FUN_005fefd0(6);
  uVar1 = FUN_00596ad0();
  hResInfo = FindResourceA(hModule,(LPCSTR)(uVar1 + 0x1e & 0xffff),&DAT_0000012f);
  hResData = LoadResource(hModule,hResInfo);
  pvVar2 = LockResource(hResData);
  puVar3 = (undefined1 *)(param_1 + 0x1bd);
  iVar5 = 0x100;
  puVar4 = (undefined1 *)((int)pvVar2 + 2);
  do {
    puVar3[-1] = puVar4[-2];
    *puVar3 = puVar4[-1];
    puVar3[1] = *puVar4;
    puVar3[2] = 0x44;
    puVar3 = puVar3 + 4;
    iVar5 = iVar5 + -1;
    puVar4 = puVar4 + 3;
  } while (iVar5 != 0);
  FreeResource(hResData);
  return;
}

