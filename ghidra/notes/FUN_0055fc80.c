
bool FUN_0055fc80(void *param_1,int param_2,undefined4 *param_3,void *param_4)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  uint *puVar6;
  undefined3 extraout_var;
  void *this;
  void *pvVar7;
  bool bVar8;
  uint local_34;
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a0d8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece80(param_3);
  iVar3 = FUN_00506e20();
  bVar8 = iVar3 != 0;
  if ((iVar3 != 0) && (local_30 = FUN_0055ef30(param_2), local_30 != 0)) {
    local_30 = local_30 + -1;
    iVar4 = FUN_0053e290(local_30);
    FUN_0056f450(local_2c,iVar3,4,param_2);
    iVar3 = 0;
    local_4 = 0;
    pvVar7 = (void *)0x0;
    FUN_00513120((int)local_2c);
    while ((local_10 != 0 && (pvVar7 == (void *)0x0))) {
      iVar5 = FUN_0052bed0((int)local_2c);
      local_34 = *(uint *)(iVar5 + 0x50) >> 1 & 1;
      if (local_34 == 0) {
        if (iVar3 == iVar4) {
          pvVar7 = (void *)FUN_0052bed0((int)local_2c);
        }
        iVar3 = iVar3 + 1;
      }
      FUN_005130d0((int)local_2c);
    }
    puVar6 = FUN_004025b0(pvVar7,&local_34);
    local_4._0_1_ = 1;
    FUN_004f26d0(param_3,puVar6);
    local_4._0_1_ = 0;
    FUN_00619730();
    puVar6 = FUN_004025b0(pvVar7,(uint *)&param_3);
    pvVar7 = param_4;
    local_4._0_1_ = 2;
    bVar2 = FUN_0055fe70(puVar6,param_1,param_4);
    if ((CONCAT31(extraout_var,bVar2) == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if (local_30 == 0) {
      if (param_2 == 1) {
        iVar3 = 1;
      }
      else if (param_2 == 2) {
        iVar3 = 2;
      }
      else {
        iVar3 = 0;
      }
      if ((iVar3 == 0) || (bVar2 = true, !bVar8)) {
        bVar2 = false;
      }
      bVar8 = bVar2;
      if (iVar3 != 0) {
        this = (void *)FUN_00506f30(iVar3);
        if ((this == (void *)0x0) || (bVar1 = true, !bVar2)) {
          bVar1 = false;
        }
        bVar8 = bVar1;
        if ((this != (void *)0x0) &&
           ((iVar3 = FUN_0052f590(this,1,pvVar7), iVar3 == 0 || (bVar8 = true, !bVar1)))) {
          bVar8 = false;
        }
      }
    }
    local_4 = 0xffffffff;
    FUN_0056f4d0(local_2c);
  }
  ExceptionList = local_c;
  return bVar8;
}

