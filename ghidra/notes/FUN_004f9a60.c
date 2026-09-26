
int __thiscall FUN_004f9a60(void *this,int param_1)

{
  uint uVar1;
  int iVar2;
  int *this_00;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  int local_30;
  int local_2c;
  undefined4 local_28 [2];
  uint local_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ff38;
  local_c = ExceptionList;
  local_2c = 1;
  local_30 = 1;
  ExceptionList = &local_c;
  do {
    if (2 < local_30) {
      ExceptionList = local_c;
      return local_2c;
    }
    this_00 = (int *)FUN_00539fd0(this,local_30);
    uVar3 = (uint)this_00[9] >> 4 & 3;
    uVar4 = this_00[9] & 0xf;
    if (uVar3 == 1) {
      uVar3 = 1;
    }
    else if (uVar3 == 2) {
      uVar3 = 2;
    }
    else {
      uVar3 = 0;
    }
    uVar1 = *(uint *)((int)this + 0x50);
    if (uVar3 == 1) {
      iVar2 = 7;
    }
    else {
      iVar2 = 8;
    }
    uVar5 = 2;
    if (((*(uint *)((int)this + 0x24) >> 6 & 3) == uVar3) ||
       ((((uVar1 >> iVar2 & 1) != 0 && ((uVar1 & 4) != 0)) && ((uVar1 & 0x10) == 0)))) {
      uVar5 = 1;
    }
    if (((uVar5 != uVar4) && (FUN_00540630(this_00,uVar5), uVar5 == 1)) && (uVar4 == 2)) {
      FUN_004fd450(local_28,param_1);
      local_4 = 0;
      local_20 = uVar5;
      local_2c = (**(code **)(*this_00 + 0x24))(local_28);
      FUN_004f8340(this_00,local_28);
      local_4 = 0xffffffff;
      FUN_004fd4d0(local_28);
    }
    local_30 = local_30 + 1;
  } while (local_2c != 0);
  ExceptionList = local_c;
  return 0;
}

