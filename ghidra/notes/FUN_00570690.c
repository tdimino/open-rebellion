
uint __cdecl FUN_00570690(uint param_1,void *param_2,undefined4 *param_3)

{
  void *this;
  undefined4 *puVar1;
  bool bVar2;
  ushort uVar3;
  short sVar4;
  uint uVar5;
  undefined3 extraout_var;
  undefined2 extraout_var_00;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_2;
  puStack_8 = &LAB_0064c100;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar1 = param_3;
  *param_3 = 0;
  uVar5 = FUN_005868c0(param_1,this,param_3);
  if (*(int *)(param_1 + 4) == 0) {
    ExceptionList = local_c;
    return uVar5;
  }
  if (*(int *)(param_1 + 8) == 0) {
    if (uVar5 != 0) {
      if (*(int *)((int)this + 4) == -1) {
        uVar5 = FUN_00570bc0(*(uint *)(param_1 + 0x18),param_1,this,puVar1);
      }
      if (uVar5 != 0) {
        if (*(int *)((int)this + 4) == -1) {
          uVar5 = FUN_00570bc0(*(uint *)(param_1 + 0x1c),param_1,this,puVar1);
        }
        if ((uVar5 != 0) && (*(int *)((int)this + 4) == -1)) {
          uVar5 = FUN_00570bc0(*(uint *)(param_1 + 0x20),param_1,this,puVar1);
        }
      }
    }
  }
  else if (uVar5 != 0) {
    if (*(int *)((int)this + 4) == -1) {
      uVar5 = FUN_00570940(*(uint *)(param_1 + 0x24),param_1,this,puVar1);
    }
    if (uVar5 != 0) {
      if (*(int *)((int)this + 4) == -1) {
        uVar5 = FUN_00570940(*(uint *)(param_1 + 0x28),param_1,this,puVar1);
      }
      if ((uVar5 != 0) && (*(int *)((int)this + 4) == -1)) {
        uVar5 = FUN_00570940(*(uint *)(param_1 + 0x2c),param_1,this,puVar1);
      }
    }
  }
  if ((uVar5 == 0) || (*(int *)((int)this + 4) != -1)) goto LAB_00570870;
  if (*(int *)(param_1 + 8) == 0) {
    param_2 = (void *)0x0;
    bVar2 = FUN_00570f80(*(int *)(param_1 + 0x10),*(int *)(param_1 + 0x18),(int *)&param_2);
    uVar5 = CONCAT31(extraout_var,bVar2);
    if ((uVar5 == 0) || (param_2 == (void *)0x0)) goto LAB_0057082a;
    bVar2 = true;
  }
  else {
    param_2 = (void *)0x0;
    uVar5 = FUN_00570e40(*(int *)(param_1 + 0x10),*(int *)(param_1 + 0x24),(int *)&param_2);
    if ((uVar5 == 0) || (param_2 == (void *)0x0)) {
LAB_0057082a:
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
  }
  if ((uVar5 != 0) && (!bVar2)) {
    local_14 = 0x40;
    local_10 = 0x90;
    local_4 = 1;
    FUN_00520580(this,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
    *puVar1 = 0xffffffff;
  }
LAB_00570870:
  if (*(int *)(param_1 + 8) == 0) {
    if (uVar5 == 0) {
      ExceptionList = local_c;
      return 0;
    }
    if (*(int *)((int)this + 4) != -1) {
      ExceptionList = local_c;
      return uVar5;
    }
    sVar4 = FUN_004f4290(*(int *)(param_1 + 0x18));
    if (1 < CONCAT22(extraout_var_00,sVar4)) {
      ExceptionList = local_c;
      return uVar5;
    }
    local_14 = 0x40;
    local_10 = 0x94;
    local_4 = 3;
    FUN_00520580(this,&local_14);
  }
  else {
    if (uVar5 == 0) {
      ExceptionList = local_c;
      return 0;
    }
    if (*(int *)((int)this + 4) != -1) {
      ExceptionList = local_c;
      return uVar5;
    }
    uVar3 = FUN_005f50e0(*(int *)(param_1 + 0x24));
    if (1 < uVar3) {
      ExceptionList = local_c;
      return uVar5;
    }
    local_14 = 0x40;
    local_10 = 0x94;
    local_4 = 2;
    FUN_00520580(this,&local_14);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  *puVar1 = 0xffffffff;
  ExceptionList = local_c;
  return uVar5;
}

