
undefined4 __thiscall FUN_00528890(void *this,void *param_1)

{
  uint uVar1;
  uint uVar2;
  void *this_00;
  uint *puVar3;
  uint *puVar4;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644658;
  local_c = ExceptionList;
  if (((*(uint *)((int)param_1 + 0x50) & 2) != 0) && ((*(uint *)((int)param_1 + 0x50) & 0x1c) == 0))
  {
    ExceptionList = &local_c;
    local_10 = this;
    puVar3 = FUN_004025b0(this,(uint *)&local_10);
    local_4 = 0;
    puVar4 = FUN_0048a640(this_00,&param_1);
    uVar1 = *puVar4;
    uVar2 = *puVar3;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (uVar1 == uVar2) {
      ExceptionList = local_c;
      return 1;
    }
  }
  ExceptionList = local_c;
  return 0;
}

