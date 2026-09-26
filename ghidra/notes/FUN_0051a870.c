
bool FUN_0051a870(void *param_1)

{
  bool bVar1;
  int *this;
  int iVar2;
  int iVar3;
  int *piVar4;
  bool bVar5;
  uint local_5c;
  int local_58;
  int local_54;
  uint local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00642e28;
  local_c = ExceptionList;
  local_5c = 0x90000109;
  bVar1 = false;
  local_4 = 0;
  ExceptionList = &local_c;
  this = (int *)FUN_00505f60(&local_5c);
  local_4 = 0xffffffff;
  FUN_00619730();
  bVar5 = this != (int *)0x0;
  if (this != (int *)0x0) {
    local_50 = 0;
    local_54 = 0;
    local_58 = 0;
    iVar2 = FUN_0055d640(&local_50,&local_54,&local_58);
    if ((iVar2 != 0) && (bVar5)) {
      bVar1 = true;
    }
    iVar2 = FUN_0051be90(this,local_50,local_54,local_58,param_1);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    iVar2 = 0;
    FUN_00539df0(local_2c,this,3,(uint)this[9] >> 6 & 3);
    local_4 = 1;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      FUN_0052bed0((int)local_2c);
      iVar3 = FUN_0040f340();
      iVar2 = iVar2 + iVar3;
      FUN_005130d0((int)local_2c);
    }
    if (this[0x17] < iVar2) {
      iVar2 = FUN_00509cc0(this,iVar2,param_1);
      if ((iVar2 == 0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
    }
    iVar2 = 0;
    FUN_0052d690(local_4c,this,3,(uint)this[9] >> 6 & 3);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00513120((int)local_4c);
    while (local_30 != 0) {
      piVar4 = (int *)FUN_0052bed0((int)local_4c);
      iVar3 = (**(code **)(*piVar4 + 0x1c8))();
      iVar2 = iVar2 + iVar3;
      FUN_005130d0((int)local_4c);
    }
    if (this[0x19] < iVar2) {
      iVar2 = FUN_00509dc0(this,iVar2,param_1);
      if ((iVar2 == 0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_0052d710(local_4c);
    local_4 = 0xffffffff;
    FUN_00539e70(local_2c);
  }
  ExceptionList = local_c;
  return bVar5;
}

