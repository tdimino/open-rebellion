
int * __thiscall FUN_004e31b0(void *this,uint *param_1)

{
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  int *piVar4;
  void *pvVar5;
  void *pvVar6;
  uint *puVar7;
  int iVar8;
  uint **ppuVar9;
  int *local_44;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063df56;
  pvStack_c = ExceptionList;
  bVar1 = false;
  bVar2 = false;
  local_44 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  puVar7 = param_1;
  local_4 = 0;
  if ((0x8f < *param_1 >> 0x18) && (*param_1 >> 0x18 < 0x98)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    iVar8 = *(int *)((int)this + 0x2c);
    param_1 = (uint *)&stack0xffffffa4;
    FUN_004f26d0(&stack0xffffffa4,puVar7);
    pvVar3 = FUN_00403d30((void *)(iVar8 + 0x2c));
    piVar4 = FUN_004f3220(*(int *)((int)this + 0x30),puVar7);
    if ((piVar4 != (int *)0x0) && (pvVar3 != (void *)0x0)) {
      if ((*(uint *)((int)pvVar3 + 0x28) & 0x200000) == 0) {
        FUN_0052cab0(local_2c,piVar4,1);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          pvVar5 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 2;
          if (pvVar5 == (void *)0x0) {
            pvVar5 = (void *)0x0;
          }
          else {
            ppuVar9 = &param_1;
            pvVar6 = (void *)FUN_0052bed0((int)local_2c);
            puVar7 = FUN_004025b0(pvVar6,(uint *)ppuVar9);
            bVar2 = true;
            local_4 = CONCAT31(local_4._1_3_,3);
            pvVar5 = FUN_004f5b10(pvVar5,puVar7,0);
          }
          local_4 = 2;
          if (bVar2) {
            bVar2 = false;
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,1);
          if (pvVar5 != (void *)0x0) {
            FUN_004f57b0(local_34,pvVar5);
          }
          FUN_005130d0((int)local_2c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_0052cbb0(local_2c);
      }
      bVar1 = false;
      if ((*(uint *)((int)pvVar3 + 0x28) & 0x400000) == 0) {
        FUN_0052c740(local_2c,piVar4,1);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          pvVar5 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 5;
          if (pvVar5 == (void *)0x0) {
            pvVar5 = (void *)0x0;
          }
          else {
            ppuVar9 = &param_1;
            pvVar6 = (void *)FUN_0052bed0((int)local_2c);
            puVar7 = FUN_004025b0(pvVar6,(uint *)ppuVar9);
            bVar1 = true;
            local_4 = CONCAT31(local_4._1_3_,6);
            pvVar5 = FUN_004f5b10(pvVar5,puVar7,0);
          }
          local_4 = 5;
          if (bVar1) {
            bVar1 = false;
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,4);
          if (pvVar5 != (void *)0x0) {
            FUN_004f57b0(local_34,pvVar5);
          }
          FUN_005130d0((int)local_2c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_0052c840(local_2c);
      }
      bVar1 = false;
      if ((*(uint *)((int)pvVar3 + 0x28) & 0x3800000) == 0) {
        FUN_0052c420(local_2c,piVar4,1);
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          pvVar3 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 8;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            ppuVar9 = &param_1;
            pvVar5 = (void *)FUN_0052bed0((int)local_2c);
            puVar7 = FUN_004025b0(pvVar5,(uint *)ppuVar9);
            bVar1 = true;
            local_4 = CONCAT31(local_4._1_3_,9);
            pvVar3 = FUN_004f5b10(pvVar3,puVar7,0);
          }
          local_4 = 8;
          if (bVar1) {
            bVar1 = false;
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,7);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_34,pvVar3);
          }
          FUN_005130d0((int)local_2c);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_0052c520(local_2c);
      }
    }
  }
  iVar8 = thunk_FUN_005f5060((int)local_34);
  if ((iVar8 != 0) && (local_44 = (int *)FUN_004f5cd0(0x200), local_44 != (int *)0x0)) {
    local_44[8] = *(int *)((int)this + 0x30);
    (**(code **)(*local_44 + 0x24))();
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return local_44;
}

