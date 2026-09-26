
undefined4 __cdecl FUN_00517ee0(void *param_1,int param_2,int param_3,int param_4,void *param_5)

{
  bool bVar1;
  bool bVar2;
  int *this;
  uint uVar3;
  int iVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined4 uVar5;
  int local_38;
  int local_34;
  uint local_30;
  undefined4 local_2c;
  uint local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642a20;
  local_c = ExceptionList;
  if ((param_2 == 2) || (param_2 == 1)) {
    local_34 = 0;
    local_38 = 0;
    ExceptionList = &local_c;
    this = (int *)FUN_005054c0();
    while (this != (int *)0x0) {
      if (((this[0x14] & 0x40U) != 0) && ((this[0x14] & 4U) != 0)) {
        uVar3 = (uint)this[9] >> 6 & 3;
        if (uVar3 == 1) {
          iVar4 = (**(code **)(*this + 0x1c0))();
          local_34 = local_34 + iVar4;
        }
        else if (uVar3 == 2) {
          iVar4 = (**(code **)(*this + 0x1c0))();
          local_38 = local_38 + iVar4;
        }
      }
      local_30 = 0x10;
      local_2c = 0x20;
      local_4 = 0;
      this = (int *)FUN_004f6010(this,&local_30,1);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    FUN_00562ae0(local_28);
    local_4 = 1;
    bVar1 = FUN_0055dbf0(local_34,local_38,param_2,param_3,param_4,local_28);
    bVar2 = FUN_00518ac0(param_1,local_28,param_5);
    if ((CONCAT31(extraout_var_00,bVar2) == 0) || (CONCAT31(extraout_var,bVar1) == 0)) {
      uVar5 = 0;
    }
    else {
      uVar5 = 1;
    }
    local_4 = 0xffffffff;
    FUN_00562b30();
  }
  else {
    uVar5 = 0;
  }
  ExceptionList = local_c;
  return uVar5;
}

