
int __thiscall FUN_0058a7e0(void *this,int param_1)

{
  bool bVar1;
  int iVar2;
  void *pvVar3;
  uint *puVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar5;
  int *piVar6;
  int local_54;
  uint local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fcc0;
  pvStack_c = ExceptionList;
  iVar5 = 1;
  ExceptionList = &pvStack_c;
  FUN_005947c0(local_2c,param_1,1);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  if (local_10 != 0) {
    do {
      piVar6 = &local_54;
      local_54 = 0;
      iVar2 = FUN_0052bed0((int)local_2c);
      iVar2 = FUN_00588840((void *)((int)this + 8),iVar2,piVar6);
      if ((iVar2 == 0) || (iVar5 == 0)) {
        iVar5 = 0;
      }
      else {
        iVar5 = 1;
      }
      if (local_54 != 0) {
        iVar2 = FUN_0052bed0((int)local_2c);
        if ((*(uint *)(iVar2 + 0x24) >> 6 & 3) !=
            (*(uint *)(*(int *)((int)this + 0x14) + 0x24) >> 6 & 3)) {
          puVar4 = &local_50;
          pvVar3 = (void *)FUN_0052bed0((int)local_2c);
          puVar4 = FUN_004025b0(pvVar3,puVar4);
          local_4._0_1_ = 1;
          bVar1 = FUN_004f44b0((void *)((int)this + 0x68),puVar4,0);
          if ((CONCAT31(extraout_var,bVar1) == 0) || (iVar5 == 0)) {
            iVar5 = 0;
          }
          else {
            iVar5 = 1;
          }
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
        }
      }
      FUN_005130d0((int)local_2c);
    } while (local_10 != 0);
  }
  FUN_0056f3d0(local_4c,param_1,1);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00513120((int)local_4c);
  if (local_30 != 0) {
    do {
      piVar6 = &param_1;
      param_1 = 0;
      iVar2 = FUN_0052bed0((int)local_4c);
      iVar2 = FUN_00588840((void *)((int)this + 8),iVar2,piVar6);
      if ((iVar2 == 0) || (iVar5 == 0)) {
        iVar5 = 0;
      }
      else {
        iVar5 = 1;
      }
      if (param_1 != 0) {
        iVar2 = FUN_0052bed0((int)local_4c);
        if ((*(uint *)(iVar2 + 0x24) >> 6 & 3) !=
            (*(uint *)(*(int *)((int)this + 0x14) + 0x24) >> 6 & 3)) {
          puVar4 = &local_50;
          pvVar3 = (void *)FUN_0052bed0((int)local_4c);
          puVar4 = FUN_004025b0(pvVar3,puVar4);
          local_4._0_1_ = 3;
          bVar1 = FUN_004f44b0((void *)((int)this + 0x60),puVar4,0);
          if ((CONCAT31(extraout_var_00,bVar1) == 0) || (iVar5 == 0)) {
            iVar5 = 0;
          }
          else {
            iVar5 = 1;
          }
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_00619730();
        }
      }
      FUN_005130d0((int)local_4c);
    } while (local_30 != 0);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_0056f4d0(local_4c);
  local_4 = 0xffffffff;
  FUN_00594840(local_2c);
  ExceptionList = pvStack_c;
  return iVar5;
}

