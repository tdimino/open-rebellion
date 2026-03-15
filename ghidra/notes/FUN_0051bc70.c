
bool FUN_0051bc70(uint *param_1,undefined4 param_2,undefined4 param_3,int param_4,void *param_5)

{
  int iVar1;
  int iVar2;
  uint *puVar3;
  int *piVar4;
  int *this;
  bool bVar5;
  uint local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642fe0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_0051cab0(param_1);
  this = (int *)0x0;
  bVar5 = iVar2 != 0;
  puVar3 = param_1;
  if (iVar2 != 0) {
    local_14 = local_14 & 0xffffff | 0x80000000;
    local_14 = local_14 ^ (*(uint *)(iVar2 + 0x40) ^ local_14) & 0xffffff;
    local_4 = 0;
    puVar3 = (uint *)FUN_00506a40(&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((puVar3 == (uint *)0x0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
  }
  if ((puVar3 != (uint *)0x0) && (iVar2 != 0)) {
    this = FUN_004f7d50(puVar3,param_1,param_3,param_5,1,1);
    if ((this == (int *)0x0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
  }
  if (this != (int *)0x0) {
    iVar2 = FUN_0050cf70(this,param_4);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    iVar2 = (**(code **)(*this + 0xbc))(param_5);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    iVar2 = FUN_0050cfa0(this,1);
    if ((iVar2 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    piVar4 = (int *)FUN_00402d80(this,&param_1);
    iVar2 = *piVar4;
    iVar1 = *(int *)(local_10 + 0x150);
    FUN_00619730();
    if (iVar2 == iVar1) {
      puVar3 = FUN_004025b0(this,(uint *)&param_1);
      local_4 = 1;
      FUN_004f26d0((void *)(local_10 + 0x154),puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return bVar5;
}

