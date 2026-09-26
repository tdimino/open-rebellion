
void __fastcall FUN_00412680(void *param_1)

{
  uint uVar1;
  int *piVar2;
  HMODULE hModule;
  HRSRC hResInfo;
  HGLOBAL hResData;
  void *pvVar3;
  undefined2 extraout_var;
  int iVar4;
  void *pvVar5;
  short *psStack_2c;
  short *local_28;
  undefined4 uStack_24;
  HMODULE local_20;
  int local_1c;
  void *pvStack_18;
  undefined2 local_14;
  ushort uStack_12;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a856;
  pvStack_c = ExceptionList;
  uVar1 = *(uint *)((int)param_1 + 0x14);
  local_1c = *(int *)((int)param_1 + 0x10);
  local_14 = (undefined2)uVar1;
  uStack_12 = (ushort)(uVar1 >> 0x10);
  local_28 = (short *)0x0;
  ExceptionList = &pvStack_c;
  hModule = (HMODULE)FUN_005fefd0((uint)uStack_12);
  local_20 = hModule;
  hResInfo = FindResourceA(hModule,(LPCSTR)(uVar1 & 0xffff),(LPCSTR)0xa);
  hResData = uStack_24;
  if ((hResInfo != (HRSRC)0x0) &&
     (hResData = LoadResource(hModule,hResInfo), uStack_24 = hResData, hResData != (HGLOBAL)0x0)) {
    local_28 = LockResource(hResData);
  }
  if (local_28 != (short *)0x0) {
    iVar4 = 0;
    psStack_2c = local_28;
    if (0 < *(int *)((int)param_1 + 0x48)) {
      do {
        psStack_2c = psStack_2c + 1;
        pvStack_18 = (void *)FUN_00618b70(0x28);
        pvVar5 = (void *)0x0;
        uStack_4 = 0;
        if (pvStack_18 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          pvVar3 = FUN_005fbd20(pvStack_18,local_20,CONCAT22(extraout_var,*psStack_2c),2);
        }
        uStack_4 = 0xffffffff;
        *(void **)(*(int *)((int)param_1 + 0x4c) + iVar4 * 4) = pvVar3;
        piVar2 = *(int **)(*(int *)(local_1c + 0x20) + iVar4 * 4);
        if (piVar2 == (int *)0x0) {
          pvStack_18 = (void *)FUN_00618b70(0x38);
          uStack_4 = 1;
          if (pvStack_18 != (void *)0x0) {
            pvVar5 = FUN_0041ca70(pvStack_18,
                                  *(undefined4 *)(*(int *)((int)param_1 + 0x4c) + iVar4 * 4));
          }
          uStack_4 = 0xffffffff;
          (**(code **)(**(int **)((int)param_1 + 0xc) + 0x2c))(pvVar5);
          (**(code **)(**(int **)((int)param_1 + 0x10) + 0x10))(iVar4,pvVar5);
        }
        else {
          (**(code **)(*piVar2 + 8))(*(undefined4 *)(*(int *)((int)param_1 + 0x4c) + iVar4 * 4));
        }
        iVar4 = iVar4 + 1;
        hResData = uStack_24;
      } while (iVar4 < *(int *)((int)param_1 + 0x48));
    }
    if (*local_28 != 0) {
      uStack_24 = (HGLOBAL)CONCAT22(uStack_12,*local_28);
      FUN_00403f00(param_1,(ushort *)&uStack_24,0);
    }
    FreeResource(hResData);
  }
  FUN_0041cea0();
  ExceptionList = pvStack_c;
  return;
}

