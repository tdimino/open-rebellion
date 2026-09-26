
void __fastcall FUN_00411f00(int *param_1)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  HMODULE hModule;
  HRSRC hResInfo;
  void *pvVar4;
  undefined2 extraout_var;
  int iVar5;
  void *pvVar6;
  undefined2 *puStack_24;
  HGLOBAL pvStack_18;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a796;
  pvStack_c = ExceptionList;
  puStack_24 = (undefined2 *)0x0;
  uVar1 = param_1[5];
  iVar2 = param_1[4];
  ExceptionList = &pvStack_c;
  hModule = (HMODULE)FUN_005fefd0(uVar1 >> 0x10);
  hResInfo = FindResourceA(hModule,(LPCSTR)(uVar1 & 0xffff),(LPCSTR)0xa);
  if ((hResInfo != (HRSRC)0x0) &&
     (pvStack_18 = LoadResource(hModule,hResInfo), pvStack_18 != (HGLOBAL)0x0)) {
    puStack_24 = LockResource(pvStack_18);
  }
  if (puStack_24 == (undefined2 *)0x0) {
    (**(code **)(*param_1 + 0x20))();
    ExceptionList = pvStack_c;
    return;
  }
  iVar5 = 0;
  if (0 < param_1[0x12]) {
    do {
      pvVar4 = (void *)FUN_00618b70(0x28);
      pvVar6 = (void *)0x0;
      uStack_4 = 0;
      if (pvVar4 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_005fbd20(pvVar4,hModule,CONCAT22(extraout_var,*puStack_24),2);
      }
      uStack_4 = 0xffffffff;
      *(void **)(param_1[0x13] + iVar5 * 4) = pvVar4;
      piVar3 = *(int **)(*(int *)(iVar2 + 0x20) + iVar5 * 4);
      if (piVar3 == (int *)0x0) {
        pvVar4 = (void *)FUN_00618b70(0x38);
        uStack_4 = 1;
        if (pvVar4 != (void *)0x0) {
          pvVar6 = FUN_0041ca70(pvVar4,*(undefined4 *)(param_1[0x13] + iVar5 * 4));
        }
        uStack_4 = 0xffffffff;
        (**(code **)(*(int *)param_1[3] + 0x2c))(pvVar6);
        (**(code **)(*(int *)param_1[4] + 0x10))(iVar5,pvVar6);
      }
      else {
        (**(code **)(*piVar3 + 8))(*(undefined4 *)(param_1[0x13] + iVar5 * 4));
      }
      iVar5 = iVar5 + 1;
      puStack_24 = puStack_24 + 1;
    } while (iVar5 < param_1[0x12]);
  }
  FreeResource(pvStack_18);
  ExceptionList = pvStack_c;
  return;
}

