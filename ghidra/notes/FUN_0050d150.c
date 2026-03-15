
uint __thiscall FUN_0050d150(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  uint uVar6;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641e18;
  local_c = ExceptionList;
  uVar6 = 1;
  switch(param_1) {
  case 0:
    break;
  case 1:
    ExceptionList = &local_c;
    FUN_00539df0(local_2c,this,1,param_2);
    local_4 = 0;
    bVar1 = FUN_00559b80((int)local_2c);
    uVar6 = CONCAT31(extraout_var,bVar1);
    if ((uVar6 != 0) && (local_10 != 0)) {
      piVar5 = (int *)FUN_0052bed0((int)local_2c);
      uVar6 = (**(code **)(*piVar5 + 0xac))(8,param_3);
    }
    local_4 = 0xffffffff;
    FUN_00539e70(local_2c);
    break;
  case 2:
    ExceptionList = &local_c;
    FUN_00504cc0(local_2c,this,1,param_2);
    local_4 = 1;
    bVar1 = FUN_00559b80((int)local_2c);
    uVar6 = CONCAT31(extraout_var_00,bVar1);
    if ((uVar6 != 0) && (local_10 != 0)) {
      piVar5 = (int *)FUN_0052bed0((int)local_2c);
      uVar6 = (**(code **)(*piVar5 + 0xac))(8,param_3);
    }
    local_4 = 0xffffffff;
    FUN_00504d40(local_2c);
    break;
  case 3:
    ExceptionList = &local_c;
    FUN_004f2640(local_2c,this,1,param_2);
    local_4 = 2;
    iVar4 = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      iVar2 = FUN_0052bed0((int)local_2c);
      iVar4 = iVar4 + (uint)((*(uint *)(iVar2 + 0xac) & 1) == 0);
      FUN_005130d0((int)local_2c);
    }
    if (iVar4 != 0) {
      iVar2 = FUN_0053e290(iVar4 + -1);
      FUN_00513120((int)local_2c);
      for (iVar4 = 0; (local_10 != 0 && (iVar4 < iVar2)); iVar4 = iVar4 + (uint)((uVar6 & 1) == 0))
      {
        iVar3 = FUN_0052bed0((int)local_2c);
        uVar6 = *(uint *)(iVar3 + 0xac);
        FUN_005130d0((int)local_2c);
      }
      iVar4 = FUN_0052bed0((int)local_2c);
      uVar6 = (uint)(iVar4 != 0);
    }
    if ((uVar6 != 0) && (local_10 != 0)) {
      piVar5 = (int *)FUN_0052bed0((int)local_2c);
      uVar6 = (**(code **)(*piVar5 + 0x2e4))(param_3);
    }
    local_4 = 0xffffffff;
    FUN_004f26c0(local_2c);
    break;
  case 4:
    ExceptionList = &local_c;
    FUN_004f2640(local_2c,this,1,param_2);
    local_4 = 3;
    iVar4 = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      iVar2 = FUN_0052bed0((int)local_2c);
      iVar4 = iVar4 + (uint)((*(uint *)(iVar2 + 0xac) & 1) != 0);
      FUN_005130d0((int)local_2c);
    }
    if (iVar4 != 0) {
      iVar2 = FUN_0053e290(iVar4 + -1);
      FUN_00513120((int)local_2c);
      for (iVar4 = 0; (local_10 != 0 && (iVar4 < iVar2)); iVar4 = iVar4 + (uint)((uVar6 & 1) != 0))
      {
        iVar3 = FUN_0052bed0((int)local_2c);
        uVar6 = *(uint *)(iVar3 + 0xac);
        FUN_005130d0((int)local_2c);
      }
      iVar4 = FUN_0052bed0((int)local_2c);
      uVar6 = (uint)(iVar4 != 0);
    }
    if ((uVar6 != 0) && (local_10 != 0)) {
      piVar5 = (int *)FUN_0052bed0((int)local_2c);
      uVar6 = (**(code **)(*piVar5 + 0x214))(param_3);
    }
    local_4 = 0xffffffff;
    FUN_004f26c0(local_2c);
    break;
  case 5:
    ExceptionList = &local_c;
    FUN_004f2640(local_2c,this,1,param_2);
    local_4 = 4;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      iVar4 = FUN_0052bed0((int)local_2c);
      if ((*(uint *)(iVar4 + 0xac) & 1) != 0) {
        piVar5 = (int *)FUN_0052bed0((int)local_2c);
        uVar6 = (**(code **)(*piVar5 + 0x214))(param_3);
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_004f26c0(local_2c);
    break;
  default:
    uVar6 = 0;
  }
  ExceptionList = local_c;
  return uVar6;
}

