
int * __fastcall FUN_00474ff0(int param_1)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  void *pvVar4;
  void *pvVar5;
  uint *puVar6;
  int *piVar7;
  void *pvVar8;
  undefined4 *puVar9;
  int iVar10;
  int *local_5c;
  undefined1 local_50 [4];
  undefined1 local_4c [4];
  uint local_48;
  undefined1 local_44 [4];
  undefined1 *local_40;
  undefined4 *local_3c;
  void *local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632c1c;
  pvStack_c = ExceptionList;
  bVar2 = false;
  bVar1 = false;
  local_5c = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  iVar10 = *(int *)(param_1 + 0x4c);
  local_3c = (undefined4 *)&stack0xffffff8c;
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  bVar3 = false;
  FUN_004f26d0(&stack0xffffff8c,(undefined4 *)(param_1 + 0x2c));
  pvVar4 = FUN_00419e40((void *)(iVar10 + 0x78));
  if (((pvVar4 != (void *)0x0) && ((*(uint *)((int)pvVar4 + 0x38) & 0x4000000) != 0)) &&
     ((*(uint *)((int)pvVar4 + 0x38) & 2) == 0)) {
    iVar10 = *(int *)(param_1 + 0x4c);
    local_40 = &stack0xffffff8c;
    local_3c = (undefined4 *)((int)pvVar4 + 0x28);
    FUN_004f26d0(&stack0xffffff8c,local_3c);
    pvVar5 = FUN_00403d30((void *)(iVar10 + 0x2c));
    if ((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x30) & 1) != 0)) {
      for (pvVar4 = (void *)thunk_FUN_005f5060((int)pvVar4 + 0x48); pvVar4 != (void *)0x0;
          pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
        puVar6 = FUN_00403040(pvVar4,local_50);
        local_4._0_1_ = 1;
        pvVar5 = FUN_004f5940((void *)(*(int *)(param_1 + 0x4c) + 0x58),puVar6);
        local_4._0_1_ = 0;
        FUN_00619730();
        puVar6 = FUN_00403040(pvVar4,local_4c);
        local_4._0_1_ = 2;
        piVar7 = FUN_004f2e20(*(int *)(param_1 + 0x20),puVar6);
        local_4._0_1_ = 0;
        FUN_00619730();
        if (((pvVar5 != (void *)0x0) && (piVar7 != (int *)0x0)) &&
           (((*(uint *)((int)pvVar5 + 0x24) & 0x800) == 0 &&
            ((*(uint *)((int)pvVar5 + 0x24) & 0x600000) != 0)))) {
          FUN_00536da0(local_2c,piVar7,1);
          local_4._0_1_ = 3;
          FUN_00513120((int)local_2c);
          while (local_10 != 0) {
            puVar6 = &local_48;
            pvVar5 = (void *)FUN_0052bed0((int)local_2c);
            puVar6 = FUN_004025b0(pvVar5,puVar6);
            local_4._0_1_ = 4;
            pvVar5 = FUN_004f5940((void *)(*(int *)(param_1 + 0x4c) + 0x8c),puVar6);
            local_4._0_1_ = 3;
            FUN_00619730();
            if (pvVar5 != (void *)0x0) {
              if ((*(byte *)((int)pvVar5 + 0x30) & 2) == 0) {
                pvVar8 = (void *)FUN_00618b70(0x20);
                local_4._0_1_ = 7;
                local_38 = pvVar8;
                if (pvVar8 == (void *)0x0) {
                  pvVar5 = (void *)0x0;
                }
                else {
                  puVar9 = FUN_00403040(pvVar5,&local_40);
                  bVar2 = true;
                  local_4 = CONCAT31(local_4._1_3_,8);
                  pvVar5 = FUN_004f5b10(pvVar8,puVar9,0);
                }
                local_4 = 7;
                if (bVar2) {
                  bVar2 = false;
                  FUN_00619730();
                }
                local_4._0_1_ = 3;
                if (pvVar5 != (void *)0x0) {
                  FUN_004f57b0(local_34,pvVar5);
                }
              }
              else {
                pvVar8 = (void *)FUN_00618b70(0x20);
                local_4._0_1_ = 5;
                local_38 = pvVar8;
                if (pvVar8 == (void *)0x0) {
                  pvVar5 = (void *)0x0;
                }
                else {
                  puVar9 = FUN_00403040(pvVar5,local_44);
                  bVar1 = true;
                  local_4 = CONCAT31(local_4._1_3_,6);
                  pvVar5 = FUN_004f5b10(pvVar8,puVar9,0);
                }
                local_4 = 5;
                if (bVar1) {
                  bVar1 = false;
                  FUN_00619730();
                }
                local_4._0_1_ = 3;
                if (pvVar5 != (void *)0x0) {
                  FUN_004f57b0(local_34,pvVar5);
                  bVar3 = true;
                }
              }
            }
            FUN_005130d0((int)local_2c);
          }
          local_4._0_1_ = 0;
          FUN_00536ea0(local_2c);
        }
      }
      iVar10 = thunk_FUN_005f5060((int)local_34);
      if (((iVar10 != 0) && (bVar3)) &&
         (local_5c = (int *)FUN_004f5cd0(0x201), local_5c != (int *)0x0)) {
        local_5c[8] = *(int *)(param_1 + 0x20);
        (**(code **)(*local_5c + 0x24))();
        (**(code **)(*local_5c + 0x2c))(local_40);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return local_5c;
}

