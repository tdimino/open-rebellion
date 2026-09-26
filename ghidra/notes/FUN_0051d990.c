
uint FUN_0051d990(int param_1,uint param_2,void *param_3)

{
  short sVar1;
  void *pvVar2;
  uint *puVar3;
  int *piVar4;
  void *pvVar5;
  int iVar6;
  undefined4 uVar7;
  int iVar8;
  undefined4 *puVar9;
  uint uVar10;
  uint uVar11;
  bool bVar12;
  undefined1 local_2c [4];
  int local_28;
  undefined4 local_24 [2];
  undefined4 local_1c [2];
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006431b0;
  pvStack_c = ExceptionList;
  uVar11 = 1;
  ExceptionList = &pvStack_c;
  FUN_005f50a0(param_3);
  FUN_005205a0(local_24);
  local_4 = 0;
  FUN_005205a0(local_1c);
  local_4 = CONCAT31(local_4._1_3_,1);
  pvVar2 = (void *)thunk_FUN_005f5060(param_2);
  while ((pvVar2 != (void *)0x0 && (bVar12 = uVar11 != 0, uVar11 = 0, bVar12))) {
    puVar3 = FUN_00403040(pvVar2,local_2c);
    local_4._0_1_ = 2;
    piVar4 = (int *)FUN_00504dc0(puVar3);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    uVar11 = 0;
    if (piVar4 != (int *)0x0) {
      FUN_005f50a0(local_24);
      uVar11 = (**(code **)(*piVar4 + 0x3c))(*(undefined4 *)((int)pvVar2 + 0x1c),local_24);
      pvVar5 = (void *)thunk_FUN_005f5060(param_2);
      if (pvVar2 == pvVar5) {
        FUN_00520600(local_1c,local_24);
      }
      else {
        FUN_005f64d0(local_1c,local_24);
      }
    }
    pvVar2 = *(void **)((int)pvVar2 + 0x10);
  }
  for (iVar6 = thunk_FUN_005f5060((int)local_1c); iVar6 != 0; iVar6 = *(int *)(iVar6 + 0x10)) {
    uVar10 = 0;
    if (uVar11 == 0) goto LAB_0051dc74;
    if (*(int *)(iVar6 + 0x1c) == 0) break;
    piVar4 = (int *)FUN_004f5cd0(*(int *)(iVar6 + 0x18));
    uVar11 = (uint)(piVar4 != (int *)0x0);
    if (uVar11 != 0) {
      piVar4[8] = param_1;
      (**(code **)(*piVar4 + 0x24))(param_2);
      uVar7 = FUN_004ece30((undefined4 *)&stack0xffffffd0);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,3);
      (**(code **)(*piVar4 + 0x2c))(uVar7);
      pvStack_c = (void *)CONCAT31(pvStack_c._1_3_,1);
      FUN_00619730();
      iVar8 = (**(code **)(*piVar4 + 0x10))();
      if ((iVar8 != 0) &&
         (puVar9 = FUN_00520690(param_3,*(undefined4 *)(iVar6 + 0x18),1),
         puVar9 != (undefined4 *)0x0)) {
        FUN_00520570(&uStack_14);
        local_4 = CONCAT31(local_4._1_3_,4);
        uVar10 = FUN_0051de80(local_28);
        if ((uVar10 == 0) && (iVar8 = (**(code **)(*piVar4 + 0x18))(&uStack_14), iVar8 != 0)) {
          uVar7 = 1;
        }
        else {
          uVar7 = 0;
        }
        puVar9[7] = uVar7;
        uVar7 = (**(code **)(*piVar4 + 0x14))();
        puVar9[8] = uVar7;
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
      }
    }
    if (piVar4 != (int *)0x0) {
      (**(code **)*piVar4)(1);
    }
  }
  uVar10 = 0;
  if (uVar11 != 0) {
    puVar9 = FUN_00520690(param_3,0x100,1);
    uVar10 = (uint)(puVar9 != (undefined4 *)0x0);
    if (uVar10 != 0) {
      sVar1 = FUN_005f50e0(param_2);
      puVar9[7] = (uint)(sVar1 == 1);
      if (uVar10 != 0) {
        puVar9 = FUN_00520690(param_3,0x103,1);
        uVar11 = param_2;
        uVar10 = (uint)(puVar9 != (undefined4 *)0x0);
        if (uVar10 != 0) {
          puVar9[7] = 0;
          sVar1 = FUN_005f50e0(param_2);
          if (sVar1 == 1) {
            puVar3 = &param_2;
            pvVar2 = (void *)thunk_FUN_005f5060(uVar11);
            FUN_00403040(pvVar2,puVar3);
            local_4._0_1_ = 5;
            uStack_14 = 0x90;
            uStack_10 = 0x98;
            if ((param_2 >> 0x18 < 0x90) || (0x97 < param_2 >> 0x18)) {
              bVar12 = false;
            }
            else {
              bVar12 = true;
            }
            FUN_00619730();
            if (!bVar12) {
              puVar9[7] = 1;
            }
            local_4 = CONCAT31(local_4._1_3_,1);
            FUN_00619730();
          }
        }
      }
    }
  }
LAB_0051dc74:
  local_4 = local_4 & 0xffffff00;
  FUN_005205e0(local_1c);
  local_4 = 0xffffffff;
  FUN_005205e0(local_24);
  ExceptionList = pvStack_c;
  return uVar10;
}

