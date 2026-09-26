
undefined4 __cdecl FUN_0048b460(int param_1,void *param_2)

{
  HMODULE hModule;
  HRSRC pHVar1;
  HGLOBAL pvVar2;
  ushort *puVar3;
  byte *pbVar4;
  void *this;
  uint uVar5;
  int iVar6;
  void *this_00;
  LPCSTR lpName;
  undefined4 local_24;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063513b;
  pvStack_c = ExceptionList;
  uVar5 = 0;
  local_24 = 0;
  lpName = (LPCSTR)((-(uint)(param_1 != 1) & 0x800) + 0x6000);
  ExceptionList = &pvStack_c;
  hModule = (HMODULE)FUN_005fefd0(2);
  pHVar1 = FindResourceA(hModule,lpName,(LPCSTR)0xa);
  pvVar2 = LoadResource(hModule,pHVar1);
  puVar3 = LockResource(pvVar2);
  if (puVar3 != (ushort *)0x0) {
    uVar5 = (uint)*puVar3;
  }
  FreeResource(pvVar2);
  iVar6 = 1;
  if (uVar5 != 0) {
    local_24 = 1;
    param_1 = 0x10;
    do {
      pHVar1 = FindResourceA(hModule,(LPCSTR)((uint)(lpName + iVar6 * 3) & 0xffff),(LPCSTR)0xa);
      pvVar2 = LoadResource(hModule,pHVar1);
      if (pvVar2 != (HGLOBAL)0x0) {
        pbVar4 = LockResource(pvVar2);
        if (puVar3 != (ushort *)0x0) {
          this = (void *)FUN_00618b70(0x20);
          this_00 = (void *)0x0;
          uStack_4 = 0;
          if (this != (void *)0x0) {
            this_00 = FUN_005f5f10(this,0);
          }
          uStack_4 = 0xffffffff;
          *(uint *)((int)this_00 + 0x1c) = (*pbVar4 & 0xf) + param_1;
          FUN_005f5600(this_00,(uint)*(ushort *)(pbVar4 + 2));
          FUN_005f5440(param_2,this_00);
        }
        FreeResource(pvVar2);
      }
      iVar6 = iVar6 + 1;
      param_1 = param_1 + 0x10;
    } while (iVar6 <= (int)uVar5);
  }
  ExceptionList = pvStack_c;
  return local_24;
}

