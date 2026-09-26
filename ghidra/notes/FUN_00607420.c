
undefined4 __fastcall FUN_00607420(int param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  HDC hDC;
  void *pvVar6;
  int iVar7;
  int iVar8;
  int local_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006565fb;
  pvStack_c = ExceptionList;
  local_1c = 0;
  ExceptionList = &pvStack_c;
  if (*(int **)(param_1 + 0xa0) != (int *)0x0) {
    ExceptionList = &pvStack_c;
    iVar2 = FUN_005fc0e0(*(int **)(param_1 + 0xa0));
    iVar3 = FUN_005fc0f0(*(int **)(param_1 + 0xa0));
    iVar4 = *(int *)(param_1 + 0x34) / iVar3;
    iVar5 = *(int *)(param_1 + 0x30) / iVar2;
    if (iVar4 != 1) {
      iVar4 = iVar4 + 1;
    }
    if (iVar5 != 1) {
      iVar5 = iVar5 + 1;
    }
    puVar1 = *(undefined4 **)(param_1 + 0x104);
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
      *(undefined4 *)(param_1 + 0x104) = 0;
    }
    hDC = GetDC(*(HWND *)(param_1 + 0x18));
    pvVar6 = (void *)FUN_00618b70(0x28);
    uStack_4 = 0;
    if (pvVar6 == (void *)0x0) {
      pvVar6 = (void *)0x0;
    }
    else {
      pvVar6 = FUN_005fbea0(pvVar6,iVar5 * iVar2,iVar4 * iVar3,*(uint **)(param_1 + 0xa0),hDC);
    }
    uStack_4 = 0xffffffff;
    *(void **)(param_1 + 0x104) = pvVar6;
    if (pvVar6 != (void *)0x0) {
      if (0 < iVar4) {
        iVar7 = 0;
        local_1c = iVar4;
        do {
          if (0 < iVar5) {
            iVar8 = 0;
            iVar4 = iVar5;
            do {
              FUN_005fcc30(*(void **)(param_1 + 0xa0),*(uint **)(param_1 + 0x104),iVar8,iVar7,0,0,
                           (undefined4 *)0x0,0);
              iVar8 = iVar8 + iVar2;
              iVar4 = iVar4 + -1;
            } while (iVar4 != 0);
          }
          iVar7 = iVar7 + iVar3;
          local_1c = local_1c + -1;
        } while (local_1c != 0);
      }
      local_1c = 1;
      *(undefined4 *)(param_1 + 0xa4) = *(undefined4 *)(param_1 + 0x104);
    }
    ReleaseDC(*(HWND *)(param_1 + 0x18),hDC);
  }
  FUN_006076e0(param_1);
  ExceptionList = pvStack_c;
  return local_1c;
}

