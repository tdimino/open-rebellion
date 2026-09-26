
void __fastcall FUN_00411b80(int param_1)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  HMODULE hModule;
  HRSRC hResInfo;
  void *pvVar4;
  int iVar5;
  undefined2 extraout_var;
  int iVar6;
  void *pvVar7;
  ushort *puStack_28;
  ushort *local_24;
  HGLOBAL pvStack_20;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a736;
  pvStack_c = ExceptionList;
  uVar1 = *(uint *)(param_1 + 0x14);
  iVar2 = *(int *)(param_1 + 0x10);
  local_24 = (ushort *)0x0;
  ExceptionList = &pvStack_c;
  hModule = (HMODULE)FUN_005fefd0(uVar1 >> 0x10);
  hResInfo = FindResourceA(hModule,(LPCSTR)(uVar1 & 0xffff),(LPCSTR)0xa);
  if ((hResInfo != (HRSRC)0x0) &&
     (pvStack_20 = LoadResource(hModule,hResInfo), pvStack_20 != (HGLOBAL)0x0)) {
    local_24 = LockResource(pvStack_20);
  }
  if (local_24 != (ushort *)0x0) {
    iVar5 = *(int *)(param_1 + 0x4c);
    iVar6 = 0;
    *(uint *)(param_1 + 0x48) = (uint)*local_24;
    puStack_28 = local_24;
    if (0 < iVar5) {
      do {
        puStack_28 = puStack_28 + 1;
        pvVar4 = (void *)FUN_00618b70(0x28);
        pvVar7 = (void *)0x0;
        uStack_4 = 0;
        if (pvVar4 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          pvVar4 = FUN_005fbd20(pvVar4,hModule,CONCAT22(extraout_var,*puStack_28),2);
        }
        uStack_4 = 0xffffffff;
        *(void **)(*(int *)(param_1 + 0x50) + iVar6 * 4) = pvVar4;
        piVar3 = *(int **)(*(int *)(iVar2 + 0x20) + iVar6 * 4);
        if (piVar3 == (int *)0x0) {
          pvVar4 = (void *)FUN_00618b70(0x38);
          uStack_4 = 1;
          if (pvVar4 != (void *)0x0) {
            pvVar7 = FUN_0041ca70(pvVar4,*(undefined4 *)(*(int *)(param_1 + 0x50) + iVar6 * 4));
          }
          uStack_4 = 0xffffffff;
          (**(code **)(**(int **)(param_1 + 0xc) + 0x2c))(pvVar7);
          (**(code **)(**(int **)(param_1 + 0x10) + 0x10))(iVar6,pvVar7);
        }
        else {
          (**(code **)(*piVar3 + 8))(*(undefined4 *)(*(int *)(param_1 + 0x50) + iVar6 * 4));
        }
        iVar5 = *(int *)(param_1 + 0x4c);
        iVar6 = iVar6 + 1;
      } while (iVar6 < iVar5);
    }
    DAT_006b12b0 = 0;
    FUN_0041d690(CONCAT22((short)((uint)iVar5 >> 0x10),local_24[2]),0,0);
    FUN_0041d690(0x2d37,0,0);
    FreeResource(pvStack_20);
  }
  ExceptionList = pvStack_c;
  return;
}

