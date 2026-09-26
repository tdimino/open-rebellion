
bool FUN_0055ff60(int *param_1,void *param_2,int param_3)

{
  bool bVar1;
  int iVar2;
  uint *puVar3;
  void *pvVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  bool bVar5;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_30;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a140;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_004ece60((uint *)(param_1 + 0x27));
  bVar5 = false;
  if (iVar2 == 0) {
    iVar2 = FUN_004ece60((uint *)((int)param_2 + 0x9c));
    bVar5 = iVar2 == 0;
  }
  bVar1 = false;
  if (bVar5) {
    FUN_004fd450(&local_44,param_3);
    bVar5 = true;
    local_4 = 0;
    local_40 = 1;
    local_30 = FUN_004fd2f0();
    puVar3 = FUN_0050c640(param_1);
    local_4._0_1_ = 1;
    if ((*puVar3 >> 0x18 < 0x90) || (0x97 < *puVar3 >> 0x18)) {
      bVar5 = false;
    }
    FUN_00619730();
    local_4._0_1_ = 0;
    FUN_00619730();
    bVar1 = true;
    if (bVar5) {
      puVar3 = FUN_0050c640(param_1);
      local_4._0_1_ = 2;
      pvVar4 = (void *)FUN_00505f60(puVar3);
      local_4._0_1_ = 0;
      FUN_00619730();
      if (pvVar4 != (void *)0x0) {
        bVar5 = FUN_0050d5a0(pvVar4,6,1,(int)&local_44);
        bVar1 = FUN_0050d5a0(pvVar4,6,2,(int)&local_44);
        if ((CONCAT31(extraout_var_00,bVar1) == 0) || (CONCAT31(extraout_var,bVar5) == 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
      }
    }
    FUN_004fd450(&local_28,(int)&local_44);
    local_4._0_1_ = 3;
    local_24 = 1;
    local_20 = 3;
    iVar2 = FUN_004f9510(param_1,1,&local_28);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    iVar2 = FUN_004f9510(param_1,2,&local_28);
    pvVar4 = param_2;
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    iVar2 = FUN_004f9510(param_2,1,&local_28);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    iVar2 = FUN_004f9510(pvVar4,2,&local_28);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    puVar3 = FUN_004025b0(pvVar4,(uint *)&param_2);
    local_4._0_1_ = 4;
    iVar2 = FUN_004ee500(param_1,(int *)puVar3,&local_44);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    local_4._0_1_ = 3;
    FUN_00619730();
    puVar3 = FUN_004025b0(param_1,(uint *)&param_2);
    local_4._0_1_ = 5;
    iVar2 = FUN_004ee500(pvVar4,(int *)puVar3,&local_44);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4._0_1_ = 3;
    FUN_00619730();
    FUN_004ef1c0(param_1,local_30);
    FUN_004ef1c0(pvVar4,local_30);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_004fd4d0(&local_28);
    local_4 = 0xffffffff;
    FUN_004fd4d0(&local_44);
  }
  ExceptionList = local_c;
  return bVar1;
}

