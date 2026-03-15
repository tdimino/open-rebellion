
int __thiscall FUN_0051a680(void *this,void *param_1)

{
  int *this_00;
  int iVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  undefined4 uVar5;
  int local_5c;
  int local_58;
  uint local_54;
  void *local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642df8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_50 = this;
  this_00 = (int *)FUN_00505f60((uint *)((int)this + 0x154));
  local_54 = 0;
  local_58 = 0;
  local_5c = 0;
  iVar1 = FUN_0055d610(&local_54,&local_58,&local_5c);
  if ((iVar1 == 0) || (this_00 == (int *)0x0)) {
    iVar1 = 0;
  }
  else {
    iVar1 = 1;
  }
  if (this_00 != (int *)0x0) {
    iVar2 = FUN_0051be90(this_00,local_54,local_58,local_5c,param_1);
    if ((iVar2 == 0) || (iVar1 == 0)) {
      iVar1 = 0;
    }
    else {
      iVar1 = 1;
    }
    iVar2 = 0;
    FUN_00539df0(local_2c,this_00,3,(uint)this_00[9] >> 6 & 3);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      FUN_0052bed0((int)local_2c);
      iVar3 = FUN_0040f340();
      iVar2 = iVar2 + iVar3;
      FUN_005130d0((int)local_2c);
    }
    if (this_00[0x17] < iVar2) {
      iVar2 = FUN_00509cc0(this_00,iVar2,param_1);
      if ((iVar2 == 0) || (iVar1 == 0)) {
        iVar1 = 0;
      }
      else {
        iVar1 = 1;
      }
    }
    iVar2 = 0;
    FUN_0052d690(local_4c,this_00,3,(uint)this_00[9] >> 6 & 3);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00513120((int)local_4c);
    while (local_30 != 0) {
      piVar4 = (int *)FUN_0052bed0((int)local_4c);
      iVar3 = (**(code **)(*piVar4 + 0x1c8))();
      iVar2 = iVar2 + iVar3;
      FUN_005130d0((int)local_4c);
    }
    if (this_00[0x19] < iVar2) {
      iVar2 = FUN_00509dc0(this_00,iVar2,param_1);
      if ((iVar2 == 0) || (iVar1 == 0)) {
        iVar1 = 0;
      }
      else {
        iVar1 = 1;
      }
    }
    param_1 = (void *)0x20000005;
    local_4._0_1_ = 2;
    uVar5 = FUN_0053efa0((uint *)&param_1);
    local_4._0_1_ = 1;
    *(undefined4 *)((int)local_50 + 0xb4) = uVar5;
    FUN_00619730();
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_0052d710(local_4c);
    local_4 = 0xffffffff;
    FUN_00539e70(local_2c);
  }
  ExceptionList = local_c;
  return iVar1;
}

