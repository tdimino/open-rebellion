
bool __thiscall
FUN_00542b60(void *this,int *param_1,undefined4 param_2,undefined4 param_3,uint *param_4,
            undefined4 param_5,int param_6,int param_7,void *param_8)

{
  void *this_00;
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int *piVar3;
  void *pvVar4;
  bool bVar5;
  int local_48 [2];
  int local_40 [2];
  int local_38 [2];
  undefined4 local_30;
  undefined4 local_2c;
  int local_28 [2];
  uint local_20 [4];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_8;
  puStack_8 = &LAB_00647188;
  pvStack_c = ExceptionList;
  local_30 = 1;
  local_2c = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_8,&local_30);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (param_1[3] == 0x240) {
    local_30 = 0x50;
    local_2c = 0x80;
    if ((*param_4 >> 0x18 < 0x50) || (0x7f < *param_4 >> 0x18)) {
      bVar5 = false;
      FUN_00619730();
    }
    else {
      bVar5 = true;
      FUN_00619730();
    }
  }
  else if (param_1[3] == 0x241) {
    local_30 = 0x40;
    local_2c = 0x50;
    if ((*param_4 >> 0x18 < 0x40) || (0x4f < *param_4 >> 0x18)) {
      bVar5 = false;
      FUN_00619730();
    }
    else {
      bVar5 = true;
      FUN_00619730();
    }
  }
  else {
    bVar5 = false;
  }
  if (!bVar5) {
    local_30 = 1;
    local_2c = 1;
    local_4 = 1;
    FUN_00520580(this_00,&local_30);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  FUN_004f4340(local_38);
  local_4 = 2;
  FUN_004f4340(local_40);
  local_4._0_1_ = 3;
  FUN_004f4340(local_48);
  local_4._0_1_ = 4;
  FUN_004f4340(local_28);
  local_4._0_1_ = 5;
  if (*(int *)((int)this_00 + 4) == -1) {
    bVar1 = FUN_0054bb90(param_1,param_6,param_7,local_38,local_40,local_48,local_28,this_00);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
  }
  FUN_004ece30(&param_8);
  local_4._0_1_ = 6;
  if (*(int *)((int)this_00 + 4) == -1) {
    iVar2 = FUN_0054bf00((int)local_28,&param_8,this_00);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    if (*(int *)((int)this_00 + 4) == -1) {
      iVar2 = FUN_0054c200(param_1,(int)local_38,(int)local_40,(int)local_48,this_00);
      if ((iVar2 == 0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
    }
  }
  FUN_00568fe0(local_20);
  local_4._0_1_ = 7;
  if (*(int *)((int)this_00 + 4) == -1) {
    iVar2 = FUN_005830a0((int)local_38,(int)local_40,(int)local_48,local_20,this_00);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
  }
  if (*(int *)((int)this_00 + 4) == -1) {
    if (local_10 == 1) {
      iVar2 = 1;
    }
    else if (local_10 == 2) {
      iVar2 = 2;
    }
    else {
      iVar2 = 0;
    }
    piVar3 = FUN_004f2d10(iVar2,(uint *)(param_1 + 2));
    if (piVar3 == (int *)0x0) {
      local_30 = 0x40;
      local_2c = 0x21;
      local_4._0_1_ = 8;
      FUN_00520580(this_00,&local_30);
      local_4._0_1_ = 7;
      FUN_00619730();
    }
    if (*(int *)((int)this_00 + 4) == -1) {
      pvVar4 = (void *)FUN_0051cab0(param_4);
      bVar5 = pvVar4 != (void *)0x0;
      if (pvVar4 != (void *)0x0) {
        iVar2 = FUN_0054c590(this,local_10,local_38,local_40,local_48,local_20,piVar3,pvVar4,this_00
                            );
        if ((iVar2 == 0) || (!bVar5)) {
          bVar5 = false;
        }
        else {
          bVar5 = true;
        }
      }
    }
  }
  local_4._0_1_ = 6;
  FUN_00619730();
  local_4._0_1_ = 5;
  FUN_00619730();
  local_4._0_1_ = 4;
  FUN_004f4380(local_28);
  local_4._0_1_ = 3;
  FUN_004f4380(local_48);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_004f4380(local_40);
  local_4 = 0xffffffff;
  FUN_004f4380(local_38);
  ExceptionList = pvStack_c;
  return bVar5;
}

