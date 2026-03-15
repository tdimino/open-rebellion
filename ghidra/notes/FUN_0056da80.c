
int __thiscall
FUN_0056da80(void *this,void *param_1,undefined4 param_2,undefined4 *param_3,void *param_4)

{
  void *pvVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  void *pvVar4;
  uint *puVar5;
  int *piVar6;
  void *this_00;
  int iVar7;
  void **ppvVar8;
  int local_3c;
  undefined1 auStack_38 [4];
  void *local_34;
  uint local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064bc68;
  local_c = ExceptionList;
  local_3c = 0;
  ExceptionList = &local_c;
  bVar2 = FUN_00521070(this,&local_3c);
  local_34 = (void *)thunk_FUN_00506e80();
  pvVar1 = param_4;
  if ((local_34 == (void *)0x0) || (CONCAT31(extraout_var,bVar2) == 0)) {
    iVar7 = 0;
  }
  else {
    iVar7 = 1;
  }
  if ((local_34 != (void *)0x0) && (local_3c != 0)) {
    iVar3 = FUN_0055f6c0(local_34,1,param_4);
    if ((iVar3 == 0) || (iVar7 == 0)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_004f2640(local_2c,local_3c,1,*(uint *)((int)this + 0x24) >> 6 & 3);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      iVar7 = FUN_0052bed0((int)local_2c);
      local_30 = *(uint *)(iVar7 + 0x78) >> 7 & 1;
      if (local_30 == 0) {
        iVar7 = FUN_0052bed0((int)local_2c);
        local_30 = *(uint *)(iVar7 + 0xac) & 1;
        if (local_30 != 0) {
          ppvVar8 = &param_4;
          *param_3 = 1;
          pvVar4 = (void *)FUN_0052bed0((int)local_2c);
          puVar5 = FUN_004025b0(pvVar4,(uint *)ppvVar8);
                    /* WARNING: Load size is inaccurate */
          local_4._0_1_ = 1;
          iVar7 = (**(code **)(*this + 0x1cc))(puVar5,0,pvVar1);
          if ((iVar7 == 0) || (!bVar2)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          local_4._0_1_ = 0;
          FUN_00619730();
          piVar6 = (int *)FUN_00535760(param_1,auStack_38);
          local_4._0_1_ = 2;
          pvVar4 = pvVar1;
          this_00 = (void *)FUN_0052bed0((int)local_2c);
          iVar7 = FUN_00534390(this_00,piVar6,pvVar4);
          if ((iVar7 == 0) || (!bVar2)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
          iVar7 = FUN_00521880(this,3,pvVar1);
          if ((iVar7 == 0) || (!bVar2)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
        }
      }
      FUN_005130d0((int)local_2c);
    }
    iVar7 = FUN_0055f6c0(local_34,0,pvVar1);
    if ((iVar7 == 0) || (!bVar2)) {
      iVar7 = 0;
    }
    else {
      iVar7 = 1;
    }
    local_4 = 0xffffffff;
    FUN_004f26c0(local_2c);
  }
  ExceptionList = local_c;
  return iVar7;
}

