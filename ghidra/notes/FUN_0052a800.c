
uint __thiscall FUN_0052a800(void *this,void *param_1)

{
  uint *puVar1;
  void *this_00;
  undefined4 *puVar2;
  uint uVar3;
  bool bVar4;
  int local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644978;
  local_c = ExceptionList;
  bVar4 = true;
  ExceptionList = &local_c;
  FUN_0042dbe0(&local_14);
  local_4 = 0;
  puVar1 = FUN_0040d760(this,local_10);
  local_4._0_1_ = 1;
  puVar1 = FUN_004ece40(puVar1);
  local_4._0_1_ = 0;
  FUN_00619730();
  if (puVar1 == (uint *)0x0) {
    FUN_004fcdd0(&local_14);
  }
  else {
    this_00 = (void *)thunk_FUN_0052be30(this);
    bVar4 = this_00 != (void *)0x0;
    uVar3 = 0;
    if (!bVar4) goto LAB_0052a8c3;
    puVar2 = FUN_00402d80(this_00,local_10);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_14,puVar2);
    local_4._0_1_ = 0;
    FUN_00619730();
  }
  uVar3 = 0;
  if (bVar4) {
    uVar3 = FUN_005295a0(this,&local_14,param_1);
  }
LAB_0052a8c3:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar3;
}

